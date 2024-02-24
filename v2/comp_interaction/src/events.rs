use bevy_ecs::{entity::Entity, event::Event, world::World};
use dyn_comp_types::events::InputEvent;
use glam::Vec2;

use crate::resources::comp_interaction::{MouseButton, XYWH};

#[derive(Debug, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Deserialize, specta::Type),
    serde(tag = "type")
)]
pub enum InteractionInputEvent {
    CursorDownOnEntity(CursorDownOnEntity),
    CursorMovedOnComposition(CursorMovedOnComposition),
    CursorEnteredComposition(CursorEnteredComposition),
    CursorExitedComposition(CursorExitedComposition),
    CursorDownOnComposition(CursorDownOnComposition),
    CursorUpOnComposition(CursorUpOnComposition),
    WheeledOnComposition(WheeledOnComposition),
    CursorDownOnResizeHandle(CursorDownOnResizeHandle),
    CursorDownOnRotateHandle(CursorDownOnRotateHandle),
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
        }
    }
}

#[derive(Event, Debug, Copy, Clone)]
#[cfg_attr(feature = "serde_support", derive(serde::Deserialize, specta::Type))]
pub struct CursorMovedOnComposition {
    pub position: Vec2,
}

#[derive(Event, Debug, Copy, Clone)]
#[cfg_attr(feature = "serde_support", derive(serde::Deserialize, specta::Type))]
pub struct CursorEnteredComposition;

#[derive(Event, Debug, Copy, Clone)]
#[cfg_attr(feature = "serde_support", derive(serde::Deserialize, specta::Type))]
pub struct CursorExitedComposition;

#[derive(Event, Debug, Copy, Clone)]
#[cfg_attr(feature = "serde_support", derive(serde::Deserialize, specta::Type))]
pub struct CursorDownOnEntity {
    pub entity: Entity,
    pub position: Vec2,
    pub button: MouseButton,
}

#[derive(Event, Debug, Copy, Clone)]
#[cfg_attr(feature = "serde_support", derive(serde::Deserialize, specta::Type))]
pub struct CursorDownOnComposition {
    pub position: Vec2,
    pub button: MouseButton,
}

#[derive(Event, Debug, Copy, Clone)]
#[cfg_attr(feature = "serde_support", derive(serde::Deserialize, specta::Type))]
pub struct CursorUpOnComposition {
    pub position: Vec2,
    pub button: MouseButton,
}

#[derive(Event, Debug, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct WheeledOnComposition {
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
pub struct CursorDownOnResizeHandle {
    pub initial_bounds: XYWH,
    pub corner: u8,
    pub rotation_in_radians: f32,
}

#[derive(Event, Debug, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct CursorDownOnRotateHandle {
    pub corner: u8,
    pub initial_rotation_in_radians: f32,
}
