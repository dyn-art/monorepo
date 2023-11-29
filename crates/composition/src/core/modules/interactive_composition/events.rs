use bevy_ecs::{entity::Entity, event::Event, world::World};
use glam::Vec2;
use serde::{Deserialize, Serialize};
use specta::Type;

use crate::core::events::input_event::InputEvent;

use super::resources::XYWH;

#[derive(Debug, Deserialize, Type, Clone)]
#[serde(tag = "type")]
pub enum InteractionInputEvent {
    CursorDownOnEntity(CursorDownOnEntity),
    CursorMovedOnComposition(CursorMovedOnComposition),
    CursorEnteredComposition(CursorEnteredComposition),
    CursorExitedComposition(CursorExitedComposition),
    CursorDownOnComposition(CursorDownOnComposition),
    CursorUpOnComposition(CursorUpOnComposition),
    CursorDownOnResizeHandle(CursorDownOnResizeHandle),
    CursorDownOnRotateHandle(CursorDownOnRotateHandle),
}

impl InputEvent for InteractionInputEvent {
    fn send_to_ecs(self, world: &mut World) {
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
        }
    }
}

// =============================================================================
// Cursor Events
// =============================================================================

#[derive(Event, Debug, Deserialize, Type, Clone)]
pub struct CursorMovedOnComposition {
    pub position: Vec2,
}

#[derive(Event, Debug, Deserialize, Type, Clone)]
pub struct CursorEnteredComposition;

#[derive(Event, Debug, Deserialize, Type, Clone)]
pub struct CursorExitedComposition;

#[derive(Event, Debug, Serialize, Deserialize, Type, Clone)]
pub struct CursorDownOnEntity {
    pub entity: Entity,
    pub position: Vec2,
}

#[derive(Event, Debug, Serialize, Deserialize, Type, Clone)]
pub struct CursorDownOnComposition {
    pub position: Vec2,
}

#[derive(Event, Debug, Serialize, Deserialize, Type, Clone)]
pub struct CursorUpOnComposition {
    pub position: Vec2,
}

#[derive(Event, Debug, Serialize, Deserialize, Type, Clone)]
pub struct CursorDownOnResizeHandle {
    #[serde(rename = "initialBounds")]
    pub initial_bounds: XYWH,
    pub corner: u8,
    pub rotation: f32,
}

#[derive(Event, Debug, Serialize, Deserialize, Type, Clone)]
pub struct CursorDownOnRotateHandle {
    pub corner: u8,
    #[serde(rename = "initialRotation")]
    pub initial_rotation: f32,
}
