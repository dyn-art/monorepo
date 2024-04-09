use crate::input::mouse::MouseButton;
use bevy_ecs::{entity::Entity, system::Resource};
use dyn_utils::properties::size::Size;
use glam::Vec2;

#[derive(Resource, Debug, Default)]
pub struct CompInteractionRes {
    pub interaction_tool: InteractionTool,
    pub interaction_mode: InteractionMode,
}

#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(tag = "type")
)]
pub enum InteractionTool {
    #[default]
    /// When the user wants to select nodes and move them around.
    Select,
    /// When the user wants to insert new shape nodes.
    Shape { variant: ShapeVariant },
}

#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub enum ShapeVariant {
    #[default]
    Rectangle,
}

#[derive(Debug, Default, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Deserialize, specta::Type),
    serde(tag = "type")
)]
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
        rotation_deg: f32, // For cursor
    },
    /// When the user is rotating the selected nodes.
    Rotating {
        corner: u8,
        initial_rotation_rad: f32,
        rotation_deg: f32, // For cursor
    },
    /// When the user plans to insert a new node.
    Inserting {
        initial_bounds: XYWH,
        shape_variant: ShapeVariant,
        entity: Option<Entity>,
    },
}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde_support", derive(serde::Deserialize, specta::Type))]
pub struct XYWH {
    pub position: Vec2,
    pub size: Size,
}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde_support", derive(serde::Deserialize, specta::Type))]
pub enum HandleSide {
    Top = 1,
    Bottom = 2,
    Left = 4,
    Right = 8,
}
