use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use dyn_comp_types::prelude::*;
use resources::composition::CompositionRes;
use systems::outline::rectangle::outline_rectangle;

pub mod resources;
pub mod systems;

pub mod prelude {
    pub use super::CompPlugin;
    pub use dyn_comp_types::prelude::*;
    pub use dyn_dtif::*;
}

pub struct CompPlugin {
    #[cfg(feature = "dtif")]
    pub dtif: dyn_dtif::DtifComp,
    #[cfg(not(feature = "dtif"))]
    pub size: Size,
    #[cfg(not(feature = "dtif"))]
    pub viewport: Option<Viewport>,
    #[cfg(not(feature = "dtif"))]
    pub root_nodes: Vec<Entity>,
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum CompSystemSet {
    /// After this lablel, input events got applied.
    InputEvents,
    /// After this label, the layout got applied to the compositions nodes.
    Layout,
    /// After this label, the composition nodes got outlined.
    Outline,
}

impl Plugin for CompPlugin {
    fn build(&self, app: &mut App) {
        // Register events
        app.add_event::<CompositionResizedEvent>();
        app.add_event::<CompositionViewportChangedEvent>();
        app.add_event::<EntityDeletedEvent>();
        app.add_event::<EntityMovedEvent>();
        app.add_event::<EntitySetPositionEvent>();

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
                CompSystemSet::InputEvents,
                CompSystemSet::Layout,
                CompSystemSet::Outline,
            )
                .chain(),
        );

        // Register systems
        app.add_systems(Update, outline_rectangle.in_set(CompSystemSet::Outline));

        #[cfg(feature = "dtif")]
        inject_dtif_into_ecs(&mut app.world, &self.dtif)
    }
}

#[cfg(feature = "dtif")]
fn inject_dtif_into_ecs(world: &mut World, dtif: &dyn_dtif::DtifComp) {
    let mut dtif_injector = dyn_dtif::dtif_injector::DtifInjector::new();

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
