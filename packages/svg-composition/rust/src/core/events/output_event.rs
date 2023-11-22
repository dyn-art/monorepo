use bevy_ecs::entity::Entity;
use serde::Serialize;
use specta::Type;

use crate::core::{mixin_change::MixinChange, modules::svg_render::render_change::RenderChange};

#[derive(Debug, Serialize, Clone, Type)]
#[serde(tag = "type")]
pub enum OutputEvent {
    RenderUpdate(RenderUpdateEvent),
    TrackUpdate(TrackUpdateEvent),
}

#[derive(Debug, Serialize, Clone, Type)]
pub struct RenderUpdateEvent {
    pub id: u32,
    pub updates: Vec<RenderChange>,
}

#[derive(Debug, Serialize, Clone, Type)]
pub struct TrackUpdateEvent {
    pub id: Entity,
    pub updates: Vec<MixinChange>,
}
