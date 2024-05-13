use bevy_ecs::entity::Entity;
use dyn_attributed_string::layout::layout_config::{
    HorizontalTextAlignment, LineWrap, VerticalTextAlignment,
};
use dyn_comp_bundles::{
    events::{
        CompCoreInputEvent, DeleteEntityInputEvent, FocusRootNodesInputEvent, MoveEntityInputEvent,
        UpdateCompositionSizeInputEvent, UpdateCompositionViewportInputEvent,
        UpdateEntityPositionInputEvent, UpdateEntityRotationInputEvent, UpdateEntityTextInputEvent,
    },
    properties::{TextAttributeInterval, Viewport},
};
use dyn_utils::{properties::size::Size, units::angle::Angle};
use std::collections::HashMap;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(tag = "type")]
pub enum DtifInputEvent {
    // Composition
    UpdateCompositionSize(UpdateCompositionSizeDtifInputEvent),
    UpdateCompositionViewport(UpdateCompositionViewportDtifInputEvent),

    // Node
    FocusRootNodes(FocusRootNodesDtifInputEvent),

    // Entity
    DeleteEntity(DeleteEntityDtifInputEvent),
    MoveEntity(MoveEntityDtifInputEvent),
    UpdateEntityPosition(UpdateEntityPositionDtifInputEvent),
    UpdateEntityRotation(UpdateEntityRotationDtifInputEvent),
    UpdateEntityText(UpdateEntityTextDtifInputEvent),
}

impl DtifInputEvent {
    pub fn to_comp_input_event(
        self,
        sid_to_entity: &HashMap<String, Entity>,
    ) -> Option<CompCoreInputEvent> {
        match self {
            // Composition
            DtifInputEvent::UpdateCompositionSize(event) => {
                Some(CompCoreInputEvent::UpdateCompositionSize(
                    UpdateCompositionSizeInputEvent { size: event.size },
                ))
            }
            DtifInputEvent::UpdateCompositionViewport(event) => {
                Some(CompCoreInputEvent::UpdateCompositionViewport(
                    UpdateCompositionViewportInputEvent {
                        viewport: event.viewport,
                    },
                ))
            }

            // Node
            DtifInputEvent::FocusRootNodes(_) => {
                Some(CompCoreInputEvent::FocusRootNodes(FocusRootNodesInputEvent))
            }

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
            DtifInputEvent::UpdateEntityPosition(event) => {
                sid_to_entity.get(&event.entity).map(|entity| {
                    CompCoreInputEvent::UpdateEntityPosition(UpdateEntityPositionInputEvent {
                        entity: *entity,
                        x: event.x,
                        y: event.y,
                    })
                })
            }
            DtifInputEvent::UpdateEntityRotation(event) => {
                sid_to_entity.get(&event.entity).map(|entity| {
                    CompCoreInputEvent::UpdateEntityRotation(UpdateEntityRotationInputEvent {
                        entity: *entity,
                        rotation_deg: event.rotation_deg,
                    })
                })
            }
            DtifInputEvent::UpdateEntityText(event) => {
                sid_to_entity.get(&event.entity).map(|entity| {
                    CompCoreInputEvent::UpdateEntityText(UpdateEntityTextInputEvent {
                        entity: *entity,
                        text: event.text,
                        attributes: event.attributes,
                        line_wrap: event.line_wrap,
                        horizontal_text_alignment: event.horizontal_text_alignment,
                        vertical_text_alignment: event.vertical_text_alignment,
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
pub struct UpdateCompositionSizeDtifInputEvent {
    pub size: Size,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct UpdateCompositionViewportDtifInputEvent {
    pub viewport: Viewport,
}

// =============================================================================
// Node
// =============================================================================

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct FocusRootNodesDtifInputEvent;

// =============================================================================
// Entity
// =============================================================================

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct DeleteEntityDtifInputEvent {
    pub entity: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct MoveEntityDtifInputEvent {
    pub entity: String,
    #[serde(default)]
    pub dx: Option<f32>,
    #[serde(default)]
    pub dy: Option<f32>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct UpdateEntityPositionDtifInputEvent {
    pub entity: String,
    #[serde(default)]
    pub x: Option<f32>,
    #[serde(default)]
    pub y: Option<f32>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct UpdateEntityRotationDtifInputEvent {
    pub entity: String,
    pub rotation_deg: Angle,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct UpdateEntityTextDtifInputEvent {
    pub entity: String,
    #[serde(default)]
    pub text: Option<String>,
    #[serde(default)]
    pub attributes: Option<Vec<TextAttributeInterval>>,
    #[serde(default)]
    pub line_wrap: Option<LineWrap>,
    #[serde(default)]
    pub horizontal_text_alignment: Option<HorizontalTextAlignment>,
    #[serde(default)]
    pub vertical_text_alignment: Option<VerticalTextAlignment>,
}
