pub mod conversion;
pub mod events;

use dyn_comp_asset::asset::Asset;
use dyn_comp_bundles::{
    properties::{ReferenceIdOrEntity, Viewport},
    Node, Paint,
};
use dyn_utils::properties::size::Size;
use events::DtifInputEvent;

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
    pub root_node_id: ReferenceIdOrEntity,
    /// A list of nodes.
    pub nodes: Vec<Node>,
    /// A list of paints.
    #[serde(default)]
    pub paints: Vec<Paint>,
    /// A list of assets.
    #[serde(default)]
    pub assets: Vec<Asset>,
    // A list of input events.
    #[serde(default)]
    pub events: Vec<DtifInputEvent>,
}
