use crate::models::app_error::{AppError, ErrorCode};
use dyn_attributed_string::{
    dyn_fonts_book::font::{
        info::{FontFamily, FontInfo},
        variant::{FontStretch, FontStyle, FontVariant, FontWeight},
    },
    layout::{HorizontalTextAlignment, LineWrap, TextSizingMode, VerticalTextAlignment},
};
use dyn_comp_asset::asset::{Asset, AssetContent, AssetContentType};
use dyn_comp_bundles::{
    components::{
        mixins::{
            AbsoluteLayoutElement, BlendMode, Constraint, Constraints, LayoutElement,
            LayoutElementSizingMode, LayoutParentSizingMode, StaticLayoutElement,
            StaticLayoutParent, WindingRule,
        },
        paints::{GradientColorStop, GradientVariant, ImageScaleMode},
    },
    events::{
        CoreInputEvent, CreateAssetInputEvent, CreateNodeInputEvent, CreatePaintInputEvent,
        DeleteEntityInputEvent, ExecuteLuaScriptInputEvent, FocusRootNodesInputEvent,
        MoveEntityInputEvent, RegisterLuaScriptInputEvent, UpdateCompositionSizeInputEvent,
        UpdateCompositionViewportInputEvent, UpdateDropShadowStyleInputEvent,
        UpdateEllipseNodeInputEvent, UpdateEntityBlendModeInputEvent,
        UpdateEntityChildrenInputEvent, UpdateEntityCornerRadiiInputEvent,
        UpdateEntityOpacityInputEvent, UpdateEntityRotationInputEvent, UpdateEntitySizeInputEvent,
        UpdateEntityTransformInputEvent, UpdateEntityVisibilityInputEvent,
        UpdateFillStyleInputEvent, UpdateFrameNodeInputEvent, UpdateGradientPaintInputEvent,
        UpdateImagePaintInputEvent, UpdatePolygonNodeInputEvent, UpdateSolidPaintInputEvent,
        UpdateStarNodeInputEvent, UpdateStorkeStyleInputEvent, UpdateTextNodeInputEvent,
    },
    properties::{
        AlignContent, AlignItems, AlignSelf, CompVersion, FlexDirection, JustifyContent,
        JustifyItems, JustifySelf, TextAttributeInterval, TextAttributes, Viewport,
    },
    reference_id::{ReferenceId, ReferenceIdOrEntity, ReferenceIdOrImageId},
    AssetWithId, DropShadowStyle, EllipseNode, FillStyle, FrameNode, GradientPaint, ImagePaint,
    LuaScriptWithId, Node, Paint, PolygonNode, RectangleNode, SolidPaint, StarNode, StrokeStyle,
    Style, TextNode, VectorNode,
};
use dyn_comp_dtif::DtifComposition;
use dyn_utils::{
    properties::{color::Color, corner_radii::CornerRadii, opacity::Opacity},
    units::{
        abs::Abs, angle::Angle, auto_length::AutoLength, em::Em, font_unit::FontUnit,
        length::Length, ratio::Ratio, scalar::Scalar,
    },
};
use utoipa::{OpenApi, ToSchema};

// TODO:
// Write Specta extension for OpenApi because Utoipa misses some relevant parts:
// - No support for type aliases: https://github.com/juhaku/utoipa?tab=readme-ov-file#how-to-use-rusts-type-aliases
// - No support for auto discovering types: https://github.com/juhaku/utoipa?tab=readme-ov-file#auto-discover-for-openapi-schemas-and-paths
// - No support for glam, bevy, ..
// - No support for inline imports
//
// Specta experiment:
// https://github.com/oscartbeaumont/specta/blob/main/src/lang/openapi.rs
#[derive(OpenApi)]
#[openapi(
    info(
        title = "Composition Render API",
        description = "todo",
        contact(name = "dyn.art", url = "https://dyn.art/?source=apidocs"),
        version = "1.0.0"
    ),
    paths(crate::routes::v1::render::controller::render_composition),
    components(
        // App Error
        schemas(AppError),
        schemas(ErrorCode),

       // dyn_comp_dtif

        schemas(DtifComposition),

        // dyn_comp_bundles

        // src/lib.rs
        schemas(Node),
        schemas(FrameNode),
        schemas(RectangleNode),
        schemas(EllipseNode),
        schemas(StarNode),
        schemas(PolygonNode),
        schemas(TextNode),
        schemas(VectorNode),
        schemas(Paint),
        schemas(SolidPaint),
        schemas(ImagePaint),
        schemas(GradientPaint),
        schemas(Style),
        schemas(FillStyle),
        schemas(StrokeStyle),
        schemas(DropShadowStyle),
        schemas(AssetWithId),
        schemas(LuaScriptWithId),

        // src/properties.rs
        schemas(CompVersion),
        schemas(Viewport),
        schemas(TextAttributeInterval),
        schemas(TextAttributes),
        schemas(AlignItems),
        schemas(JustifyItems),
        schemas(AlignSelf),
        schemas(JustifySelf),
        schemas(AlignContent),
        schemas(JustifyContent),
        schemas(FlexDirection),

        // src/reference_id.rs
        schemas(ReferenceId),
        schemas(ReferenceIdOrEntity),
        schemas(ReferenceIdOrImageId),

        // src/events.rs
        schemas(CoreInputEvent),
        schemas(UpdateCompositionSizeInputEvent),
        schemas(UpdateCompositionViewportInputEvent),
        schemas(FocusRootNodesInputEvent),
        schemas(CreateNodeInputEvent),
        schemas(UpdateFrameNodeInputEvent),
        schemas(UpdateEllipseNodeInputEvent),
        schemas(UpdateStarNodeInputEvent),
        schemas(UpdatePolygonNodeInputEvent),
        schemas(UpdateTextNodeInputEvent),
        schemas(UpdateFillStyleInputEvent),
        schemas(UpdateStorkeStyleInputEvent),
        schemas(UpdateDropShadowStyleInputEvent),
        schemas(CreatePaintInputEvent),
        schemas(UpdateSolidPaintInputEvent),
        schemas(UpdateImagePaintInputEvent),
        schemas(UpdateGradientPaintInputEvent),
        schemas(CreateAssetInputEvent),
        schemas(DeleteEntityInputEvent),
        schemas(UpdateEntityTransformInputEvent),
        schemas(UpdateEntitySizeInputEvent),
        schemas(MoveEntityInputEvent),
        schemas(UpdateEntityRotationInputEvent),
        schemas(UpdateEntityVisibilityInputEvent),
        schemas(UpdateEntityCornerRadiiInputEvent),
        schemas(UpdateEntityBlendModeInputEvent),
        schemas(UpdateEntityOpacityInputEvent),
        schemas(UpdateEntityChildrenInputEvent),
        schemas(RegisterLuaScriptInputEvent),
        schemas(ExecuteLuaScriptInputEvent),

        // src/components/mixins.rs
        schemas(BlendMode),
        schemas(WindingRule),
        schemas(StaticLayoutParent),
        schemas(LayoutParentSizingMode),
        schemas(AbsoluteLayoutElement),
        schemas(Constraints),
        schemas(Constraint),
        schemas(StaticLayoutElement),
        schemas(LayoutElementSizingMode),
        schemas(LayoutElement),

        // src/components/paints.rs
        schemas(ImageScaleMode),
        schemas(GradientVariant),
        schemas(GradientColorStop),

        // dyn_comp_asset

         schemas(Asset),
         schemas(AssetContent),
         schemas(AssetContentType),

         // dyn_utils

         schemas(Color),
         schemas(CornerRadii),
         schemas(Opacity),
         schemas(Abs),
         schemas(Angle),
         schemas(AutoLength),
         schemas(Em),
         schemas(FontUnit),
         schemas(Length),
         schemas(Ratio),
         schemas(Scalar),

         // dyn_attributed_string

         schemas(LineWrap),
         schemas(HorizontalTextAlignment),
         schemas(VerticalTextAlignment),
         schemas(TextSizingMode),

        // dyn_fonts_book

        schemas(FontInfo),
        schemas(FontFamily),
        schemas(FontVariant),
        schemas(FontStyle),
        schemas(FontWeight),
        schemas(FontStretch),

        // Hard coded
        schemas(ImageId),
        schemas(Vec2),
        schemas(Size),
        schemas(Mat3),
        schemas(Entity),
        schemas(Axes),
        schemas(Rect),
    )
)]
pub struct ApiDocs;

impl ApiDocs {
    pub fn generate() -> String {
        ApiDocs::openapi().to_yaml().unwrap()
    }
}

#[derive(ToSchema)]
#[allow(dead_code)]
struct ImageId {
    idx: u32,
    version: u32,
}

#[derive(ToSchema)]
#[allow(dead_code)]
struct Vec2(f32, f32);

#[derive(ToSchema)]
#[allow(dead_code)]
struct Size(f32, f32);

#[derive(ToSchema)]
#[allow(dead_code)]
struct Mat3(f32, f32, f32, f32, f32, f32, f32, f32, f32);

#[derive(ToSchema)]
#[allow(dead_code)]
struct Entity(f32);

#[derive(ToSchema)]
#[allow(dead_code)]
struct Axes {
    x: f32,
    y: f32,
}

#[derive(ToSchema)]
#[allow(dead_code)]
struct Rect {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
}
