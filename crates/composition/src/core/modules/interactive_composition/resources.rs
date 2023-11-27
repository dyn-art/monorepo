use bevy_ecs::system::Resource;
use glam::Vec2;
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Resource, Debug, Default)]
pub struct InteractiveCompositionRes {
    pub interaction_mode: InteractionMode,
}

#[derive(Debug)]
pub enum InteractionMode {
    /// Default canvas mode. Nothing is happening.
    None,
    /// When the user's pointer is pressed.
    Pressing { origin: Vec2 },
    /// When the user is moving selected nodes.
    Translating { origin: Vec2, current: Vec2 },
    /// When the user is resizing selected nodes.
    Resizing {
        corner: u8,
        inital_bounds: XYWH,
        rotation: f32, // For cursor
    },
}

impl Default for InteractionMode {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Serialize, Deserialize, Type, Clone)]
pub struct XYWH {
    pub position: Vec2,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Serialize, Deserialize, Type, Clone)]
pub enum HandleSide {
    Top = 1,
    Bottom = 2,
    Left = 4,
    Right = 8,
}
