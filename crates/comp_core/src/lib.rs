pub mod resources;
mod systems;

use bevy_app::{App, First, Last, Plugin, Update};
use bevy_ecs::schedule::{IntoSystemConfigs, IntoSystemSetConfigs, SystemSet};
use bevy_transform::TransformPlugin;
use dyn_comp_asset::CompAssetPlugin;
use dyn_comp_bundles::events::{
    CompositionResizedInputEvent, CompositionViewportChangedInputEvent, EntityDeletedInputEvent,
    EntityMovedInputEvent, EntitySetPositionInputEvent, EntitySetRotationInputEvent,
    FocusRootNodesInputEvent,
};
use resources::{composition::CompositionRes, layout::LayoutRes, tick::TickRes};
use systems::{
    events::{
        composition_resized_input_system, composition_viewport_input_system,
        despawn_removed_entities_system, entity_deleted_input_system, entity_moved_input_system,
        entity_set_position_input_system, entity_set_rotation_input_system,
        focus_root_nodes_input_system,
    },
    hierarchy::update_hierarchy_levels,
    layout::{
        add_new_layout_parents_to_layout_tree, mark_nodes_with_layout_change_as_stale,
        update_layout, update_layout_parent_children,
    },
    outline::{
        ellipse::outline_ellipse,
        polygon::outline_polygon,
        rectangle::outline_rectangle,
        star::outline_star,
        text::{outline_text_from_scratch, outline_text_on_size_change},
    },
    stroke::stroke_path_system,
    tick::collect_first_tick,
    vector::resize_vector_node,
};

pub struct CompCorePlugin {
    #[cfg(not(feature = "dtif"))]
    pub size: Size,
    #[cfg(not(feature = "dtif"))]
    pub viewport: Option<Viewport>,
    #[cfg(not(feature = "dtif"))]
    pub root_nodes: Vec<Entity>,
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum CompCoreSystemSet {
    /// After this label, the system has processed input events.
    InputEvents,

    PreCompute,
    Compute,

    /// After this label, the system has applied layout calculations to the composition's nodes.
    PreLayout,
    Layout,
    PostLayout,

    // After this label, the system has prepared the nodes for visual outlining.
    Prepare,

    /// After this label, the system has outlined the composition nodes.
    Outline,

    /// After this label, the system has made modifications based on the outlined composition nodes.
    PostOutline,
}

impl Plugin for CompCorePlugin {
    fn build(&self, app: &mut App) {
        // Register plugins
        app.add_plugins(CompAssetPlugin);
        app.add_plugins(TransformPlugin);

        // Register events
        app.add_event::<CompositionResizedInputEvent>();
        app.add_event::<CompositionViewportChangedInputEvent>();
        app.add_event::<FocusRootNodesInputEvent>();
        app.add_event::<EntityDeletedInputEvent>();
        app.add_event::<EntityMovedInputEvent>();
        app.add_event::<EntitySetPositionInputEvent>();
        app.add_event::<EntityDeletedInputEvent>();
        app.add_event::<EntitySetRotationInputEvent>();

        // Register resources
        app.init_resource::<LayoutRes>();
        app.init_resource::<TickRes>();
        #[cfg(not(feature = "dtif"))]
        app.insert_resource(CompositionRes {
            root_nodes: self.root_nodes.clone(),
            viewport: self.viewport.unwrap_or_default(),
            size: self.size,
        });

        // Register system sets
        app.configure_sets(
            Update,
            (
                CompCoreSystemSet::InputEvents,
                CompCoreSystemSet::PreCompute,
                CompCoreSystemSet::Compute,
                CompCoreSystemSet::PreLayout,
                CompCoreSystemSet::Layout,
                CompCoreSystemSet::Prepare,
                CompCoreSystemSet::Outline,
                CompCoreSystemSet::PostOutline,
            )
                .chain(),
        );

        // Register systems
        app.add_systems(
            First,
            (
                collect_first_tick,
                update_hierarchy_levels.after(collect_first_tick),
            ),
        );
        app.add_systems(
            Update,
            (
                composition_resized_input_system.in_set(CompCoreSystemSet::InputEvents),
                composition_viewport_input_system.in_set(CompCoreSystemSet::InputEvents),
                focus_root_nodes_input_system
                    .in_set(CompCoreSystemSet::InputEvents)
                    .after(composition_resized_input_system),
                entity_deleted_input_system.in_set(CompCoreSystemSet::InputEvents),
                entity_moved_input_system.in_set(CompCoreSystemSet::InputEvents),
                entity_set_position_input_system.in_set(CompCoreSystemSet::InputEvents),
                entity_set_rotation_input_system.in_set(CompCoreSystemSet::InputEvents),
            ),
        );
        app.add_systems(
            Update,
            (
                add_new_layout_parents_to_layout_tree.in_set(CompCoreSystemSet::PreLayout),
                update_layout_parent_children.in_set(CompCoreSystemSet::PreLayout),
                mark_nodes_with_layout_change_as_stale.in_set(CompCoreSystemSet::PreLayout),
                update_layout.in_set(CompCoreSystemSet::Layout),
            ),
        );
        app.add_systems(
            Update,
            (
                resize_vector_node.in_set(CompCoreSystemSet::Outline),
                outline_rectangle.in_set(CompCoreSystemSet::Outline),
                outline_ellipse.in_set(CompCoreSystemSet::Outline),
                outline_star.in_set(CompCoreSystemSet::Outline),
                outline_polygon.in_set(CompCoreSystemSet::Outline),
                outline_text_from_scratch.in_set(CompCoreSystemSet::Outline),
                outline_text_on_size_change.in_set(CompCoreSystemSet::Outline),
                stroke_path_system.in_set(CompCoreSystemSet::PostOutline),
            ),
        );
        app.add_systems(Last, despawn_removed_entities_system);
    }
}

#[cfg(feature = "dtif")]
pub fn insert_dtif_into_world(
    world: &mut bevy_ecs::world::World,
    dtif_handler: &mut dyn_comp_dtif::dtif_handler::DtifHandler,
) {
    use dyn_comp_asset::resources::AssetsRes;
    use dyn_comp_bundles::properties::Viewport;
    use glam::Vec2;

    // Load assets
    if let Some(mut asset_db) = world.get_resource_mut::<AssetsRes>() {
        dtif_handler.load_assets(asset_db.as_mut());
    }

    // Spawn nodes recursively
    let maybe_root_node_entity = dtif_handler.insert_into_world(world);
    if let Some(root_node_entity) = maybe_root_node_entity {
        if let Some(dtif) = dtif_handler.get_dtif() {
            world.insert_resource(CompositionRes {
                root_nodes: vec![root_node_entity],
                viewport: dtif.viewport.unwrap_or(Viewport {
                    physical_position: Vec2::default(),
                    physical_size: dtif.size,
                }),
                size: dtif.size,
            });
        }
    }
}
