use bevy_ecs::component::Component;
use glam::Vec2;

#[derive(Component, Debug, Default)]
pub struct InteractiveCompositionMixin {
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
