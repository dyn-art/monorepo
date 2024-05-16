use crate::{
    components::mixins::BlendMode,
    properties::{TextAttributeInterval, Viewport},
};
use bevy_ecs::{entity::Entity, event::Event, world::World};
use dyn_attributed_string::layout::{
    HorizontalTextAlignment, LineWrap, TextSizingMode, VerticalTextAlignment,
};
use dyn_utils::{
    properties::{corner_radii::CornerRadii, opacity::Opacity, size::Size},
    units::angle::Angle,
};
use std::fmt::Debug;

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
    // UpdateFrameNode
    // UpdateEllipseNode
    // UpdateStarNode
    // UpdatePolygonNode
    UpdateTextNode(UpdateTextNodeInputEvent),

    // Paint
    // UpdateSolidPaint
    // UpdateImagePaint
    // UpdateGradientPaint

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

            // Node
            CompCoreInputEvent::FocusRootNodes(event) => {
                world.send_event(event);
            }

            // Entity
            CompCoreInputEvent::DeleteEntity(event) => {
                world.send_event(event);
            }
            CompCoreInputEvent::MoveEntity(event) => {
                world.send_event(event);
            }
            CompCoreInputEvent::UpdateEntityTransform(event) => {
                world.send_event(event);
            }
            CompCoreInputEvent::UpdateEntitySize(event) => {
                world.send_event(event);
            }
            CompCoreInputEvent::UpdateEntityRotation(event) => {
                world.send_event(event);
            }
            CompCoreInputEvent::UpdateTextNode(event) => {
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
