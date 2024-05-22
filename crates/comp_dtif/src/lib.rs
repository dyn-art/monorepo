use bevy_ecs::world::World;
use dyn_comp_bundles::{
    events::{
        CoreInputEvent, CreateAssetInputEvent, CreateNodeInputEvent, CreatePaintInputEvent,
        InputEvent,
    },
    properties::{CompVersion, Viewport},
    AssetWithId, Node, Paint,
};
use dyn_utils::properties::size::Size;

/// DTIF (Design Tree Interchange Format) utilizes a flat structure for easy readability
/// and efficient access & manipulation of design elements (Nodes, Paints, ..).
/// https://softwareengineering.stackexchange.com/questions/350623/flat-or-nested-json-for-hierarchal-data
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct DtifComposition {
    #[serde(default)]
    pub version: Option<CompVersion>,
    /// The absolute size of the composition.
    pub size: Size,
    /// The viewport of the composition.
    #[serde(default)]
    pub viewport: Option<Viewport>,
    /// The identifier of the root node in the composition.
    // pub root_node_ids: Vec<ReferenceIdOrEntity>, // TODO: Should be identified by system (e.g. when handling children events)
    /// A list of nodes.
    pub nodes: Vec<Node>,
    /// A list of paints.
    #[serde(default)]
    pub paints: Vec<Paint>,
    /// A list of assets.
    #[serde(default)]
    pub assets: Vec<AssetWithId>,
    // A list of input events.
    #[serde(default)]
    pub events: Vec<CoreInputEvent>,
}

impl DtifComposition {
    pub fn insert_into_world(self, world: &mut World) {
        for asset in self.assets {
            world.send_event(CreateAssetInputEvent { asset });
        }

        for node in self.nodes {
            world.send_event(CreateNodeInputEvent { node });
        }

        for paint in self.paints {
            world.send_event(CreatePaintInputEvent { paint });
        }

        for event in self.events {
            event.send_into_world(world);
        }
    }
}
