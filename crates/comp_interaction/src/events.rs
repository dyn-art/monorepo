use bevy_ecs::{entity::Entity, event::Event, world::World};
use dyn_comp_bundles::events::InputEvent;
use glam::Vec2;

use crate::resources::comp_interaction::{InteractionTool, MouseButton, XYWH};

#[derive(Debug, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Deserialize, specta::Type),
    serde(tag = "type")
)]
pub enum InteractionInputEvent {
    CursorDownOnEntity(CursorDownOnEntityInputEvent),
    CursorMovedOnComposition(CursorMovedOnCompInputEvent),
    CursorEnteredComposition(CursorEnteredCompInputEvent),
    CursorExitedComposition(CursorExitedCompInputEvent),
    CursorDownOnComposition(CursorDownOnCompInputEvent),
    CursorUpOnComposition(CursorUpOnCompInputEvent),
    WheeledOnComposition(WheeledOnCompInputEvent),
    CursorDownOnResizeHandle(CursorDownOnResizeHandleInputEvent),
    CursorDownOnRotateHandle(CursorDownOnRotateHandleInputEvent),
    InteractionToolChanged(InteractionToolChangedInputEvent),
}

impl InputEvent for InteractionInputEvent {
    fn send_into_ecs(self, world: &mut World) {
        match self {
            InteractionInputEvent::CursorMovedOnComposition(event) => {
                world.send_event(event);
            }
            InteractionInputEvent::CursorEnteredComposition(event) => {
                world.send_event(event);
            }
            InteractionInputEvent::CursorExitedComposition(event) => {
                world.send_event(event);
            }
            InteractionInputEvent::CursorDownOnEntity(event) => {
                world.send_event(event);
            }
            InteractionInputEvent::CursorDownOnComposition(event) => {
                world.send_event(event);
            }
            InteractionInputEvent::CursorUpOnComposition(event) => {
                world.send_event(event);
            }
            InteractionInputEvent::CursorDownOnResizeHandle(event) => {
                world.send_event(event);
            }
            InteractionInputEvent::CursorDownOnRotateHandle(event) => {
                world.send_event(event);
            }
            InteractionInputEvent::WheeledOnComposition(event) => {
                world.send_event(event);
            }
            InteractionInputEvent::InteractionToolChanged(event) => {
                world.send_event(event);
            }
        }
    }
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
pub struct WheeledOnCompInputEvent {
    pub position: Vec2,
    pub delta: Vec2,
    pub ctrl_key_pressed: bool,
    pub meta_key_pressed: bool,
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
