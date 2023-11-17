use bevy_ecs::component::Component;
use serde::{Deserialize, Serialize};
use specta::Type;

/// Marks the root node.
///
/// This component is intended to be used with only one entity in the game world.
/// There is no enforcement at the Bevy framework level to ensure this uniqueness,
/// so it must be managed through game logic.
#[derive(Component, Serialize, Deserialize, Clone, Default, Debug, Type)]
pub struct Root;

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
pub enum NodeType {
    None,
    Group,
    Rectangle,
    Frame,
}

impl Default for NodeType {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Component, Serialize, Deserialize, Clone, Default, Debug, Type)]
pub struct Node {
    // Keep track of node type to know what node the renderer is dealing with in the render cycle
    // without a separate system for each node type/variant
    pub node_type: NodeType,
}

#[derive(Component, Serialize, Deserialize, Clone, Debug, Type)]
pub struct Frame {
    clip_content: bool,
}

impl Default for Frame {
    fn default() -> Self {
        Self {
            clip_content: false,
        }
    }
}

#[derive(Component, Serialize, Deserialize, Clone, Default, Debug, Type)]
pub struct Group;

#[derive(Component, Serialize, Deserialize, Clone, Default, Debug, Type)]
pub struct Rectangle;
