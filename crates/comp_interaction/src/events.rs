use crate::{
    input::{keyboard::KeyCode, mouse::MouseButton},
    resources::comp_interaction::{InteractionTool, XYWH},
};
use bevy_ecs::{entity::Entity, event::Event, world::World};
use dyn_comp_bundles::events::InputEvent;
use glam::Vec2;

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Deserialize, specta::Type),
    serde(tag = "type")
)]
pub enum InteractionInputEvent {
    // Composition
    KeyDownOnComposition(KeyDownOnCompInputEvent),
    KeyUpOnComposition(KeyUpOnCompInputEvent),
    CursorEnteredComposition(CursorEnteredCompInputEvent),
    CursorExitedComposition(CursorExitedCompInputEvent),
    CursorMovedOnComposition(CursorMovedOnCompInputEvent),
    CursorDownOnComposition(CursorDownOnCompInputEvent),
    CursorUpOnComposition(CursorUpOnCompInputEvent),
    MouseWheeledOnComposition(MouseWheeledOnCompInputEvent),

    // Entity
    CursorDownOnEntity(CursorDownOnEntityInputEvent),

    // UI
    CursorDownOnResizeHandle(CursorDownOnResizeHandleInputEvent),
    CursorDownOnRotateHandle(CursorDownOnRotateHandleInputEvent),
    InteractionToolChanged(InteractionToolChangedInputEvent),
}

impl InputEvent for InteractionInputEvent {
    fn send_into_ecs(self, world: &mut World) {
        match self {
            // Composition
            Self::KeyDownOnComposition(event) => {
                world.send_event(event);
            }
            Self::KeyUpOnComposition(event) => {
                world.send_event(event);
            }
            Self::CursorMovedOnComposition(event) => {
                world.send_event(event);
            }
            Self::CursorDownOnComposition(event) => {
                world.send_event(event);
            }
            Self::CursorUpOnComposition(event) => {
                world.send_event(event);
            }
            Self::MouseWheeledOnComposition(event) => {
                world.send_event(event);
            }
            Self::CursorEnteredComposition(event) => {
                world.send_event(event);
            }
            Self::CursorExitedComposition(event) => {
                world.send_event(event);
            }

            // Entity
            Self::CursorDownOnEntity(event) => {
                world.send_event(event);
            }

            // UI
            Self::CursorDownOnResizeHandle(event) => {
                world.send_event(event);
            }
            Self::CursorDownOnRotateHandle(event) => {
                world.send_event(event);
            }
            Self::InteractionToolChanged(event) => {
                world.send_event(event);
            }
        }
    }
}

#[derive(Event, Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct KeyDownOnCompInputEvent {
    /// The physical key code of the key.
    pub key_code: KeyCode,
    // /// The logical key of the input
    // pub logical_key: Key,
}

#[derive(Event, Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct KeyUpOnCompInputEvent {
    /// The physical key code of the key.
    pub key_code: KeyCode,
    // /// The logical key of the input
    // pub logical_key: Key,
}

#[derive(Event, Debug, Copy, Clone)]
#[cfg_attr(feature = "serde_support", derive(serde::Deserialize, specta::Type))]
pub struct CursorMovedOnCompInputEvent {
    pub position: Vec2,
}

#[derive(Event, Debug, Copy, Clone)]
#[cfg_attr(feature = "serde_support", derive(serde::Deserialize, specta::Type))]
pub struct CursorEnteredCompInputEvent;

#[derive(Event, Debug, Copy, Clone)]
#[cfg_attr(feature = "serde_support", derive(serde::Deserialize, specta::Type))]
pub struct CursorExitedCompInputEvent;

#[derive(Event, Debug, Copy, Clone)]
#[cfg_attr(feature = "serde_support", derive(serde::Deserialize, specta::Type))]
pub struct CursorDownOnEntityInputEvent {
    pub entity: Entity,
    pub position: Vec2,
    pub button: MouseButton,
}

#[derive(Event, Debug, Copy, Clone)]
#[cfg_attr(feature = "serde_support", derive(serde::Deserialize, specta::Type))]
pub struct CursorDownOnCompInputEvent {
    pub position: Vec2,
    pub button: MouseButton,
}

#[derive(Event, Debug, Copy, Clone)]
#[cfg_attr(feature = "serde_support", derive(serde::Deserialize, specta::Type))]
pub struct CursorUpOnCompInputEvent {
    pub position: Vec2,
    pub button: MouseButton,
}

#[derive(Event, Debug, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct MouseWheeledOnCompInputEvent {
    pub position: Vec2,
    pub delta: Vec2,
}

#[derive(Event, Debug, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct CursorDownOnResizeHandleInputEvent {
    pub initial_bounds: XYWH,
    pub corner: u8,
    pub rotation_rad: f32,
}

#[derive(Event, Debug, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct CursorDownOnRotateHandleInputEvent {
    pub corner: u8,
    pub initial_rotation_rad: f32,
}

#[derive(Event, Debug, Copy, Clone)]
#[cfg_attr(feature = "serde_support", derive(serde::Deserialize, specta::Type))]
pub struct InteractionToolChangedInputEvent {
    pub tool: InteractionTool,
}
