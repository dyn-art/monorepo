use crate::{
    components::{
        mixins::BlendMode,
        paints::{GradientColorStop, GradientVariant, ImageScaleMode},
    },
    properties::{TextAttributeInterval, Viewport},
    reference_id::ReferenceIdOrEntity,
    AssetWithId, Node, Paint,
};
use bevy_app::App;
use bevy_ecs::{entity::Entity, event::Event, world::World};
use dyn_attributed_string::layout::{
    HorizontalTextAlignment, LineWrap, TextSizingMode, VerticalTextAlignment,
};
use dyn_comp_asset::asset_id::ImageId;
use dyn_utils::{
    properties::{color::Color, corner_radii::CornerRadii, opacity::Opacity, size::Size},
    units::{abs::Abs, angle::Angle},
};
use glam::Vec2;

pub trait InputEvent {
    fn register_events(app: &mut App);
    fn send_into_world(self, world: &mut World);
}

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(tag = "type")
)]
pub enum CoreInputEvent {
    // Composition
    UpdateCompositionSize(UpdateCompositionSizeInputEvent),
    UpdateCompositionViewport(UpdateCompositionViewportInputEvent),
    FocusRootNodes(FocusRootNodesInputEvent),

    // Node
    CreateNode(CreateNodeInputEvent),
    UpdateFrameNode(UpdateFrameNodeInputEvent),
    UpdateEllipseNode(UpdateEllipseNodeInputEvent),
    UpdateStarNode(UpdateStarNodeInputEvent),
    UpdatePolygonNode(UpdatePolygonNodeInputEvent),
    UpdateTextNode(UpdateTextNodeInputEvent),

    // Style
    UpdateFillStyle(UpdateFillStyleInputEvent),
    UpdateStrokeStyle(UpdateStorkeStyleInputEvent),
    UpdateDropShadowStyle(UpdateDropShadowStyleInputEvent),

    // Paint
    CreatePaint(CreatePaintInputEvent),
    UpdateSolidPaint(UpdateSolidPaintInputEvent),
    UpdateImagePaint(UpdateImagePaintInputEvent),
    UpdateGradientPaint(UpdateGradientPaintInputEvent),

    // Asset
    CreateAsset(CreateAssetInputEvent),

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
    UpdateEntityChildren(UpdateEntityChildrenInputEvent),
}

impl InputEvent for CoreInputEvent {
    fn register_events(app: &mut App) {
        // Composition
        app.add_event::<UpdateCompositionSizeInputEvent>();
        app.add_event::<UpdateCompositionViewportInputEvent>();
        app.add_event::<FocusRootNodesInputEvent>();

        // Node
        app.add_event::<CreateNodeInputEvent>();
        app.add_event::<UpdateFrameNodeInputEvent>();
        app.add_event::<UpdateEllipseNodeInputEvent>();
        app.add_event::<UpdateStarNodeInputEvent>();
        app.add_event::<UpdatePolygonNodeInputEvent>();
        app.add_event::<UpdateTextNodeInputEvent>();

        // Style
        app.add_event::<UpdateFillStyleInputEvent>();
        app.add_event::<UpdateStorkeStyleInputEvent>();
        app.add_event::<UpdateDropShadowStyleInputEvent>();

        // Paint
        app.add_event::<CreatePaintInputEvent>();
        app.add_event::<UpdateSolidPaintInputEvent>();
        app.add_event::<UpdateGradientPaintInputEvent>();
        app.add_event::<UpdateImagePaintInputEvent>();

        // Asset
        app.add_event::<CreateAssetInputEvent>();

        // Entity
        app.add_event::<DeleteEntityInputEvent>();
        app.add_event::<UpdateEntityTransformInputEvent>();
        app.add_event::<UpdateEntitySizeInputEvent>();
        app.add_event::<MoveEntityInputEvent>();
        app.add_event::<UpdateEntityRotationInputEvent>();
        app.add_event::<UpdateEntityVisibilityInputEvent>();
        app.add_event::<UpdateEntityCornerRadiiInputEvent>();
        app.add_event::<UpdateEntityBlendModeInputEvent>();
        app.add_event::<UpdateEntityOpacityInputEvent>();
        app.add_event::<UpdateEntityChildrenInputEvent>();
    }

    fn send_into_world(self, world: &mut World) {
        match self {
            // Composition
            CoreInputEvent::UpdateCompositionSize(event) => {
                world.send_event(event);
            }
            CoreInputEvent::UpdateCompositionViewport(event) => {
                world.send_event(event);
            }
            CoreInputEvent::FocusRootNodes(event) => {
                world.send_event(event);
            }

            // Node
            CoreInputEvent::CreateNode(event) => {
                world.send_event(event);
            }
            CoreInputEvent::UpdateFrameNode(event) => {
                world.send_event(event);
            }
            CoreInputEvent::UpdateEllipseNode(event) => {
                world.send_event(event);
            }
            CoreInputEvent::UpdateStarNode(event) => {
                world.send_event(event);
            }
            CoreInputEvent::UpdatePolygonNode(event) => {
                world.send_event(event);
            }
            CoreInputEvent::UpdateTextNode(event) => {
                world.send_event(event);
            }

            // Style
            CoreInputEvent::UpdateFillStyle(event) => {
                world.send_event(event);
            }
            CoreInputEvent::UpdateStrokeStyle(event) => {
                world.send_event(event);
            }
            CoreInputEvent::UpdateDropShadowStyle(event) => {
                world.send_event(event);
            }

            // Paint
            CoreInputEvent::CreatePaint(event) => {
                world.send_event(event);
            }
            CoreInputEvent::UpdateSolidPaint(event) => {
                world.send_event(event);
            }
            CoreInputEvent::UpdateImagePaint(event) => {
                world.send_event(event);
            }
            CoreInputEvent::UpdateGradientPaint(event) => {
                world.send_event(event);
            }

            // Asset
            CoreInputEvent::CreateAsset(event) => {
                world.send_event(event);
            }

            // Entity
            CoreInputEvent::DeleteEntity(event) => {
                world.send_event(event);
            }
            CoreInputEvent::UpdateEntityTransform(event) => {
                world.send_event(event);
            }
            CoreInputEvent::UpdateEntitySize(event) => {
                world.send_event(event);
            }
            CoreInputEvent::MoveEntity(event) => {
                world.send_event(event);
            }
            CoreInputEvent::UpdateEntityRotation(event) => {
                world.send_event(event);
            }
            CoreInputEvent::UpdateEntityVisibility(event) => {
                world.send_event(event);
            }
            CoreInputEvent::UpdateEntityCornerRadii(event) => {
                world.send_event(event);
            }
            CoreInputEvent::UpdateEntityBlendMode(event) => {
                world.send_event(event);
            }
            CoreInputEvent::UpdateEntityOpacity(event) => {
                world.send_event(event);
            }
            CoreInputEvent::UpdateEntityChildren(event) => {
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
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct CreateNodeInputEvent {
    pub node: Node,
}

#[derive(Event, Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct UpdateFrameNodeInputEvent {
    pub id: ReferenceIdOrEntity,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub clip_content: Option<bool>,
}

#[derive(Event, Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct UpdateEllipseNodeInputEvent {
    pub id: ReferenceIdOrEntity,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub starting_angle: Option<f32>,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub ending_angle: Option<f32>,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub inner_radius_ratio: Option<f32>,
}

#[derive(Event, Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct UpdateStarNodeInputEvent {
    pub id: ReferenceIdOrEntity,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub point_count: Option<u8>,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub inner_radius_ratio: Option<f32>,
}

#[derive(Event, Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct UpdatePolygonNodeInputEvent {
    pub id: ReferenceIdOrEntity,
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
    pub id: ReferenceIdOrEntity,
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
// Style
// =============================================================================

// TODO: For now Styles can only be created via Node
//
// #[derive(Event, Debug, Clone)]
// #[cfg_attr(
//     feature = "serde_support",
//     derive(serde::Serialize, serde::Deserialize, specta::Type)
// )]
// pub struct CreateStyleInputEvent {
//     pub style: Style,
// }

#[derive(Event, Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct UpdateFillStyleInputEvent {
    pub id: ReferenceIdOrEntity,
    pub paint_id: Option<ReferenceIdOrEntity>,
}

#[derive(Event, Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct UpdateStorkeStyleInputEvent {
    pub id: ReferenceIdOrEntity,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub paint_id: Option<ReferenceIdOrEntity>,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub width: Option<Abs>,
}

#[derive(Event, Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct UpdateDropShadowStyleInputEvent {
    pub id: ReferenceIdOrEntity,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub color: Option<Color>,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub position: Option<Vec2>,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub spread: Option<Abs>,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub blur: Option<Abs>,
}

// =============================================================================
// Paint
// =============================================================================

#[derive(Event, Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct CreatePaintInputEvent {
    pub paint: Paint,
}

#[derive(Event, Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct UpdateSolidPaintInputEvent {
    pub id: ReferenceIdOrEntity,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub color: Color,
}

#[derive(Event, Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct UpdateImagePaintInputEvent {
    pub id: ReferenceIdOrEntity,
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
    pub id: ReferenceIdOrEntity,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub variant: Option<GradientVariant>,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub stops: Option<Vec<GradientColorStop>>,
}

// =============================================================================
// Asset
// =============================================================================

#[derive(Event, Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct CreateAssetInputEvent {
    pub asset: AssetWithId,
}

// =============================================================================
// Entity
// =============================================================================

#[derive(Event, Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct DeleteEntityInputEvent {
    pub id: ReferenceIdOrEntity,
}

#[derive(Event, Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct UpdateEntityTransformInputEvent {
    pub id: ReferenceIdOrEntity,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub x: Option<f32>,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub y: Option<f32>,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub rotation_deg: Option<Angle>,
}

#[derive(Event, Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct UpdateEntitySizeInputEvent {
    pub id: ReferenceIdOrEntity,
    pub size: Size,
}

#[derive(Event, Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct MoveEntityInputEvent {
    pub id: ReferenceIdOrEntity,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub dx: Option<f32>,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub dy: Option<f32>,
}

#[derive(Event, Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct UpdateEntityRotationInputEvent {
    pub id: ReferenceIdOrEntity,
    pub rotation_deg: Angle,
}

#[derive(Event, Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct UpdateEntityVisibilityInputEvent {
    pub id: ReferenceIdOrEntity,
    pub visible: bool,
}

#[derive(Event, Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct UpdateEntityCornerRadiiInputEvent {
    pub id: ReferenceIdOrEntity,
    pub corner_radii: CornerRadii,
}

#[derive(Event, Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct UpdateEntityBlendModeInputEvent {
    pub id: ReferenceIdOrEntity,
    pub blend_mode: BlendMode,
}

#[derive(Event, Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct UpdateEntityOpacityInputEvent {
    pub id: ReferenceIdOrEntity,
    pub opacity: Opacity,
}

#[derive(Event, Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct UpdateEntityChildrenInputEvent {
    pub id: ReferenceIdOrEntity,
    pub children: Vec<ReferenceIdOrEntity>,
}
