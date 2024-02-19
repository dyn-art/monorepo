use crate::node::Node;
use dyn_comp_types::shared::Size;
use glam::Vec2;
use std::collections::HashMap;

pub mod dtif_injector;
pub mod node;

#[derive(Debug, Clone, serde::Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct DTIFComp {
    /// The version of the composition type declaration.
    pub version: String,
    /// The name of the composition.
    pub name: String,
    /// The size of the composition in pixels.
    pub size: Size,
    /// The viewport defines the area on the render target to which the camera renders its image.
    #[serde(default)]
    pub viewport: Viewport,
    /// The identifier of the root node in the composition.
    pub root_node_id: String,
    /// A mapping of node identifiers to their corresponding nodes within the composition.
    pub nodes: HashMap<String, Node>,
    /// A mapping of image identifiers to their corresponding images within the composition.
    pub images: HashMap<String, Content>,
    /// A list of font data.
    #[serde(default)]
    pub fonts: Vec<Content>,
}

#[derive(Debug, Default, Copy, Clone, serde::Serialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Viewport {
    pub physical_position: Vec2,
    pub physical_size: Vec2,
}

#[derive(Debug, Clone, serde::Serialize, specta::Type)]
#[serde(tag = "type")]
pub enum Content {
    /// Content stored as binary data.
    Binary { content: Vec<u8> },
    /// Content referenced by a URL.
    Url { url: String },
}
