use std::collections::HashMap;

use dyn_comp_types::{
    bevy_ecs::entity::Entity,
    events::CompInputEvent,
    shared::{Size, Viewport},
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(tag = "type")]
pub enum DTIFInputEvent {
    CompositionResized(CompositionResizedEvent),
    CompositionViewportChanged(CompositionViewportChangedEvent),
    EntityMoved(EntityMovedEvent),
    EntitySetPosition(EntitySetPositionEvent),
    EntityDeleted(EntityDeletedEvent),
}

impl DTIFInputEvent {
    pub fn into_comp_input_event(
        self,
        sid_to_entity: &HashMap<String, Entity>,
    ) -> Option<CompInputEvent> {
        use dyn_comp_types::events::*;

        match self {
            DTIFInputEvent::CompositionResized(event) => Some(CompInputEvent::CompositionResized(
                CompositionResizedEvent { size: event.size },
            )),
            DTIFInputEvent::CompositionViewportChanged(event) => Some(
                CompInputEvent::CompositionViewportChanged(CompositionViewportChangedEvent {
                    viewport: event.viewport,
                }),
            ),
            DTIFInputEvent::EntityMoved(event) => sid_to_entity.get(&event.entity).map(|entity| {
                CompInputEvent::EntityMoved(EntityMovedEvent {
                    dx: event.dx,
                    dy: event.dy,
                    entity: *entity,
                })
            }),
            DTIFInputEvent::EntitySetPosition(event) => {
                sid_to_entity.get(&event.entity).map(|entity| {
                    CompInputEvent::EntitySetPosition(EntitySetPositionEvent {
                        x: event.x,
                        y: event.y,
                        entity: *entity,
                    })
                })
            }
            DTIFInputEvent::EntityDeleted(event) => {
                sid_to_entity.get(&event.entity).map(|entity| {
                    CompInputEvent::EntityDeleted(EntityDeletedEvent { entity: *entity })
                })
            }
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct CompositionResizedEvent {
    pub size: Size,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct CompositionViewportChangedEvent {
    pub viewport: Viewport,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct EntityMovedEvent {
    pub entity: String,
    pub dx: f32,
    pub dy: f32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct EntitySetPositionEvent {
    pub entity: String,
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct EntityDeletedEvent {
    pub entity: String,
}
