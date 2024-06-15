use crate::{
    input::{keyboard::KeyCode, mouse::MouseButton},
    resources::arb_interaction::{InteractionTool, XYWH},
};
use bevy_app::App;
use bevy_ecs::{entity::Entity, event::Event, world::World};
use dyn_arb_bundles::events::InputEvent;
use glam::Vec2;

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "specta_support",
    derive(serde::Deserialize, specta::Type),
    serde(tag = "type")
)]
pub enum InteractionInputEvent {
    // Artboard
    KeyDownOnArtboard(KeyDownOnArbInputEvent),
    KeyUpOnArtboard(KeyUpOnArbInputEvent),
    CursorEnteredArtboard(CursorEnteredArbInputEvent),
    CursorExitedArtboard(CursorExitedArbInputEvent),
    CursorMovedOnArtboard(CursorMovedOnArbInputEvent),
    CursorDownOnArtboard(CursorDownOnArbInputEvent),
    CursorUpOnArtboard(CursorUpOnArbInputEvent),
    MouseWheeledOnArtboard(MouseWheeledOnArbInputEvent),

    // Entity
    CursorDownOnEntity(CursorDownOnEntityInputEvent),

    // UI
    CursorDownOnResizeHandle(CursorDownOnResizeHandleInputEvent),
    CursorDownOnRotateHandle(CursorDownOnRotateHandleInputEvent),
    InteractionToolChanged(InteractionToolChangedInputEvent),
}

impl InputEvent for InteractionInputEvent {
    fn register_events(app: &mut App) {
        // Artboard
        app.add_event::<KeyDownOnArbInputEvent>();
        app.add_event::<KeyUpOnArbInputEvent>();
        app.add_event::<CursorEnteredArbInputEvent>();
        app.add_event::<CursorExitedArbInputEvent>();
        app.add_event::<CursorMovedOnArbInputEvent>();
        app.add_event::<CursorDownOnArbInputEvent>();
        app.add_event::<CursorUpOnArbInputEvent>();
        app.add_event::<MouseWheeledOnArbInputEvent>();

        // Entity
        app.add_event::<CursorDownOnEntityInputEvent>();

        // UI
        app.add_event::<CursorDownOnResizeHandleInputEvent>();
        app.add_event::<CursorDownOnRotateHandleInputEvent>();
        app.add_event::<InteractionToolChangedInputEvent>();
    }

    fn send_into_world(self, world: &mut World) {
        match self {
            // Artboard
            Self::KeyDownOnArtboard(event) => {
                world.send_event(event);
            }
            Self::KeyUpOnArtboard(event) => {
                world.send_event(event);
            }
            Self::CursorMovedOnArtboard(event) => {
                world.send_event(event);
            }
            Self::CursorDownOnArtboard(event) => {
                world.send_event(event);
            }
            Self::CursorUpOnArtboard(event) => {
                world.send_event(event);
            }
            Self::MouseWheeledOnArtboard(event) => {
                world.send_event(event);
            }
            Self::CursorEnteredArtboard(event) => {
                world.send_event(event);
            }
            Self::CursorExitedArtboard(event) => {
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
pub struct KeyDownOnArbInputEvent {
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
pub struct KeyUpOnArbInputEvent {
    /// The physical key code of the key.
    pub key_code: KeyCode,
    // /// The logical key of the input
    // pub logical_key: Key,
}

#[derive(Event, Debug, Copy, Clone)]
#[cfg_attr(feature = "specta_support", derive(serde::Deserialize, specta::Type))]
pub struct CursorMovedOnArbInputEvent {
    pub position: Vec2,
}

#[derive(Event, Debug, Copy, Clone)]
#[cfg_attr(feature = "specta_support", derive(serde::Deserialize, specta::Type))]
pub struct CursorEnteredArbInputEvent;

#[derive(Event, Debug, Copy, Clone)]
#[cfg_attr(feature = "specta_support", derive(serde::Deserialize, specta::Type))]
pub struct CursorExitedArbInputEvent;

#[derive(Event, Debug, Copy, Clone)]
#[cfg_attr(feature = "specta_support", derive(serde::Deserialize, specta::Type))]
pub struct CursorDownOnEntityInputEvent {
    pub entity: Entity,
    pub position: Vec2,
    pub button: MouseButton,
}

#[derive(Event, Debug, Copy, Clone)]
#[cfg_attr(feature = "specta_support", derive(serde::Deserialize, specta::Type))]
pub struct CursorDownOnArbInputEvent {
    pub position: Vec2,
    pub button: MouseButton,
}

#[derive(Event, Debug, Copy, Clone)]
#[cfg_attr(feature = "specta_support", derive(serde::Deserialize, specta::Type))]
pub struct CursorUpOnArbInputEvent {
    pub position: Vec2,
    pub button: MouseButton,
}

#[derive(Event, Debug, Copy, Clone)]
#[cfg_attr(
    feature = "specta_support",
    derive(serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct MouseWheeledOnArbInputEvent {
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
