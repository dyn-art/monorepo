pub mod resources;
mod systems;

use bevy_app::{App, First, Last, Plugin, Update};
use bevy_ecs::{
    component::Tick,
    schedule::{IntoSystemConfigs, IntoSystemSetConfigs, SystemSet},
};
use bevy_transform::TransformPlugin;
use dyn_comp_asset::CompAssetPlugin;
use dyn_comp_bundles::events::{
    CompositionResizedInputEvent, CompositionViewportChangedInputEvent, EntityDeletedInputEvent,
    EntityMovedInputEvent, EntitySetPositionInputEvent, EntitySetRotationInputEvent,
    FocusRootNodesInputEvent,
};
use resources::{composition::CompositionRes, tick::TickRes};
use systems::{
    constraints::{apply_constraints, apply_constraints_offset},
    events::{
        composition_resized_input_system, composition_viewport_input_system,
        despawn_removed_entities_system, entity_deleted_input_system, entity_moved_input_system,
        entity_set_position_input_system, entity_set_rotation_input_system,
        focus_root_nodes_input_system,
    },
    group::{
        compute_group_size, compute_group_transform, mark_group_size_as_stale,
        mark_group_transform_as_stale,
    },
    hierarchy::update_hierarchy_levels,
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
    #[cfg(feature = "dtif")]
    pub dtif: dyn_comp_dtif::DtifComposition,
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
        app.add_event::<CompositionResizedInputEvent>();
        app.add_event::<CompositionViewportChangedInputEvent>();
        app.add_event::<FocusRootNodesInputEvent>();
        app.add_event::<EntityDeletedInputEvent>();
        app.add_event::<EntityMovedInputEvent>();
        app.add_event::<EntitySetPositionInputEvent>();
        app.add_event::<EntityDeletedInputEvent>();
        app.add_event::<EntitySetRotationInputEvent>();

        // Register resources
        app.insert_resource(TickRes {
            first_in_cycle: Tick::new(0),
        });
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
                focus_root_nodes_input_system.in_set(CompCoreSystemSet::InputEvents),
                entity_deleted_input_system.in_set(CompCoreSystemSet::InputEvents),
                entity_moved_input_system.in_set(CompCoreSystemSet::InputEvents),
                entity_set_position_input_system.in_set(CompCoreSystemSet::InputEvents),
                entity_set_rotation_input_system.in_set(CompCoreSystemSet::InputEvents),
            ),
        );
        app.add_systems(
            Update,
            (
                mark_group_size_as_stale.in_set(CompCoreSystemSet::PreCompute),
                mark_group_transform_as_stale.in_set(CompCoreSystemSet::PreCompute),
                compute_group_size.in_set(CompCoreSystemSet::Compute),
                compute_group_transform.in_set(CompCoreSystemSet::Compute),
                apply_constraints_offset.in_set(CompCoreSystemSet::PreLayout),
                apply_constraints.in_set(CompCoreSystemSet::Layout),
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

        #[cfg(feature = "dtif")]
        inject_dtif_into_ecs(&mut app.world, &self.dtif)
    }
}

#[cfg(feature = "dtif")]
fn inject_dtif_into_ecs(world: &mut bevy_ecs::world::World, dtif: &dyn_comp_dtif::DtifComposition) {
    use dyn_comp_asset::resources::AssetsRes;
    use dyn_comp_bundles::properties::Viewport;
    use glam::Vec2;

    let mut dtif_injector = dyn_comp_dtif::dtif_injector::DtifInjector::new();

    // Load assets
    if let Some(mut asset_db) = world.get_resource_mut::<AssetsRes>() {
        dtif_injector.load_assets(dtif, asset_db.as_mut());
    }

    // Spawn nodes recursively
    let maybe_root_node_entity = dtif_injector.inject_from_root(dtif, world);
    if let Some(root_node_entity) = maybe_root_node_entity {
        world.insert_resource(CompositionRes {
            root_nodes: vec![root_node_entity],
            viewport: dtif.viewport.unwrap_or(Viewport {
                physical_position: Vec2::default(),
                physical_size: dtif.size,
            }),
            size: dtif.size,
        })
    }
}
