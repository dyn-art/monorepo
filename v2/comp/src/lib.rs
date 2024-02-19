use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_transform::prelude::*;
use dyn_dtif::dtif_injector;
use glam::{UVec2, Vec2, Vec4};
use smallvec::SmallVec;

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

impl Plugin for CompPlugin {
    fn build(&self, app: &mut App) {
        // Register events
        // TODO

        // Register resources
        // TODO

        // Register systems
        // TODO

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
        // TODO
    }
}
