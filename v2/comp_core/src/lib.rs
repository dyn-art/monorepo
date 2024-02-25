mod resources;
mod systems;

use bevy_app::{App, Plugin, Update};
use bevy_ecs::schedule::{IntoSystemConfigs, IntoSystemSetConfigs, SystemSet};
use dyn_comp_types::events::{
    CompositionResizedInputEvent, CompositionViewportChangedInputEvent, EntityDeletedInputEvent,
    EntityMovedInputEvent, EntitySetPositionInputEvent,
};
use resources::composition::CompositionRes;
use systems::outline::rectangle::outline_rectangle;

pub struct CompCorePlugin {
    #[cfg(feature = "dtif")]
    pub dtif: dyn_comp_dtif::CompDtif,
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
}

impl Plugin for CompCorePlugin {
    fn build(&self, app: &mut App) {
        // Register events
        app.add_event::<CompositionResizedInputEvent>();
        app.add_event::<CompositionViewportChangedInputEvent>();
        app.add_event::<EntityDeletedInputEvent>();
        app.add_event::<EntityMovedInputEvent>();
        app.add_event::<EntitySetPositionInputEvent>();

        // Register resources
        #[cfg(not(feature = "dtif"))]
        app.insert_resource(CompositionRes {
            root_nodes: self.root_nodes.clone(),
            viewport: self.viewport.unwrap_or_default(),
            size: self.size,
        });
        // TODO
        // - Font Cache
        // - Asset Cache

        // Register system sets
        app.configure_sets(
            Update,
            (
                CompCoreSystemSet::InputEvents,
                CompCoreSystemSet::Layout,
                CompCoreSystemSet::Prepare,
                CompCoreSystemSet::Outline,
            )
                .chain(),
        );

        // Register systems
        app.add_systems(Update, outline_rectangle.in_set(CompCoreSystemSet::Outline));

        #[cfg(feature = "dtif")]
        inject_dtif_into_ecs(&mut app.world, &self.dtif)
    }
}

#[cfg(feature = "dtif")]
fn inject_dtif_into_ecs(world: &mut bevy_ecs::world::World, dtif: &dyn_comp_dtif::CompDtif) {
    let mut dtif_injector = dyn_comp_dtif::dtif_injector::DtifInjector::new();

    // Load fonts into cache
    // TODO

    // Load images into cache
    // TODO

    // Spawn nodes recursively
    let maybe_root_node_entity = dtif_injector.inject_from_root(dtif, world);
    if let Some(root_node_entity) = maybe_root_node_entity {
        world.insert_resource(CompositionRes {
            root_nodes: vec![root_node_entity],
            viewport: dtif.viewport,
            size: dtif.size,
        })
    }
}