use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use resources::composition::CompositionRes;
use systems::outline::rectangle::outline_rectangle;

pub mod resources;
pub mod systems;

// Game Plan
// Export as String
// 1. Identify RootCompNode
// 2. Get root SVGElement from SVGNode component which is attached to the RootCompNode entity
// 3. Walk the tree, needs context to commands or world though I guess to query other entities

// Determine Updates
// 1. Query for changed SVGNodes
// 2. Drain updates
// 3. Decide where to put which updates based on SVGNodes root SVGElements indent level and child index

pub struct CompPlugin {
    #[cfg(feature = "dtif")]
    pub dtif: dyn_dtif::DTIFComp,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum CompSystem {
    /// After this lable, input events got applied.
    Input,
    /// After this label, the layout got applied to the compositions nodes.
    Layout,
    /// After this label, the composition nodes got outlined.
    Outline,
}

impl Plugin for CompPlugin {
    fn build(&self, app: &mut App) {
        // Register events
        // TODO

        // Register resources
        // TODO

        // Register systems
        app.add_systems(PostUpdate, (outline_rectangle.in_set(CompSystem::Outline)));

        inject_dtif_into_ecs(&mut app.world, &self.dtif)
    }
}

fn inject_dtif_into_ecs(world: &mut World, dtif: &dyn_dtif::DTIFComp) {
    let mut dtif_injector = dyn_dtif::dtif_injector::DTIFInjector::new();

    // Load fonts into cache
    // TODO

    // Load images into cache
    // TODO

    // Spawn nodes recursively
    let maybe_root_node_entity = dtif_injector.inject_from_root(dtif, world);
    if let Some(root_node_entity) = maybe_root_node_entity {
        world.insert_resource(CompositionRes {
            version: dtif.version.clone(),
            name: dtif.name.clone(),
            root_node: root_node_entity,
            viewport: dtif.viewport,
            size: dtif.size,
        })
    }
}
