use bevy_ecs::prelude::*;
use bevy_transform::prelude::*;
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
