use bevy_ecs::entity::Entity;
use dyn_composition::core::modules::node::components::types::NodeType;
use serde::Serialize;
use specta::Type;

use super::bindgen_render::RenderChange;

pub mod resources;

#[derive(Debug, Serialize, Clone, Type)]
pub enum OutputEvent {
    RenderUpdate {
        entity: Entity,
        node_type: NodeType,
        changes: Vec<RenderChange>,
    },
}
