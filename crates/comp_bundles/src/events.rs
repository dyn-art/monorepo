use crate::{
    components::{
        mixins::BlendMode,
        paints::{GradientColorStop, GradientVariant, ImageScaleMode},
    },
    properties::{TextAttributeInterval, Viewport},
};
use bevy_ecs::{entity::Entity, event::Event, world::World};
use dyn_attributed_string::layout::{
    HorizontalTextAlignment, LineWrap, TextSizingMode, VerticalTextAlignment,
};
use dyn_comp_asset::asset_id::ImageId;
use dyn_utils::{
    properties::{color::Color, corner_radii::CornerRadii, opacity::Opacity, size::Size},
    units::angle::Angle,
};

pub trait InputEvent {
    fn send_into_ecs(self, world: &mut World);
}

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(tag = "type")
)]
pub enum CompCoreInputEvent {
    // Composition
    UpdateCompositionSize(UpdateCompositionSizeInputEvent),
    UpdateCompositionViewport(UpdateCompositionViewportInputEvent),
    FocusRootNodes(FocusRootNodesInputEvent),

    // Node
    UpdateFrameNode(UpdateFrameNodeInputEvent),
    UpdateEllipseNode(UpdateEllipseNodeInputEvent),
    UpdateStarNode(UpdateStarNodeInputEvent),
    UpdatePolygonNode(UpdatePolygonNodeInputEvent),
    UpdateTextNode(UpdateTextNodeInputEvent),

    // Paint
    UpdateSolidPaint(UpdateSolidPaintInputEvent),
    UpdateImagePaint(UpdateImagePaintInputEvent),
    UpdateGradientPaint(UpdateGradientPaintInputEvent),

    // Entity
    DeleteEntity(DeleteEntityInputEvent),
    UpdateEntityTransform(UpdateEntityTransformInputEvent),
    UpdateEntitySize(UpdateEntitySizeInputEvent),
    MoveEntity(MoveEntityInputEvent),
    UpdateEntityRotation(UpdateEntityRotationInputEvent),
    UpdateEntityVisibility(UpdateEntityVisibilityInputEvent),
    UpdateEntityCornerRadii(UpdateEntityCornerRadiiInputEvent),
    UpdateEntityBlendMode(UpdateEntityBlendModeInputEvent),
    UpdateEntityOpacity(UpdateEntityOpacityInputEvent),
}

impl InputEvent for CompCoreInputEvent {
    fn send_into_ecs(self, world: &mut World) {
        match self {
            // Composition
            CompCoreInputEvent::UpdateCompositionSize(event) => {
                world.send_event(event);
            }
            CompCoreInputEvent::UpdateCompositionViewport(event) => {
                world.send_event(event);
            }
            CompCoreInputEvent::FocusRootNodes(event) => {
                world.send_event(event);
            }

            // Node
            CompCoreInputEvent::UpdateFrameNode(event) => {
                world.send_event(event);
            }
            CompCoreInputEvent::UpdateEllipseNode(event) => {
                world.send_event(event);
            }
            CompCoreInputEvent::UpdateStarNode(event) => {
                world.send_event(event);
            }
            CompCoreInputEvent::UpdatePolygonNode(event) => {
                world.send_event(event);
            }
            CompCoreInputEvent::UpdateTextNode(event) => {
                world.send_event(event);
            }

            // Paint
            CompCoreInputEvent::UpdateSolidPaint(event) => {
                world.send_event(event);
            }
            CompCoreInputEvent::UpdateImagePaint(event) => {
                world.send_event(event);
            }
            CompCoreInputEvent::UpdateGradientPaint(event) => {
                world.send_event(event);
            }

            // Entity
            CompCoreInputEvent::DeleteEntity(event) => {
                world.send_event(event);
            }
            CompCoreInputEvent::UpdateEntityTransform(event) => {
                world.send_event(event);
            }
            CompCoreInputEvent::UpdateEntitySize(event) => {
                world.send_event(event);
            }
            CompCoreInputEvent::MoveEntity(event) => {
                world.send_event(event);
            }
            CompCoreInputEvent::UpdateEntityRotation(event) => {
                world.send_event(event);
            }
            CompCoreInputEvent::UpdateEntityVisibility(event) => {
                world.send_event(event);
            }
            CompCoreInputEvent::UpdateEntityCornerRadii(event) => {
                world.send_event(event);
            }
            CompCoreInputEvent::UpdateEntityBlendMode(event) => {
                world.send_event(event);
            }
            CompCoreInputEvent::UpdateEntityOpacity(event) => {
                world.send_event(event);
            }
        }
    }
}

// =============================================================================
// Composition
// =============================================================================

#[derive(Event, Debug, Default, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct UpdateCompositionSizeInputEvent {
    pub size: Size,
}

#[derive(Event, Debug, Default, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct UpdateCompositionViewportInputEvent {
    pub viewport: Viewport,
}

#[derive(Event, Debug, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct FocusRootNodesInputEvent;

// =============================================================================
// Node
// =============================================================================

#[derive(Event, Debug, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct UpdateFrameNodeInputEvent {
    pub entity: Entity,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub clip_content: Option<bool>,
}

#[derive(Event, Debug, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct UpdateEllipseNodeInputEvent {
    pub entity: Entity,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub starting_angle: Option<f32>,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub ending_angle: Option<f32>,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub inner_radius_ratio: Option<f32>,
}

#[derive(Event, Debug, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct UpdateStarNodeInputEvent {
    pub entity: Entity,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub point_count: Option<u8>,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub inner_radius_ratio: Option<f32>,
}

#[derive(Event, Debug, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct UpdatePolygonNodeInputEvent {
    pub entity: Entity,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub point_count: Option<u8>,
}

#[derive(Event, Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct UpdateTextNodeInputEvent {
    pub entity: Entity,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub text: Option<String>,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub attributes: Option<Vec<TextAttributeInterval>>,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub line_wrap: Option<LineWrap>,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub horizontal_text_alignment: Option<HorizontalTextAlignment>,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub vertical_text_alignment: Option<VerticalTextAlignment>,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub sizing_mode: Option<TextSizingMode>,
}

// =============================================================================
// Paint
// =============================================================================

#[derive(Event, Debug, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct UpdateSolidPaintInputEvent {
    pub entity: Entity,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub color: Color,
}

#[derive(Event, Debug, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct UpdateImagePaintInputEvent {
    pub entity: Entity,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub scale_mode: Option<ImageScaleMode>,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub image_id: Option<ImageId>,
}

#[derive(Event, Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct UpdateGradientPaintInputEvent {
    pub entity: Entity,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub variant: Option<GradientVariant>,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub stops: Option<Vec<GradientColorStop>>,
}

// =============================================================================
// Entity
// =============================================================================

#[derive(Event, Debug, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct DeleteEntityInputEvent {
    pub entity: Entity,
}

#[derive(Event, Debug, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct UpdateEntityTransformInputEvent {
    pub entity: Entity,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub x: Option<f32>,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub y: Option<f32>,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub rotation_deg: Option<Angle>,
}

#[derive(Event, Debug, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct UpdateEntitySizeInputEvent {
    pub entity: Entity,
    pub size: Size,
}

#[derive(Event, Debug, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct MoveEntityInputEvent {
    pub entity: Entity,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub dx: Option<f32>,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub dy: Option<f32>,
}

#[derive(Event, Debug, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct UpdateEntityRotationInputEvent {
    pub entity: Entity,
    pub rotation_deg: Angle,
}

#[derive(Event, Debug, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct UpdateEntityVisibilityInputEvent {
    pub entity: Entity,
    pub visible: bool,
}

#[derive(Event, Debug, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct UpdateEntityCornerRadiiInputEvent {
    pub entity: Entity,
    pub corner_radii: CornerRadii,
}

#[derive(Event, Debug, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct UpdateEntityBlendModeInputEvent {
    pub entity: Entity,
    pub blend_mode: BlendMode,
}

#[derive(Event, Debug, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct UpdateEntityOpacityInputEvent {
    pub entity: Entity,
    pub opacity: Opacity,
}
