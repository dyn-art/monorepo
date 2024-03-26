pub mod conversion;
pub mod dtif_injector;
pub mod events;
pub mod nodes;
pub mod paints;
pub mod styles;

use crate::nodes::Node;
use dtif_injector::DtifInjector;
use dyn_comp_asset::asset::Asset;
use dyn_comp_bundles::properties::Viewport;
use dyn_utils::properties::size::Size;
use events::DtifInputEvent;
use paints::Paint;
use std::collections::HashMap;

/// DTIF (Design Tree Interchange Format) utilizes a flat structure for easy readability
/// and efficient access & manipulation of design elements (Nodes, Paints, ..).
/// https://softwareengineering.stackexchange.com/questions/350623/flat-or-nested-json-for-hierarchal-data
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct DtifComposition {
    /// The version of the composition type declaration.
    #[serde(default)]
    pub version: Option<String>,
    /// The size of the composition in pixels.
    pub size: Size,
    /// The viewport defines the area on the render target to which the camera renders its image.
    #[serde(default)]
    pub viewport: Option<Viewport>,
    /// The identifier of the root node in the composition.
    pub root_node_id: String,
    /// A mapping of node identifiers to their corresponding nodes within the composition.
    pub nodes: HashMap<String, Node>,
    /// A mapping of paint identifiers to their corresponding paints within the composition.
    #[serde(default)]
    pub paints: HashMap<String, Paint>,
    /// A mapping of asset identifiers to their corresponding assets within the composition.
    #[serde(default)]
    pub assets: HashMap<String, Asset>,
    // A list of input events.
    #[serde(default)]
    pub events: Vec<DtifInputEvent>,
}

pub trait ToEcsBundleImpl {
    type Bundle: bevy_ecs::bundle::Bundle;
    fn to_ecs_bundle(&self, dtif_injector: &DtifInjector) -> Self::Bundle;
}
