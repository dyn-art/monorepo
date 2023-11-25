use bevy_ecs::system::Resource;
use glam::Vec2;

#[derive(Resource, Debug, Default)]
pub struct InteractiveCompositionRes {
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
