pub mod resources;
mod systems;

use bevy_app::{App, Plugin, Update};
use bevy_ecs::schedule::{IntoSystemConfigs, IntoSystemSetConfigs, SystemSet};
use bevy_transform::TransformPlugin;
use dyn_comp_asset::CompAssetPlugin;
use dyn_comp_bundles::events::{
    CompositionResizedInputEvent, CompositionViewportChangedInputEvent, EntityDeletedInputEvent,
    EntityMovedInputEvent, EntitySetPositionInputEvent, EntitySetRotationInputEvent,
};
use resources::composition::CompositionRes;
use systems::{
    events::{
        handle_entity_deleted_event, handle_entity_moved_event, handle_entity_set_position_event,
        handle_entity_set_rotation_event,
    },
    outline::{
        ellipse::outline_ellipse, polygon::outline_polygon, rectangle::outline_rectangle,
        star::outline_star,
    },
    stroke::stroke_path_system,
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

    /// After this label, the system has applied layout calculations to the composition's nodes.
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
        app.add_event::<EntityDeletedInputEvent>();
        app.add_event::<EntityMovedInputEvent>();
        app.add_event::<EntitySetPositionInputEvent>();
        app.add_event::<EntityDeletedInputEvent>();
        app.add_event::<EntitySetRotationInputEvent>();

        // Register resources
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
                CompCoreSystemSet::Layout,
                CompCoreSystemSet::Prepare,
                CompCoreSystemSet::Outline,
                CompCoreSystemSet::PostOutline,
            )
                .chain(),
        );

        // Register systems
        app.add_systems(
            Update,
            (
                handle_entity_deleted_event.in_set(CompCoreSystemSet::InputEvents),
                handle_entity_moved_event.in_set(CompCoreSystemSet::InputEvents),
                handle_entity_set_position_event.in_set(CompCoreSystemSet::InputEvents),
                handle_entity_set_rotation_event.in_set(CompCoreSystemSet::InputEvents),
                outline_rectangle.in_set(CompCoreSystemSet::Outline),
                outline_ellipse.in_set(CompCoreSystemSet::Outline),
                outline_star.in_set(CompCoreSystemSet::Outline),
                outline_polygon.in_set(CompCoreSystemSet::Outline),
                stroke_path_system.in_set(CompCoreSystemSet::PostOutline),
            ),
        );

        #[cfg(feature = "dtif")]
        inject_dtif_into_ecs(&mut app.world, &self.dtif)
    }
}

#[cfg(feature = "dtif")]
fn inject_dtif_into_ecs(world: &mut bevy_ecs::world::World, dtif: &dyn_comp_dtif::DtifComposition) {
    use dyn_comp_asset::resources::AssetDatabaseRes;
    use dyn_comp_bundles::viewport::Viewport;
    use glam::Vec2;

    let mut dtif_injector = dyn_comp_dtif::dtif_injector::DtifInjector::new();

    // Load assets
    if let Some(mut asset_db) = world.get_resource_mut::<AssetDatabaseRes>() {
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
