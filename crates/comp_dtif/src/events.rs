use bevy_ecs::entity::Entity;
use dyn_attributed_string::layout::{
    HorizontalTextAlignment, LineWrap, TextSizingMode, VerticalTextAlignment,
};
use dyn_comp_asset::asset_id::AssetId;
use dyn_comp_bundles::{
    components::{
        mixins::BlendMode,
        paints::{GradientColorStop, ImageScaleMode},
    },
    events::{
        CompCoreInputEvent, DeleteEntityInputEvent, FocusRootNodesInputEvent, MoveEntityInputEvent,
        UpdateCompositionSizeInputEvent, UpdateCompositionViewportInputEvent,
        UpdateEllipseNodeInputEvent, UpdateEntityBlendModeInputEvent,
        UpdateEntityCornerRadiiInputEvent, UpdateEntityOpacityInputEvent,
        UpdateEntityRotationInputEvent, UpdateEntitySizeInputEvent,
        UpdateEntityTransformInputEvent, UpdateEntityVisibilityInputEvent,
        UpdateFrameNodeInputEvent, UpdateGradientPaintInputEvent, UpdateImagePaintInputEvent,
        UpdatePolygonNodeInputEvent, UpdateSolidPaintInputEvent, UpdateStarNodeInputEvent,
        UpdateTextNodeInputEvent,
    },
    properties::{TextAttributeInterval, Viewport},
};
use dyn_utils::{
    properties::{color::Color, corner_radii::CornerRadii, opacity::Opacity, size::Size},
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
    UpdateFrameNode(UpdateFrameNodeDtifInputEvent),
    UpdateEllipseNode(UpdateEllipseNodeDtifInputEvent),
    UpdateStarNode(UpdateStarNodeDtifInputEvent),
    UpdatePolygonNode(UpdatePolygonNodeDtifInputEvent),
    UpdateTextNode(UpdateTextNodeDtifInputEvent),

    // Paint
    UpdateSolidPaint(UpdateSolidPaintDtifInputEvent),
    UpdateImagePaint(UpdateImagePaintDtifInputEvent),
    UpdateGradientPaint(UpdateGradientPaintDtifInputEvent),

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
        sid_to_asset: &HashMap<String, AssetId>,
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
            DtifInputEvent::UpdateFrameNode(event) => {
                sid_to_entity.get(&event.entity).map(|entity| {
                    CompCoreInputEvent::UpdateFrameNode(UpdateFrameNodeInputEvent {
                        entity: *entity,
                        clip_content: event.clip_content,
                    })
                })
            }
            DtifInputEvent::UpdateEllipseNode(event) => {
                sid_to_entity.get(&event.entity).map(|entity| {
                    CompCoreInputEvent::UpdateEllipseNode(UpdateEllipseNodeInputEvent {
                        entity: *entity,
                        starting_angle: event.starting_angle,
                        ending_angle: event.ending_angle,
                        inner_radius_ratio: event.inner_radius_ratio,
                    })
                })
            }
            DtifInputEvent::UpdateStarNode(event) => {
                sid_to_entity.get(&event.entity).map(|entity| {
                    CompCoreInputEvent::UpdateStarNode(UpdateStarNodeInputEvent {
                        entity: *entity,
                        point_count: event.point_count,
                        inner_radius_ratio: event.inner_radius_ratio,
                    })
                })
            }
            DtifInputEvent::UpdatePolygonNode(event) => {
                sid_to_entity.get(&event.entity).map(|entity| {
                    CompCoreInputEvent::UpdatePolygonNode(UpdatePolygonNodeInputEvent {
                        entity: *entity,
                        point_count: event.point_count,
                    })
                })
            }
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

            // Paint
            DtifInputEvent::UpdateSolidPaint(event) => {
                sid_to_entity.get(&event.entity).map(|entity| {
                    CompCoreInputEvent::UpdateSolidPaint(UpdateSolidPaintInputEvent {
                        entity: *entity,
                        color: event.color,
                    })
                })
            }
            DtifInputEvent::UpdateImagePaint(event) => {
                sid_to_entity.get(&event.entity).map(|entity| {
                    CompCoreInputEvent::UpdateImagePaint(UpdateImagePaintInputEvent {
                        entity: *entity,
                        scale_mode: event.scale_mode,
                        image_id: event.asset_id.and_then(|image_id| {
                            sid_to_asset.get(&image_id).and_then(|asset| {
                                if let AssetId::Image(id) = asset {
                                    Some(*id)
                                } else {
                                    None
                                }
                            })
                        }),
                    })
                })
            }
            DtifInputEvent::UpdateGradientPaint(event) => {
                sid_to_entity.get(&event.entity).map(|entity| {
                    CompCoreInputEvent::UpdateGradientPaint(UpdateGradientPaintInputEvent {
                        entity: *entity,
                        stops: event.stops.clone(),
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
pub struct UpdateFrameNodeDtifInputEvent {
    pub entity: String,
    #[serde(default)]
    pub clip_content: Option<bool>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct UpdateEllipseNodeDtifInputEvent {
    pub entity: String,
    #[serde(default)]
    pub starting_angle: Option<f32>,
    #[serde(default)]
    pub ending_angle: Option<f32>,
    #[serde(default)]
    pub inner_radius_ratio: Option<f32>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct UpdateStarNodeDtifInputEvent {
    pub entity: String,
    #[serde(default)]
    pub point_count: Option<u8>,
    #[serde(default)]
    pub inner_radius_ratio: Option<f32>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePolygonNodeDtifInputEvent {
    pub entity: String,
    #[serde(default)]
    pub point_count: Option<u8>,
}

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
// Paint
// =============================================================================

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct UpdateSolidPaintDtifInputEvent {
    pub entity: String,
    #[serde(default)]
    pub color: Color,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct UpdateImagePaintDtifInputEvent {
    pub entity: String,
    #[serde(default)]
    pub scale_mode: Option<ImageScaleMode>,
    #[serde(default)]
    pub asset_id: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, specta::Type)]
pub struct UpdateGradientPaintDtifInputEvent {
    pub entity: String,
    #[serde(default)]
    pub stops: Vec<GradientColorStop>,
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
