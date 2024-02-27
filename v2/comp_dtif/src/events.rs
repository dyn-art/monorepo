use bevy_ecs::entity::Entity;
use dyn_comp_types::{
    common::{Size, Viewport},
    events::{
        CompCoreInputEvent, CompositionResizedInputEvent, CompositionViewportChangedInputEvent,
        EntityDeletedInputEvent, EntityMovedInputEvent, EntitySetPositionInputEvent,
    },
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
    pub fn to_comp_input_event(
        self,
        sid_to_entity: &HashMap<String, Entity>,
    ) -> Option<CompCoreInputEvent> {
        match self {
            DtifInputEvent::CompositionResized(event) => {
                Some(CompCoreInputEvent::CompositionResized(
                    CompositionResizedInputEvent { size: event.size },
                ))
            }
            DtifInputEvent::CompositionViewportChanged(event) => {
                Some(CompCoreInputEvent::CompositionViewportChanged(
                    CompositionViewportChangedInputEvent {
                        viewport: event.viewport,
                    },
                ))
            }
            DtifInputEvent::EntityMoved(event) => sid_to_entity.get(&event.entity).map(|entity| {
                CompCoreInputEvent::EntityMoved(EntityMovedInputEvent {
                    dx: event.dx,
                    dy: event.dy,
                    entity: *entity,
                })
            }),
            DtifInputEvent::EntitySetPosition(event) => {
                sid_to_entity.get(&event.entity).map(|entity| {
                    CompCoreInputEvent::EntitySetPosition(EntitySetPositionInputEvent {
                        x: event.x,
                        y: event.y,
                        entity: *entity,
                    })
                })
            }
            DtifInputEvent::EntityDeleted(event) => {
                sid_to_entity.get(&event.entity).map(|entity| {
                    CompCoreInputEvent::EntityDeleted(EntityDeletedInputEvent { entity: *entity })
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
