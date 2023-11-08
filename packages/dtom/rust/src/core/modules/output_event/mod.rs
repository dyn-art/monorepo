use bevy_ecs::entity::Entity;
use dyn_composition::core::modules::node::components::types::NodeType;
use serde::Serialize;
use specta::Type;

use super::bindgen_render::RenderChange;

pub mod resources;

#[derive(Debug, Serialize, Clone, Type)]
#[serde(tag = "type")]
pub enum OutputEvent {
    RenderUpdate(RenderUpdateEvent),
}

#[derive(Debug, Serialize, Clone, Type)]
pub struct RenderUpdateEvent {
    pub entity: Entity,
    #[serde(rename = "nodeType")]
    pub node_type: NodeType,
    pub changes: Vec<RenderChange>,
}
