pub mod lua;

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
#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct DtifComposition {
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub version: Option<CompVersion>,
    /// The absolute size of the composition.
    pub size: Size,
    /// The viewport of the composition.
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub viewport: Option<Viewport>,
    /// A list of nodes.
    pub nodes: Vec<Node>,
    /// A list of paints.
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub paints: Vec<Paint>,
    /// A list of assets.
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub assets: Vec<AssetWithId>,
    /// A list of input events.
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub events: Vec<CoreInputEvent>,
    /// A map of scripts.
    #[cfg(feature = "lua_scripts")]
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub scripts: std::collections::HashMap<String, lua::script::LuaScript>,
}

impl DtifComposition {
    pub fn send_into_world(self, world: &mut World) {
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
