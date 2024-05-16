use bevy_ecs::entity::Entity;
use dyn_attributed_string::layout::{
    HorizontalTextAlignment, LineWrap, TextSizingMode, VerticalTextAlignment,
};
use dyn_comp_bundles::{
    components::mixins::BlendMode,
    events::{
        CompCoreInputEvent, DeleteEntityInputEvent, FocusRootNodesInputEvent, MoveEntityInputEvent,
        UpdateCompositionSizeInputEvent, UpdateCompositionViewportInputEvent,
        UpdateEntityBlendModeInputEvent, UpdateEntityCornerRadiiInputEvent,
        UpdateEntityOpacityInputEvent, UpdateEntityRotationInputEvent, UpdateEntitySizeInputEvent,
        UpdateEntityTransformInputEvent, UpdateEntityVisibilityInputEvent,
        UpdateTextNodeInputEvent,
    },
    properties::{TextAttributeInterval, Viewport},
};
use dyn_utils::{
    properties::{corner_radii::CornerRadii, opacity::Opacity, size::Size},
    units::angle::Angle,
};
use std::collections::HashMap;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(tag = "type")]
pub enum DtifInputEvent {
    // Composition
    UpdateCompositionSize(UpdateCompositionSizeDtifInputEvent),
    UpdateCompositionViewport(UpdateCompositionViewportDtifInputEvent),
    FocusRootNodes(FocusRootNodesDtifInputEvent),

    // Node
    UpdateTextNode(UpdateTextNodeDtifInputEvent),

    // Entity
    DeleteEntity(DeleteEntityDtifInputEvent),
    UpdateEntityTransform(UpdateEntityTransformDtifInputEvent),
    UpdateEntitySize(UpdateEntitySizeDtifInputEvent),
    MoveEntity(MoveEntityDtifInputEvent),
    UpdateEntityRotation(UpdateEntityRotationDtifInputEvent),
    UpdateEntityVisibility(UpdateEntityVisibilityDtifInputEvent),
    UpdateEntityCornerRadii(UpdateEntityCornerRadiiDtifInputEvent),
    UpdateEntityBlendMode(UpdateEntityBlendModeDtifInputEvent),
    UpdateEntityOpacity(UpdateEntityOpacityDtifInputEvent),
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
            DtifInputEvent::FocusRootNodes(_) => {
                Some(CompCoreInputEvent::FocusRootNodes(FocusRootNodesInputEvent))
            }

            // Node
            DtifInputEvent::UpdateTextNode(event) => {
                sid_to_entity.get(&event.entity).map(|entity| {
                    CompCoreInputEvent::UpdateTextNode(UpdateTextNodeInputEvent {
                        entity: *entity,
                        text: event.text,
                        attributes: event.attributes,
                        line_wrap: event.line_wrap,
                        horizontal_text_alignment: event.horizontal_text_alignment,
                        vertical_text_alignment: event.vertical_text_alignment,
                        sizing_mode: event.sizing_mode,
                    })
                })
            }

            // Entity
            DtifInputEvent::DeleteEntity(event) => sid_to_entity.get(&event.entity).map(|entity| {
                CompCoreInputEvent::DeleteEntity(DeleteEntityInputEvent { entity: *entity })
            }),
            DtifInputEvent::UpdateEntityTransform(event) => {
                sid_to_entity.get(&event.entity).map(|entity| {
                    CompCoreInputEvent::UpdateEntityTransform(UpdateEntityTransformInputEvent {
                        entity: *entity,
                        x: event.x,
                        y: event.y,
                        rotation_deg: event.rotation_deg,
                    })
                })
            }
            DtifInputEvent::UpdateEntitySize(event) => {
                sid_to_entity.get(&event.entity).map(|entity| {
                    CompCoreInputEvent::UpdateEntitySize(UpdateEntitySizeInputEvent {
                        entity: *entity,
                        size: event.size,
                    })
                })
            }
            DtifInputEvent::MoveEntity(event) => sid_to_entity.get(&event.entity).map(|entity| {
                CompCoreInputEvent::MoveEntity(MoveEntityInputEvent {
                    entity: *entity,
                    dx: event.dx,
                    dy: event.dy,
                })
            }),
            DtifInputEvent::UpdateEntityRotation(event) => {
                sid_to_entity.get(&event.entity).map(|entity| {
                    CompCoreInputEvent::UpdateEntityRotation(UpdateEntityRotationInputEvent {
                        entity: *entity,
                        rotation_deg: event.rotation_deg,
                    })
                })
            }
            DtifInputEvent::UpdateEntityVisibility(event) => {
                sid_to_entity.get(&event.entity).map(|entity| {
                    CompCoreInputEvent::UpdateEntityVisibility(UpdateEntityVisibilityInputEvent {
                        entity: *entity,
                        visible: event.visible,
                    })
                })
            }
            DtifInputEvent::UpdateEntityCornerRadii(event) => {
                sid_to_entity.get(&event.entity).map(|entity| {
                    CompCoreInputEvent::UpdateEntityCornerRadii(UpdateEntityCornerRadiiInputEvent {
                        entity: *entity,
                        corner_radii: event.corner_radii,
                    })
                })
            }
            DtifInputEvent::UpdateEntityBlendMode(event) => {
                sid_to_entity.get(&event.entity).map(|entity| {
                    CompCoreInputEvent::UpdateEntityBlendMode(UpdateEntityBlendModeInputEvent {
                        entity: *entity,
                        blend_mode: event.blend_mode,
                    })
                })
            }
            DtifInputEvent::UpdateEntityOpacity(event) => {
                sid_to_entity.get(&event.entity).map(|entity| {
                    CompCoreInputEvent::UpdateEntityOpacity(UpdateEntityOpacityInputEvent {
                        entity: *entity,
                        opacity: event.opacity,
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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct FocusRootNodesDtifInputEvent;

// =============================================================================
// Node
// =============================================================================

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTextNodeDtifInputEvent {
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
    #[serde(default)]
    pub sizing_mode: Option<TextSizingMode>,
}

// =============================================================================
// Entity
// =============================================================================

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct DeleteEntityDtifInputEvent {
    pub entity: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct UpdateEntityTransformDtifInputEvent {
    pub entity: String,
    #[serde(default)]
    pub x: Option<f32>,
    #[serde(default)]
    pub y: Option<f32>,
    #[serde(default)]
    pub rotation_deg: Option<Angle>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct UpdateEntitySizeDtifInputEvent {
    pub entity: String,
    #[serde(default)]
    size: Size,
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
#[serde(rename_all = "camelCase")]
pub struct UpdateEntityRotationDtifInputEvent {
    pub entity: String,
    pub rotation_deg: Angle,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct UpdateEntityVisibilityDtifInputEvent {
    pub entity: String,
    pub visible: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct UpdateEntityCornerRadiiDtifInputEvent {
    pub entity: String,
    pub corner_radii: CornerRadii,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct UpdateEntityBlendModeDtifInputEvent {
    pub entity: String,
    pub blend_mode: BlendMode,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct UpdateEntityOpacityDtifInputEvent {
    pub entity: String,
    pub opacity: Opacity,
}
