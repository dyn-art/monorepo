use bevy_ecs::entity::Entity;
use dyn_comp_bundles::{
    events::{
        CompCoreInputEvent, DeleteEntityInputEvent, MoveEntityInputEvent,
        ResizeCompositionInputEvent, SetCompositionViewportInputEvent, SetEntityPositionInputEvent,
        SetEntityRotationInputEvent,
    },
    properties::Viewport,
};
use dyn_utils::{properties::size::Size, units::angle::Angle};
use std::collections::HashMap;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(tag = "type")]
pub enum DtifInputEvent {
    // Composition
    ResizeComposition(ResizeCompositionDtifInputEvent),
    SetCompositionViewport(SetCompositionViewportDtifInputEvent),

    // Entity
    DeleteEntity(DeleteEntityDtifInputEvent),
    MoveEntity(MoveEntityDtifInputEvent),
    SetEntityPosition(SetEntityPositionDtifInputEvent),
    SetEntityRotation(SetEntityRotationDtifInputEvent),
}

impl DtifInputEvent {
    pub fn to_comp_input_event(
        self,
        sid_to_entity: &HashMap<String, Entity>,
    ) -> Option<CompCoreInputEvent> {
        match self {
            // Composition
            DtifInputEvent::ResizeComposition(event) => {
                Some(CompCoreInputEvent::ResizeComposition(
                    ResizeCompositionInputEvent { size: event.size },
                ))
            }
            DtifInputEvent::SetCompositionViewport(event) => Some(
                CompCoreInputEvent::SetCompositionViewport(SetCompositionViewportInputEvent {
                    viewport: event.viewport,
                }),
            ),

            // Entity
            DtifInputEvent::DeleteEntity(event) => sid_to_entity.get(&event.entity).map(|entity| {
                CompCoreInputEvent::DeleteEntity(DeleteEntityInputEvent { entity: *entity })
            }),
            DtifInputEvent::MoveEntity(event) => sid_to_entity.get(&event.entity).map(|entity| {
                CompCoreInputEvent::MoveEntity(MoveEntityInputEvent {
                    entity: *entity,
                    dx: event.dx,
                    dy: event.dy,
                })
            }),
            DtifInputEvent::SetEntityPosition(event) => {
                sid_to_entity.get(&event.entity).map(|entity| {
                    CompCoreInputEvent::SetEntityPosition(SetEntityPositionInputEvent {
                        entity: *entity,
                        x: event.x,
                        y: event.y,
                    })
                })
            }
            DtifInputEvent::SetEntityRotation(event) => {
                sid_to_entity.get(&event.entity).map(|entity| {
                    CompCoreInputEvent::SetEntityRotation(SetEntityRotationInputEvent {
                        entity: *entity,
                        rotation_deg: event.rotation_deg,
                    })
                })
            }
        }
    }
}

// =============================================================================
// Composition
// =============================================================================

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct ResizeCompositionDtifInputEvent {
    pub size: Size,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct SetCompositionViewportDtifInputEvent {
    pub viewport: Viewport,
}

// =============================================================================
// Entity
// =============================================================================

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct MoveEntityDtifInputEvent {
    pub entity: String,
    pub dx: f32,
    pub dy: f32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct SetEntityPositionDtifInputEvent {
    pub entity: String,
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct DeleteEntityDtifInputEvent {
    pub entity: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct SetEntityRotationDtifInputEvent {
    pub entity: String,
    pub rotation_deg: Angle,
}
