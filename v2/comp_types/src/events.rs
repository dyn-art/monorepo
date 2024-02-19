use std::fmt::Debug;

use bevy_ecs::{entity::Entity, event::Event, world::World};

use crate::shared::{Size, Viewport};

pub trait InputEvent {
    fn send_into_ecs(self, world: &mut World);
}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(tag = "type")
)]
pub enum CompInputEvent {
    CompositionResized(CompositionResizedEvent),
    CompositionViewportChanged(CompositionViewportChangedEvent),
    EntityMoved(EntityMovedEvent),
    EntitySetPosition(EntitySetPositionEvent),
    EntityDeleted(EntityDeletedEvent),
}

impl InputEvent for CompInputEvent {
    fn send_into_ecs(self, world: &mut World) {
        match self {
            CompInputEvent::CompositionResized(event) => {
                world.send_event(event);
            }
            CompInputEvent::CompositionViewportChanged(event) => {
                world.send_event(event);
            }
            CompInputEvent::EntityMoved(event) => {
                world.send_event(event);
            }
            CompInputEvent::EntitySetPosition(event) => {
                world.send_event(event);
            }
            CompInputEvent::EntityDeleted(event) => {
                world.send_event(event);
            }
        }
    }
}

#[derive(Event, Debug, Default, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct CompositionResizedEvent {
    pub size: Size,
}

#[derive(Event, Debug, Default, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct CompositionViewportChangedEvent {
    pub viewport: Viewport,
}

#[derive(Event, Debug, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct EntityMovedEvent {
    pub entity: Entity,
    pub dx: f32,
    pub dy: f32,
}

#[derive(Event, Debug, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct EntitySetPositionEvent {
    pub entity: Entity,
    pub x: f32,
    pub y: f32,
}

#[derive(Event, Debug, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct EntityDeletedEvent {
    pub entity: Entity,
}
