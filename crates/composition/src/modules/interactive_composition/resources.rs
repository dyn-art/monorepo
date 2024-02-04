use bevy_ecs::system::Resource;
use glam::Vec2;
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Resource, Debug)]
pub struct InteractiveCompositionRes {
    pub interaction_mode: InteractionMode,
}

#[derive(Debug, Default)]
pub enum InteractionMode {
    /// Default canvas mode. Nothing is happening.
    #[default]
    None,
    /// When the user's pointer is pressed.
    Pressing { origin: Vec2, button: MouseButton },
    /// When the user is dragging.
    Dragging { current: Vec2 },
    /// When the user is moving selected nodes.
    Translating { origin: Vec2, current: Vec2 },
    /// When the user is resizing the selected nodes.
    Resizing {
        corner: u8,
        initial_bounds: XYWH,
        rotation_in_degrees: f32, // For cursor
    },
    /// When the user is rotating the selected nodes.
    Rotating {
        corner: u8,
        initial_rotation_in_radians: f32,
        rotation_in_degrees: f32, // For cursor
    },
}

#[derive(Debug, Serialize, Deserialize, Type, Clone)]
pub struct XYWH {
    pub position: Vec2,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Serialize, Deserialize, Type, Clone)]
pub enum HandleSide {
    Top = 1,
    Bottom = 2,
    Left = 4,
    Right = 8,
}

#[derive(Serialize, Deserialize, PartialEq, Type, Copy, Clone, Debug)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    Unkown,
}
