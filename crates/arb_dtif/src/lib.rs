use bevy_ecs::world::World;
use dyn_arb_bundles::{
    events::{
        CoreInputEvent, CreateAssetInputEvent, CreateNodeInputEvent, CreatePaintInputEvent,
        InputEvent,
    },
    properties::{ArbVersion, Viewport},
    AssetWithId, Node, Paint,
};
use dyn_utils::properties::size::Size;

// Note: Cannot be inlined because of Utoipa lack of renaming Schema references
// https://github.com/juhaku/utoipa/issues/894#issuecomment-2164362189
#[cfg(feature = "lua_scripts")]
use dyn_arb_bundles::LuaScriptWithId;

/// DTIF (Design Tree Interchange Format) utilizes a flat structure for easy readability
/// and efficient access & manipulation of design elements (Nodes, Paints, ..).
/// https://softwareengineering.stackexchange.com/questions/350623/flat-or-nested-json-for-hierarchal-data
#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "specta_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(feature = "utoipa_support", derive(utoipa::ToSchema))]
pub struct DtifArtboard {
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub version: Option<ArbVersion>,
    /// The absolute size of the canvas.
    pub size: Size,
    /// The viewport of the canvas.
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub viewport: Option<Viewport>,
    /// A list of nodes.
    pub nodes: Vec<Node>,
    /// A list of paints.
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub paints: Vec<Paint>,
    /// A list of assets.
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub assets: Vec<AssetWithId>,
    /// A list of input events.
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub events: Vec<CoreInputEvent>,
    /// A list of scripts.
    #[cfg(feature = "lua_scripts")]
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub scripts: Vec<LuaScriptWithId>,
}

impl DtifArtboard {
    pub fn send_into_world(&mut self, world: &mut World) {
        for asset in std::mem::take(&mut self.assets) {
            world.send_event(CreateAssetInputEvent { asset });
        }

        for node in std::mem::take(&mut self.nodes) {
            world.send_event(CreateNodeInputEvent { node });
        }

        for paint in std::mem::take(&mut self.paints) {
            world.send_event(CreatePaintInputEvent { paint });
        }

        for event in std::mem::take(&mut self.events) {
            event.send_into_world(world);
        }

        #[cfg(feature = "lua_scripts")]
        for script in std::mem::take(&mut self.scripts) {
            world.send_event(dyn_arb_bundles::events::RegisterLuaScriptInputEvent { script });
        }
    }
}
