use bevy_ecs::entity::Entity;
use dyn_comp_types::{
    events::{
        CompInputEvent, CompositionResizedEvent, CompositionViewportChangedEvent,
        EntityDeletedEvent, EntityMovedEvent, EntitySetPositionEvent,
    },
    shared::{Size, Viewport},
};
use std::collections::HashMap;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(tag = "type")]
pub enum DtifInputEvent {
    CompositionResized(DtifCompositionResizedEvent),
    CompositionViewportChanged(DtifCompositionViewportChangedEvent),
    EntityMoved(DtifEntityMovedEvent),
    EntitySetPosition(DtifEntitySetPositionEvent),
    EntityDeleted(DtifEntityDeletedEvent),
}

impl DtifInputEvent {
    pub fn into_comp_input_event(
        self,
        sid_to_entity: &HashMap<String, Entity>,
    ) -> Option<CompInputEvent> {
        match self {
            DtifInputEvent::CompositionResized(event) => Some(CompInputEvent::CompositionResized(
                CompositionResizedEvent { size: event.size },
            )),
            DtifInputEvent::CompositionViewportChanged(event) => Some(
                CompInputEvent::CompositionViewportChanged(CompositionViewportChangedEvent {
                    viewport: event.viewport,
                }),
            ),
            DtifInputEvent::EntityMoved(event) => sid_to_entity.get(&event.entity).map(|entity| {
                CompInputEvent::EntityMoved(EntityMovedEvent {
                    dx: event.dx,
                    dy: event.dy,
                    entity: *entity,
                })
            }),
            DtifInputEvent::EntitySetPosition(event) => {
                sid_to_entity.get(&event.entity).map(|entity| {
                    CompInputEvent::EntitySetPosition(EntitySetPositionEvent {
                        x: event.x,
                        y: event.y,
                        entity: *entity,
                    })
                })
            }
            DtifInputEvent::EntityDeleted(event) => {
                sid_to_entity.get(&event.entity).map(|entity| {
                    CompInputEvent::EntityDeleted(EntityDeletedEvent { entity: *entity })
                })
            }
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct DtifCompositionResizedEvent {
    pub size: Size,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct DtifCompositionViewportChangedEvent {
    pub viewport: Viewport,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct DtifEntityMovedEvent {
    pub entity: String,
    pub dx: f32,
    pub dy: f32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct DtifEntitySetPositionEvent {
    pub entity: String,
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct DtifEntityDeletedEvent {
    pub entity: String,
}
