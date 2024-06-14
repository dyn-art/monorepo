use crate::{
    input::{keyboard::KeyCode, mouse::MouseButton},
    resources::cnv_interaction::{InteractionTool, XYWH},
};
use bevy_app::App;
use bevy_ecs::{entity::Entity, event::Event, world::World};
use dyn_cnv_bundles::events::InputEvent;
use glam::Vec2;

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "specta_support",
    derive(serde::Deserialize, specta::Type),
    serde(tag = "type")
)]
pub enum InteractionInputEvent {
    // Canvas
    KeyDownOnCanvas(KeyDownOnCnvInputEvent),
    KeyUpOnCanvas(KeyUpOnCnvInputEvent),
    CursorEnteredCanvas(CursorEnteredCnvInputEvent),
    CursorExitedCanvas(CursorExitedCnvInputEvent),
    CursorMovedOnCanvas(CursorMovedOnCnvInputEvent),
    CursorDownOnCanvas(CursorDownOnCnvInputEvent),
    CursorUpOnCanvas(CursorUpOnCnvInputEvent),
    MouseWheeledOnCanvas(MouseWheeledOnCnvInputEvent),

    // Entity
    CursorDownOnEntity(CursorDownOnEntityInputEvent),

    // UI
    CursorDownOnResizeHandle(CursorDownOnResizeHandleInputEvent),
    CursorDownOnRotateHandle(CursorDownOnRotateHandleInputEvent),
    InteractionToolChanged(InteractionToolChangedInputEvent),
}

impl InputEvent for InteractionInputEvent {
    fn register_events(app: &mut App) {
        // Canvas
        app.add_event::<KeyDownOnCnvInputEvent>();
        app.add_event::<KeyUpOnCnvInputEvent>();
        app.add_event::<CursorEnteredCnvInputEvent>();
        app.add_event::<CursorExitedCnvInputEvent>();
        app.add_event::<CursorMovedOnCnvInputEvent>();
        app.add_event::<CursorDownOnCnvInputEvent>();
        app.add_event::<CursorUpOnCnvInputEvent>();
        app.add_event::<MouseWheeledOnCnvInputEvent>();

        // Entity
        app.add_event::<CursorDownOnEntityInputEvent>();

        // UI
        app.add_event::<CursorDownOnResizeHandleInputEvent>();
        app.add_event::<CursorDownOnRotateHandleInputEvent>();
        app.add_event::<InteractionToolChangedInputEvent>();
    }

    fn send_into_world(self, world: &mut World) {
        match self {
            // Canvas
            Self::KeyDownOnCanvas(event) => {
                world.send_event(event);
            }
            Self::KeyUpOnCanvas(event) => {
                world.send_event(event);
            }
            Self::CursorMovedOnCanvas(event) => {
                world.send_event(event);
            }
            Self::CursorDownOnCanvas(event) => {
                world.send_event(event);
            }
            Self::CursorUpOnCanvas(event) => {
                world.send_event(event);
            }
            Self::MouseWheeledOnCanvas(event) => {
                world.send_event(event);
            }
            Self::CursorEnteredCanvas(event) => {
                world.send_event(event);
            }
            Self::CursorExitedCanvas(event) => {
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
    feature = "specta_support",
    derive(serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct KeyDownOnCnvInputEvent {
    /// The physical key code of the key.
    pub key_code: KeyCode,
    // /// The logical key of the input
    // pub logical_key: Key,
}

#[derive(Event, Debug, Clone)]
#[cfg_attr(
    feature = "specta_support",
    derive(serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct KeyUpOnCnvInputEvent {
    /// The physical key code of the key.
    pub key_code: KeyCode,
    // /// The logical key of the input
    // pub logical_key: Key,
}

#[derive(Event, Debug, Copy, Clone)]
#[cfg_attr(feature = "specta_support", derive(serde::Deserialize, specta::Type))]
pub struct CursorMovedOnCnvInputEvent {
    pub position: Vec2,
}

#[derive(Event, Debug, Copy, Clone)]
#[cfg_attr(feature = "specta_support", derive(serde::Deserialize, specta::Type))]
pub struct CursorEnteredCnvInputEvent;

#[derive(Event, Debug, Copy, Clone)]
#[cfg_attr(feature = "specta_support", derive(serde::Deserialize, specta::Type))]
pub struct CursorExitedCnvInputEvent;

#[derive(Event, Debug, Copy, Clone)]
#[cfg_attr(feature = "specta_support", derive(serde::Deserialize, specta::Type))]
pub struct CursorDownOnEntityInputEvent {
    pub entity: Entity,
    pub position: Vec2,
    pub button: MouseButton,
}

#[derive(Event, Debug, Copy, Clone)]
#[cfg_attr(feature = "specta_support", derive(serde::Deserialize, specta::Type))]
pub struct CursorDownOnCnvInputEvent {
    pub position: Vec2,
    pub button: MouseButton,
}

#[derive(Event, Debug, Copy, Clone)]
#[cfg_attr(feature = "specta_support", derive(serde::Deserialize, specta::Type))]
pub struct CursorUpOnCnvInputEvent {
    pub position: Vec2,
    pub button: MouseButton,
}

#[derive(Event, Debug, Copy, Clone)]
#[cfg_attr(
    feature = "specta_support",
    derive(serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct MouseWheeledOnCnvInputEvent {
    pub position: Vec2,
    pub delta: Vec2,
}

#[derive(Event, Debug, Copy, Clone)]
#[cfg_attr(
    feature = "specta_support",
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
    feature = "specta_support",
    derive(serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct CursorDownOnRotateHandleInputEvent {
    pub corner: u8,
    pub initial_rotation_rad: f32,
}

#[derive(Event, Debug, Copy, Clone)]
#[cfg_attr(feature = "specta_support", derive(serde::Deserialize, specta::Type))]
pub struct InteractionToolChangedInputEvent {
    pub tool: InteractionTool,
}
