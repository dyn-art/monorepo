use bevy_ecs::entity::Entity;
use serde::{Deserialize, Serialize};
use specta::Type;

use super::modules::{
    composition::{events::CoreInputEvent, resources::font_cache::font::FontWithContent},
    node::components::{
        bundles::{FrameNodeBundle, GroupNodeBundle, RectangleNodeBundle, TextNodeBundle},
        mixins::Paint,
    },
};

pub mod dtif_processor;

#[derive(Serialize, Deserialize, Debug, Type, PartialEq, Eq, Hash, Clone, Copy)]
pub struct EntityId(u64);

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
    /// Note: Planned to directly use a Map once the referenced serde issue is resolved.
    ///       https://github.com/serde-rs/serde/issues/1183
    pub nodes: Vec<(EntityId, DTIFNode)>,

    /// A mapping of paint identifiers to their corresponding paints within the composition.
    /// Note: Planned to directly use a Map once the referenced serde issue is resolved.
    ///       https://github.com/serde-rs/serde/issues/1183
    pub paints: Vec<(EntityId, Paint)>,

    /// A mapping of font identifiers to their corresponding font data within the composition.
    /// Note: Planned to directly use a Map once the referenced serde issue is resolved.
    ///       https://github.com/serde-rs/serde/issues/1183
    #[serde(default)]
    pub fonts: Option<Vec<(u64, FontWithContent)>>,

    /// Optional list of changes represented as core input events.
    /// This field is optional and defaults to `None` if not provided.
    #[serde(default)]
    pub changes: Option<Vec<CoreInputEvent>>,
}

fn default_dtif_version() -> String {
    String::from("1.0")
}

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
#[serde(tag = "type")]
pub enum DTIFNode {
    Frame(FrameNodeBundle),
    Group(GroupNodeBundle),
    Rectangle(RectangleNodeBundle),
    Text(TextNodeBundle),
}
