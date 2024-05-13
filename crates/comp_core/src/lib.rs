pub mod resources;
mod systems;

use bevy_app::{App, First, Last, Plugin, Update};
use bevy_ecs::schedule::{IntoSystemConfigs, IntoSystemSetConfigs, SystemSet};
use bevy_transform::TransformPlugin;
use dyn_comp_asset::CompAssetPlugin;
use dyn_comp_bundles::events::{
    DeleteEntityInputEvent, FocusRootNodesInputEvent, MoveEntityInputEvent,
    UpdateCompositionSizeInputEvent, UpdateCompositionViewportInputEvent,
    UpdateEntityPositionInputEvent, UpdateEntityRotationInputEvent, UpdateEntityTextInputEvent,
};
use resources::{composition::CompositionRes, layout::LayoutRes, tick::TickRes};
use systems::{
    events::{
        delete_entity_input_system, despawn_removed_entities_system, focus_root_nodes_input_system,
        move_entity_input_system, update_composition_size_input_system,
        update_composition_viewport_input_system, update_entity_position_input_system,
        update_entity_rotation_input_system, update_entity_text_input_system,
    },
    hierarchy::update_hierarchy_levels,
    layout::{
        absolute_layout::{apply_pre_absolute_layout_properties, update_absolute_layout},
        static_layout::{
            discover_new_static_layout_parent_nodes, mark_nodes_with_static_layout_change_as_stale,
            remove_stale_layout_nodes, update_static_layout,
            update_static_layout_parent_nodes_children,
        },
    },
    outline::{
        ellipse::outline_ellipse, polygon::outline_polygon, rectangle::outline_rectangle,
        star::outline_star, text::outline_text,
    },
    stroke::stroke_path_system,
    text::{compute_text_from_scratch, compute_text_on_node_change, compute_text_on_size_change},
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
        app.add_event::<UpdateCompositionSizeInputEvent>();
        app.add_event::<UpdateCompositionViewportInputEvent>();
        app.add_event::<FocusRootNodesInputEvent>();
        app.add_event::<DeleteEntityInputEvent>();
        app.add_event::<MoveEntityInputEvent>();
        app.add_event::<UpdateEntityPositionInputEvent>();
        app.add_event::<UpdateEntityRotationInputEvent>();
        app.add_event::<UpdateEntityTextInputEvent>();

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
                update_composition_size_input_system.in_set(CompCoreSystemSet::InputEvents),
                update_composition_viewport_input_system.in_set(CompCoreSystemSet::InputEvents),
                focus_root_nodes_input_system
                    .in_set(CompCoreSystemSet::InputEvents)
                    .after(update_composition_size_input_system),
                delete_entity_input_system.in_set(CompCoreSystemSet::InputEvents),
                move_entity_input_system.in_set(CompCoreSystemSet::InputEvents),
                update_entity_position_input_system.in_set(CompCoreSystemSet::InputEvents),
                update_entity_rotation_input_system.in_set(CompCoreSystemSet::InputEvents),
                update_entity_text_input_system.in_set(CompCoreSystemSet::InputEvents),
            ),
        );
        app.add_systems(
            Update,
            (
                compute_text_from_scratch.in_set(CompCoreSystemSet::Compute),
                compute_text_on_size_change.in_set(CompCoreSystemSet::Compute),
                compute_text_on_node_change.in_set(CompCoreSystemSet::Compute),
            ),
        );
        app.add_systems(
            Update,
            (
                discover_new_static_layout_parent_nodes.in_set(CompCoreSystemSet::PreLayout),
                update_static_layout_parent_nodes_children.in_set(CompCoreSystemSet::PreLayout),
                mark_nodes_with_static_layout_change_as_stale.in_set(CompCoreSystemSet::PreLayout),
                apply_pre_absolute_layout_properties.in_set(CompCoreSystemSet::PreLayout),
                remove_stale_layout_nodes.in_set(CompCoreSystemSet::PreLayout),
                update_absolute_layout.in_set(CompCoreSystemSet::Layout),
                update_static_layout
                    .in_set(CompCoreSystemSet::Layout)
                    .after(update_absolute_layout),
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
                outline_text.in_set(CompCoreSystemSet::Outline),
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
        } else {
            panic!("Failed to get DTIF from DTIF-Handler!");
        }
    } else {
        panic!("Failed to insert root node into world!");
    }
}
