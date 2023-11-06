//! Events emitted by the Composition

use bevy_ecs::entity::Entity;
use dyn_dtom::core::composition::nodes::types::NodeType;
use serde::Serialize;
use specta::Type;

use crate::composition_api::plugins::bindgen_render_plugin::RenderChange;

#[derive(Debug, Serialize, Clone, Type)]
pub enum OutputEvent {
    RenderUpdate {
        entity: Entity,
        node_type: NodeType,
        changes: Vec<RenderChange>,
    },
}
