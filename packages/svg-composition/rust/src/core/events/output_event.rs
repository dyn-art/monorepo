use bevy_ecs::entity::Entity;
use dyn_composition::core::modules::interactive_composition::resources::InteractionMode;
use serde::Serialize;
use specta::Type;

use crate::core::{mixin_change::MixinChange, modules::svg_render::render_change::RenderChange};

#[derive(Debug, Serialize, Clone, Type)]
#[serde(tag = "type")]
pub enum OutputEvent {
    RenderUpdate(RenderUpdateEvent),
    TrackUpdate(TrackUpdateEvent),
    SelectionChange(SelectionChangeEvent),
    InteractionModeChange(InteractionModeChangeEvent),
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

#[derive(Debug, Serialize, Clone, Type)]
pub struct SelectionChangeEvent {
    pub selected: Vec<Entity>,
}

#[derive(Debug, Serialize, Clone, Type)]
pub struct InteractionModeChangeEvent {
    #[serde(rename = "interactionMode")]
    pub interaction_mode: RawInteractionMode,
}

#[derive(Debug, Serialize, Clone, Type, Eq, PartialEq)]
#[serde(tag = "type")]
pub enum RawInteractionMode {
    None,
    Translating,
    Pressing,
}

impl Default for RawInteractionMode {
    fn default() -> Self {
        Self::None
    }
}
