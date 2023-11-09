use bevy_ecs::{entity::Entity, event::Event, world::World};
use serde::{Deserialize, Serialize};
use specta::Type;

use crate::core::events::input_event::InputEvent;

#[derive(Debug, Serialize, Deserialize, Type, Clone)]
#[serde(tag = "type")]
pub enum CoreInputEvent {
    EntityMoved(EntityMoved),
    EntitySetPosition(EntitySetPosition),
}

impl InputEvent for CoreInputEvent {
    fn send_to_ecs(self, world: &mut World) {
        match self {
            CoreInputEvent::EntityMoved(event) => {
                world.send_event(event);
            }
            CoreInputEvent::EntitySetPosition(event) => {
                world.send_event(event);
            }
        }
    }
}

// =============================================================================
// Entity Events
// =============================================================================

#[derive(Event, Debug, Serialize, Deserialize, Type, Clone)]
pub struct EntityMoved {
    pub entity: Entity,
    pub dx: f32,
    pub dy: f32,
}

#[derive(Event, Debug, Serialize, Deserialize, Type, Clone)]
pub struct EntitySetPosition {
    pub entity: Entity,
    pub x: f32,
    pub y: f32,
}
