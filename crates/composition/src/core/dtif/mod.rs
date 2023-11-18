use std::collections::HashMap;

use bevy_ecs::entity::Entity;
use serde::{Deserialize, Serialize};
use specta::Type;

use super::modules::{
    composition::events::CoreInputEvent,
    node::components::bundles::{FrameNodeBundle, GroupNodeBundle, RectangleNodeBundle},
};

pub mod dtif_processor;

/// Represents the composition in which all nodes exist.
#[derive(Serialize, Deserialize, Debug, Type)]
pub struct DTIFComposition {
    /// The version of the composition type declaration, used internally.
    /// Defaults to the latest version.
    #[serde(default = "default_dtif_version")]
    pub version: String,

    /// The name of the composition.
    /// Example: 'My super cool composition'.
    pub name: String,

    /// The width of the composition, in units.
    pub width: f32,

    /// The height of the composition, in units.
    pub height: f32,

    /// The identifier of the root node in the composition.
    #[serde(rename = "rootNodeId")]
    pub root_node_id: Entity,

    /// A mapping of node identifiers to their corresponding nodes within the composition.
    /// Note: Planned to use `Entity` as a key once a specific serde issue is resolved.
    ///       https://github.com/serde-rs/serde/issues/1183
    pub nodes: HashMap<String, DTIFNode>,

    /// Optional list of changes represented as core input events.
    /// This field is optional and defaults to `None` if not provided.
    #[serde(default)]
    pub changes: Option<Vec<CoreInputEvent>>,
}

fn default_dtif_version() -> String {
    String::from("1.0")
}

#[derive(Serialize, Deserialize, Debug, Type)]
#[serde(tag = "type")]
pub enum DTIFNode {
    Rectangle(RectangleNodeBundle),
    Frame(FrameNodeBundle),
    Group(GroupNodeBundle),
}
