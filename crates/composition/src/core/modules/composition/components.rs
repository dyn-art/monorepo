use bevy_ecs::{component::Component, entity::Entity};
use glam::Vec2;

#[derive(Component, Debug)]
pub struct CompositionMixin {
    pub version: String,
    pub name: String,
    pub width: f32,
    pub height: f32,
    pub root_node: Entity,
}

#[derive(Component, Debug, Default)]
pub struct CompositionInteractionMixin {
    pub interaction_mode: InteractionMode,
}

#[derive(Debug)]
pub enum InteractionMode {
    None,
    Translating { origin: Vec2, current: Vec2 },
    Pressing { origin: Vec2 },
}

impl Default for InteractionMode {
    fn default() -> Self {
        Self::None
    }
}
