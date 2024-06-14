pub mod components;
pub mod events;
pub mod mapper;
pub mod properties;
pub mod reference_id;
pub mod utils;

use crate::components::{
    mixins::{CornerRadiiMixin, ImageAssetMixin, PathMixin, SizeMixin},
    nodes::{
        CnvNode, EllipseCnvNode, FrameCnvNode, PolygonCnvNode, RectangleCnvNode, StarCnvNode,
        TextCnvNode, VectorCnvNode,
    },
    paints::{CnvPaint, GradientCnvPaint, ImageCnvPaint, SolidCnvPaint},
    styles::{DropShadowCnvStyle, StrokeCnvStyle},
};
use bevy_ecs::{
    bundle::Bundle,
    system::{Commands, EntityCommands},
};
use bevy_transform::{components::Transform, TransformBundle};
use components::{
    mixins::{
        AbsoluteLayoutElementMixin, BlendMode, BlendModeMixin, LayoutElement, OpacityMixin,
        StaticLayoutElementMixin, StaticLayoutParent, StaticLayoutParentMixin, VisibilityMixin,
        WindingRule,
    },
    nodes::{CnvNodeVariant, EllipseArcData},
    paints::{CnvPaintVariant, GradientColorStop, GradientVariant, ImageScaleMode},
    styles::{CnvStyle, CnvStyleVariant, FillCnvStyle},
};
use dyn_attributed_string::layout::{
    HorizontalTextAlignment, LineWrap, TextSizingMode, VerticalTextAlignment,
};
use dyn_cnv_asset::{
    asset::{Asset, AssetContent, AssetContentType},
    asset_id::ImageId,
};
use dyn_utils::{
    properties::{color::Color, corner_radii::CornerRadii, opacity::Opacity, size::Size},
    serde::{default_as_false, default_as_true},
    units::{abs::Abs, angle::Angle},
};
use glam::{Vec2, Vec3};
use mapper::string_to_tiny_skia_path;
use properties::TextAttributeInterval;
use reference_id::{ReferenceId, ReferenceIdOrEntity, ReferenceIdOrImageId};

// =============================================================================
// Node
// =============================================================================

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "specta_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(tag = "type")
)]
#[cfg_attr(feature = "utoipa_support", derive(utoipa::ToSchema))]
pub enum Node {
    Frame(FrameNode),
    Rectangle(RectangleNode),
    Ellipse(EllipseNode),
    Star(StarNode),
    Polygon(PolygonNode),
    Text(TextNode),
    Vector(VectorNode),
}

#[derive(Bundle, Debug)]
pub struct FrameCnvNodeBundle {
    pub node: CnvNode,
    pub frame: FrameCnvNode,
    pub transform: TransformBundle,
    pub size: SizeMixin,
    pub corner_radii: CornerRadiiMixin,
    pub visibility: VisibilityMixin,
    pub blend_mode: BlendModeMixin,
    pub opacity: OpacityMixin,
}

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "specta_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(feature = "utoipa_support", derive(utoipa::ToSchema))]
pub struct FrameNode {
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub id: Option<ReferenceId>,
    #[cfg_attr(feature = "specta_support", serde(default = "default_as_false"))]
    pub clip_content: bool,
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub layout_parent: Option<StaticLayoutParent>,
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub corner_radii: CornerRadii,
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub translation: Vec2,
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub rotation_deg: Angle,
    pub size: Size,
    #[cfg_attr(feature = "specta_support", serde(default = "default_as_true"))]
    pub visible: bool,
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub blend_mode: BlendMode,
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub opacity: Opacity,
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub layout_element: LayoutElement,
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub styles: Vec<Style>,
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub children: Vec<ReferenceIdOrEntity>,
}

impl FrameNode {
    pub fn to_bundle(&self) -> FrameCnvNodeBundle {
        FrameCnvNodeBundle {
            node: CnvNode {
                variant: CnvNodeVariant::Frame,
            },
            frame: FrameCnvNode {
                clip_content: self.clip_content,
            },
            transform: TransformBundle::from_transform(Transform {
                translation: self.translation.extend(0.0),
                rotation: self.rotation_deg.to_quat(),
                scale: Vec3::ONE,
            }),
            size: SizeMixin(self.size),
            corner_radii: CornerRadiiMixin(self.corner_radii),
            visibility: VisibilityMixin(self.visible),
            blend_mode: BlendModeMixin(self.blend_mode),
            opacity: OpacityMixin(self.opacity),
        }
    }

    pub fn spawn<'a>(&self, commands: &'a mut Commands) -> EntityCommands<'a> {
        let mut entity_commands = commands.spawn(self.to_bundle());

        if let Some(layout_parent) = self.layout_parent {
            entity_commands.insert(StaticLayoutParentMixin(layout_parent));
        }

        match self.layout_element {
            LayoutElement::Absolute(layout_element) => {
                entity_commands.insert(AbsoluteLayoutElementMixin(layout_element))
            }
            LayoutElement::Static(layout_element) => {
                entity_commands.insert(StaticLayoutElementMixin(layout_element))
            }
        };

        return entity_commands;
    }
}

#[derive(Bundle, Debug)]
pub struct RectangleCnvNodeBundle {
    pub node: CnvNode,
    pub rectangle: RectangleCnvNode,
    pub transform: TransformBundle,
    pub size: SizeMixin,
    pub corner_radii: CornerRadiiMixin,
    pub visibility: VisibilityMixin,
    pub blend_mode: BlendModeMixin,
    pub opacity: OpacityMixin,
}

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "specta_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(feature = "utoipa_support", derive(utoipa::ToSchema))]
pub struct RectangleNode {
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub id: Option<ReferenceId>,
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub corner_radii: CornerRadii,
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub translation: Vec2,
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub rotation_deg: Angle,
    pub size: Size,
    #[cfg_attr(feature = "specta_support", serde(default = "default_as_true"))]
    pub visible: bool,
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub blend_mode: BlendMode,
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub opacity: Opacity,
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub layout_element: LayoutElement,
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub styles: Vec<Style>,
}

impl RectangleNode {
    pub fn to_bundle(&self) -> RectangleCnvNodeBundle {
        RectangleCnvNodeBundle {
            node: CnvNode {
                variant: CnvNodeVariant::Rectangle,
            },
            rectangle: RectangleCnvNode::default(),
            transform: TransformBundle::from_transform(Transform {
                translation: self.translation.extend(0.0),
                rotation: self.rotation_deg.to_quat(),
                scale: Vec3::ONE,
            }),
            size: SizeMixin(self.size),
            corner_radii: CornerRadiiMixin(self.corner_radii),
            visibility: VisibilityMixin(self.visible),
            blend_mode: BlendModeMixin(self.blend_mode),
            opacity: OpacityMixin(self.opacity),
        }
    }

    pub fn spawn<'a>(&self, commands: &'a mut Commands) -> EntityCommands<'a> {
        let mut entity_commands = commands.spawn(self.to_bundle());

        match self.layout_element {
            LayoutElement::Absolute(layout_element) => {
                entity_commands.insert(AbsoluteLayoutElementMixin(layout_element))
            }
            LayoutElement::Static(layout_element) => {
                entity_commands.insert(StaticLayoutElementMixin(layout_element))
            }
        };

        return entity_commands;
    }
}

#[derive(Bundle, Debug)]
pub struct EllipseCnvNodeBundle {
    pub node: CnvNode,
    pub ellipse: EllipseCnvNode,
    pub transform: TransformBundle,
    pub size: SizeMixin,
    pub visibility: VisibilityMixin,
    pub blend_mode: BlendModeMixin,
    pub opacity: OpacityMixin,
}

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "specta_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(feature = "utoipa_support", derive(utoipa::ToSchema))]
pub struct EllipseNode {
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub id: Option<ReferenceId>,
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub starting_angle: f32,
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub ending_angle: f32,
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub inner_radius_ratio: f32,
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub translation: Vec2,
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub rotation_deg: Angle,
    pub size: Size,
    #[cfg_attr(feature = "specta_support", serde(default = "default_as_true"))]
    pub visible: bool,
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub blend_mode: BlendMode,
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub opacity: Opacity,
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub layout_element: LayoutElement,
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub styles: Vec<Style>,
}

impl EllipseNode {
    pub fn to_bundle(&self) -> EllipseCnvNodeBundle {
        EllipseCnvNodeBundle {
            node: CnvNode {
                variant: CnvNodeVariant::Ellipse,
            },
            ellipse: EllipseCnvNode {
                arc_data: EllipseArcData {
                    starting_angle: self.starting_angle,
                    ending_angle: self.ending_angle,
                    inner_radius_ratio: self.inner_radius_ratio,
                },
            },
            transform: TransformBundle::from_transform(Transform {
                translation: self.translation.extend(0.0),
                rotation: self.rotation_deg.to_quat(),
                scale: Vec3::ONE,
            }),
            size: SizeMixin(self.size),
            visibility: VisibilityMixin(self.visible),
            blend_mode: BlendModeMixin(self.blend_mode),
            opacity: OpacityMixin(self.opacity),
        }
    }

    pub fn spawn<'a>(&self, commands: &'a mut Commands) -> EntityCommands<'a> {
        let mut entity_commands = commands.spawn(self.to_bundle());

        match self.layout_element {
            LayoutElement::Absolute(layout_element) => {
                entity_commands.insert(AbsoluteLayoutElementMixin(layout_element))
            }
            LayoutElement::Static(layout_element) => {
                entity_commands.insert(StaticLayoutElementMixin(layout_element))
            }
        };

        return entity_commands;
    }
}

#[derive(Bundle, Debug)]
pub struct StarCnvNodeBundle {
    pub node: CnvNode,
    pub star: StarCnvNode,
    pub transform: TransformBundle,
    pub size: SizeMixin,
    pub visibility: VisibilityMixin,
    pub blend_mode: BlendModeMixin,
    pub opacity: OpacityMixin,
}

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "specta_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(feature = "utoipa_support", derive(utoipa::ToSchema))]
pub struct StarNode {
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub id: Option<ReferenceId>,
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub inner_radius_ratio: f32,
    #[cfg_attr(
        feature = "specta_support",
        serde(default = "default_star_point_count")
    )]
    pub point_count: u8,
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub translation: Vec2,
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub rotation_deg: Angle,
    pub size: Size,
    #[cfg_attr(feature = "specta_support", serde(default = "default_as_true"))]
    pub visible: bool,
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub blend_mode: BlendMode,
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub opacity: Opacity,
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub layout_element: LayoutElement,
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub styles: Vec<Style>,
}

impl StarNode {
    pub fn to_bundle(&self) -> StarCnvNodeBundle {
        StarCnvNodeBundle {
            node: CnvNode {
                variant: CnvNodeVariant::Star,
            },
            star: StarCnvNode {
                inner_radius_ratio: self.inner_radius_ratio,
                point_count: self.point_count,
            },
            transform: TransformBundle::from_transform(Transform {
                translation: self.translation.extend(0.0),
                rotation: self.rotation_deg.to_quat(),
                scale: Vec3::ONE,
            }),
            size: SizeMixin(self.size),
            visibility: VisibilityMixin(self.visible),
            blend_mode: BlendModeMixin(self.blend_mode),
            opacity: OpacityMixin(self.opacity),
        }
    }

    pub fn spawn<'a>(&self, commands: &'a mut Commands) -> EntityCommands<'a> {
        let mut entity_commands = commands.spawn(self.to_bundle());

        match self.layout_element {
            LayoutElement::Absolute(layout_element) => {
                entity_commands.insert(AbsoluteLayoutElementMixin(layout_element))
            }
            LayoutElement::Static(layout_element) => {
                entity_commands.insert(StaticLayoutElementMixin(layout_element))
            }
        };

        return entity_commands;
    }
}

#[inline]
fn default_star_point_count() -> u8 {
    5
}

#[derive(Bundle, Debug)]
pub struct PolygonCnvNodeBundle {
    pub node: CnvNode,
    pub polygon: PolygonCnvNode,
    pub transform: TransformBundle,
    pub size: SizeMixin,
    pub visibility: VisibilityMixin,
    pub blend_mode: BlendModeMixin,
    pub opacity: OpacityMixin,
}

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "specta_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(feature = "utoipa_support", derive(utoipa::ToSchema))]
pub struct PolygonNode {
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub id: Option<ReferenceId>,
    #[cfg_attr(
        feature = "specta_support",
        serde(default = "default_polygon_point_count")
    )]
    pub point_count: u8,
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub translation: Vec2,
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub rotation_deg: Angle,
    pub size: Size,
    #[cfg_attr(feature = "specta_support", serde(default = "default_as_true"))]
    pub visible: bool,
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub blend_mode: BlendMode,
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub opacity: Opacity,
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub layout_element: LayoutElement,
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub styles: Vec<Style>,
}

impl PolygonNode {
    pub fn to_bundle(&self) -> PolygonCnvNodeBundle {
        PolygonCnvNodeBundle {
            node: CnvNode {
                variant: CnvNodeVariant::Polygon,
            },
            polygon: PolygonCnvNode {
                point_count: self.point_count,
            },
            transform: TransformBundle::from_transform(Transform {
                translation: self.translation.extend(0.0),
                rotation: self.rotation_deg.to_quat(),
                scale: Vec3::ONE,
            }),
            size: SizeMixin(self.size),
            visibility: VisibilityMixin(self.visible),
            blend_mode: BlendModeMixin(self.blend_mode),
            opacity: OpacityMixin(self.opacity),
        }
    }

    pub fn spawn<'a>(&self, commands: &'a mut Commands) -> EntityCommands<'a> {
        let mut entity_commands = commands.spawn(self.to_bundle());

        match self.layout_element {
            LayoutElement::Absolute(layout_element) => {
                entity_commands.insert(AbsoluteLayoutElementMixin(layout_element))
            }
            LayoutElement::Static(layout_element) => {
                entity_commands.insert(StaticLayoutElementMixin(layout_element))
            }
        };

        return entity_commands;
    }
}

#[inline]
fn default_polygon_point_count() -> u8 {
    3
}

#[derive(Bundle, Debug)]
pub struct TextCnvNodeBundle {
    pub node: CnvNode,
    pub text: TextCnvNode,
    pub transform: TransformBundle,
    pub size: SizeMixin,
    pub visibility: VisibilityMixin,
    pub blend_mode: BlendModeMixin,
    pub opacity: OpacityMixin,
}

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "specta_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(feature = "utoipa_support", derive(utoipa::ToSchema))]
pub struct TextNode {
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub id: Option<ReferenceId>,
    pub text: String,
    pub attributes: Vec<TextAttributeInterval>,
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub line_wrap: LineWrap,
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub horizontal_text_alignment: HorizontalTextAlignment,
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub vertical_text_alignment: VerticalTextAlignment,
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub sizing_mode: TextSizingMode,
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub translation: Vec2,
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub rotation_deg: Angle,
    pub size: Size,
    #[cfg_attr(feature = "specta_support", serde(default = "default_as_true"))]
    pub visible: bool,
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub blend_mode: BlendMode,
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub opacity: Opacity,
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub layout_element: LayoutElement,
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub styles: Vec<Style>,
}

impl TextNode {
    pub fn to_bundle(&self) -> TextCnvNodeBundle {
        TextCnvNodeBundle {
            node: CnvNode {
                variant: CnvNodeVariant::Text,
            },
            text: TextCnvNode {
                text: self.text.clone(),
                attributes: self.attributes.iter().cloned().collect(),
                line_wrap: self.line_wrap,
                horizontal_text_alignment: self.horizontal_text_alignment,
                vertical_text_alignment: self.vertical_text_alignment,
                sizing_mode: self.sizing_mode,
            },
            transform: TransformBundle::from_transform(Transform {
                translation: self.translation.extend(0.0),
                rotation: self.rotation_deg.to_quat(),
                scale: Vec3::ONE,
            }),
            size: SizeMixin(self.size),
            visibility: VisibilityMixin(self.visible),
            blend_mode: BlendModeMixin(self.blend_mode),
            opacity: OpacityMixin(self.opacity),
        }
    }

    pub fn spawn<'a>(&self, commands: &'a mut Commands) -> EntityCommands<'a> {
        let mut entity_commands = commands.spawn(self.to_bundle());

        match self.layout_element {
            LayoutElement::Absolute(layout_element) => {
                entity_commands.insert(AbsoluteLayoutElementMixin(layout_element))
            }
            LayoutElement::Static(layout_element) => {
                entity_commands.insert(StaticLayoutElementMixin(layout_element))
            }
        };

        return entity_commands;
    }
}

#[derive(Bundle, Debug)]
pub struct VectorNodeBundle {
    pub node: CnvNode,
    pub vector: VectorCnvNode,
    pub transform: TransformBundle,
    pub size: SizeMixin,
    pub visibility: VisibilityMixin,
    pub blend_mode: BlendModeMixin,
    pub opacity: OpacityMixin,
    pub path: PathMixin,
}

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "specta_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(feature = "utoipa_support", derive(utoipa::ToSchema))]
pub struct VectorNode {
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub id: Option<ReferenceId>,
    pub path: String,
    pub winding_rule: WindingRule,
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub translation: Vec2,
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub rotation_deg: Angle,
    pub size: Size,
    #[cfg_attr(feature = "specta_support", serde(default = "default_as_true"))]
    pub visible: bool,
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub blend_mode: BlendMode,
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub opacity: Opacity,
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub layout_element: LayoutElement,
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub styles: Vec<Style>,
}

impl VectorNode {
    pub fn to_bundle(&self) -> VectorNodeBundle {
        VectorNodeBundle {
            node: CnvNode {
                variant: CnvNodeVariant::Vector,
            },
            path: PathMixin {
                path: string_to_tiny_skia_path(&self.path).unwrap(),
                winding_rule: self.winding_rule,
            },
            vector: VectorCnvNode,
            transform: TransformBundle::from_transform(Transform {
                translation: self.translation.extend(0.0),
                rotation: self.rotation_deg.to_quat(),
                scale: Vec3::ONE,
            }),
            size: SizeMixin(self.size),
            visibility: VisibilityMixin(self.visible),
            blend_mode: BlendModeMixin(self.blend_mode),
            opacity: OpacityMixin(self.opacity),
        }
    }

    pub fn spawn<'a>(&self, commands: &'a mut Commands) -> EntityCommands<'a> {
        let mut entity_commands = commands.spawn(self.to_bundle());

        match self.layout_element {
            LayoutElement::Absolute(layout_element) => {
                entity_commands.insert(AbsoluteLayoutElementMixin(layout_element))
            }
            LayoutElement::Static(layout_element) => {
                entity_commands.insert(StaticLayoutElementMixin(layout_element))
            }
        };

        return entity_commands;
    }
}

// =============================================================================
// Paint
// =============================================================================

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "specta_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(tag = "type")
)]
#[cfg_attr(feature = "utoipa_support", derive(utoipa::ToSchema))]
pub enum Paint {
    Solid(SolidPaint),
    Image(ImagePaint),
    Gradient(GradientPaint),
}

#[derive(Bundle, Debug)]
pub struct SolidPaintBundle {
    pub paint: CnvPaint,
    pub solid: SolidCnvPaint,
}

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "specta_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
#[cfg_attr(feature = "utoipa_support", derive(utoipa::ToSchema))]
pub struct SolidPaint {
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub id: Option<ReferenceId>,
    pub color: Color,
}

impl SolidPaint {
    pub fn to_bundle(&self) -> SolidPaintBundle {
        SolidPaintBundle {
            paint: CnvPaint {
                variant: CnvPaintVariant::Solid,
            },
            solid: SolidCnvPaint { color: self.color },
        }
    }

    pub fn spawn<'a>(&self, commands: &'a mut Commands) -> EntityCommands<'a> {
        commands.spawn(self.to_bundle())
    }
}

#[derive(Bundle, Debug)]
pub struct ImagePaintBundle {
    pub paint: CnvPaint,
    pub image: ImageCnvPaint,
    pub asset: ImageAssetMixin,
}

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "specta_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(feature = "utoipa_support", derive(utoipa::ToSchema))]
pub struct ImagePaint {
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub id: Option<ReferenceId>,
    pub image_id: ReferenceIdOrImageId,
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub scale_mode: ImageScaleMode,
}

impl ImagePaint {
    pub fn to_bundle(&self, maybe_image_id: Option<ImageId>) -> ImagePaintBundle {
        ImagePaintBundle {
            paint: CnvPaint {
                variant: CnvPaintVariant::Image,
            },
            image: ImageCnvPaint {
                scale_mode: self.scale_mode,
            },
            asset: ImageAssetMixin(maybe_image_id),
        }
    }

    pub fn spawn<'a>(
        &self,
        commands: &'a mut Commands,
        maybe_image_id: Option<ImageId>,
    ) -> EntityCommands<'a> {
        commands.spawn(self.to_bundle(maybe_image_id))
    }
}

#[derive(Bundle, Debug)]
pub struct GradientPaintBundle {
    pub paint: CnvPaint,
    pub gradient: GradientCnvPaint,
}

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "specta_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
#[cfg_attr(feature = "utoipa_support", derive(utoipa::ToSchema))]
pub struct GradientPaint {
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub id: Option<ReferenceId>,
    pub variant: GradientVariant,
    pub stops: Vec<GradientColorStop>,
}

impl GradientPaint {
    pub fn to_bundle(&self) -> GradientPaintBundle {
        GradientPaintBundle {
            paint: CnvPaint {
                variant: CnvPaintVariant::Gradient,
            },
            gradient: GradientCnvPaint {
                variant: self.variant,
                stops: self.stops.iter().copied().collect(),
            },
        }
    }

    pub fn spawn<'a>(&self, commands: &'a mut Commands) -> EntityCommands<'a> {
        commands.spawn(self.to_bundle())
    }
}

// =============================================================================
// Style
// =============================================================================

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "specta_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(tag = "type")
)]
#[cfg_attr(feature = "utoipa_support", derive(utoipa::ToSchema))]
pub enum Style {
    Fill(FillStyle),
    Stroke(StrokeStyle),
    DropShadow(DropShadowStyle),
}

#[derive(Bundle, Debug)]
pub struct FillStyleBundle {
    pub style: CnvStyle,
    pub fill: FillCnvStyle,
    pub visibility: VisibilityMixin,
    pub blend_mode: BlendModeMixin,
    pub opacity: OpacityMixin,
}

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "specta_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(feature = "utoipa_support", derive(utoipa::ToSchema))]
pub struct FillStyle {
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub id: Option<ReferenceId>,
    pub paint_id: ReferenceIdOrEntity,
    #[cfg_attr(feature = "specta_support", serde(default = "default_as_true"))]
    pub visible: bool,
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub blend_mode: BlendMode,
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub opacity: Opacity,
}

impl FillStyle {
    pub fn to_bundle(&self) -> FillStyleBundle {
        FillStyleBundle {
            style: CnvStyle {
                variant: CnvStyleVariant::Fill,
            },
            fill: FillCnvStyle,
            visibility: VisibilityMixin(self.visible),
            blend_mode: BlendModeMixin(self.blend_mode),
            opacity: OpacityMixin(self.opacity),
        }
    }

    pub fn spawn<'a>(&self, commands: &'a mut Commands) -> EntityCommands<'a> {
        commands.spawn(self.to_bundle())
    }
}

#[derive(Bundle, Debug)]
pub struct StrokeStyleBundle {
    pub style: CnvStyle,
    pub stroke: StrokeCnvStyle,
    pub visibility: VisibilityMixin,
    pub blend_mode: BlendModeMixin,
    pub opacity: OpacityMixin,
}

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "specta_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(feature = "utoipa_support", derive(utoipa::ToSchema))]
pub struct StrokeStyle {
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub id: Option<ReferenceId>,
    pub width: Abs,
    pub paint_id: ReferenceIdOrEntity,
    #[cfg_attr(feature = "specta_support", serde(default = "default_as_true"))]
    pub visible: bool,
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub blend_mode: BlendMode,
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub opacity: Opacity,
}

impl StrokeStyle {
    pub fn to_bundle(&self) -> StrokeStyleBundle {
        StrokeStyleBundle {
            style: CnvStyle {
                variant: CnvStyleVariant::Fill,
            },
            stroke: StrokeCnvStyle {
                stroke: tiny_skia_path::Stroke {
                    width: self.width.to_pt(),
                    ..Default::default()
                },
            },
            visibility: VisibilityMixin(self.visible),
            blend_mode: BlendModeMixin(self.blend_mode),
            opacity: OpacityMixin(self.opacity),
        }
    }

    pub fn spawn<'a>(&self, commands: &'a mut Commands) -> EntityCommands<'a> {
        commands.spawn(self.to_bundle())
    }
}

#[derive(Bundle, Debug)]
pub struct DropShadowStyleBundle {
    pub style: CnvStyle,
    pub dorp_shadow: DropShadowCnvStyle,
    pub visibility: VisibilityMixin,
    pub blend_mode: BlendModeMixin,
    pub opacity: OpacityMixin,
}

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "specta_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(feature = "utoipa_support", derive(utoipa::ToSchema))]
pub struct DropShadowStyle {
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub id: Option<ReferenceId>,
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub color: Color,
    pub position: Vec2,
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub spread: Abs,
    pub blur: Abs,
    #[cfg_attr(feature = "specta_support", serde(default = "default_as_true"))]
    pub visible: bool,
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub blend_mode: BlendMode,
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub opacity: Opacity,
}

impl DropShadowStyle {
    pub fn to_bundle(&self) -> DropShadowStyleBundle {
        DropShadowStyleBundle {
            style: CnvStyle {
                variant: CnvStyleVariant::DropShadow,
            },
            dorp_shadow: DropShadowCnvStyle {
                color: self.color,
                position: self.position,
                spread: self.spread,
                blur: self.blur,
            },
            visibility: VisibilityMixin(self.visible),
            blend_mode: BlendModeMixin(self.blend_mode),
            opacity: OpacityMixin(self.opacity),
        }
    }

    pub fn spawn<'a>(&self, commands: &'a mut Commands) -> EntityCommands<'a> {
        commands.spawn(self.to_bundle())
    }
}

// =============================================================================
// Asset
// =============================================================================

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "specta_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(feature = "utoipa_support", derive(utoipa::ToSchema))]
pub struct AssetWithId {
    #[cfg_attr(feature = "specta_support", serde(default))]
    pub id: Option<ReferenceId>,
    pub content: AssetContent,
    pub content_type: AssetContentType,
}

impl AssetWithId {
    pub fn into_asset(self) -> (Option<ReferenceId>, Asset) {
        (
            self.id,
            Asset {
                content: self.content,
                content_type: self.content_type,
            },
        )
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "specta_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(feature = "utoipa_support", derive(utoipa::ToSchema))]
#[cfg(feature = "lua_scripts")]
pub struct LuaScriptWithId {
    pub id: ReferenceId,
    // TODO: Using Vec for better JSON readability, because JSON doesn't support multiline String.
    // Should we prioritize readability?
    pub source: Vec<String>,
}

#[cfg(feature = "lua_scripts")]
impl LuaScriptWithId {
    pub fn into_lua_script(self) -> (ReferenceId, dyn_cnv_lua::script::LuaScript) {
        (
            self.id,
            dyn_cnv_lua::script::LuaScript {
                source: self.source.join("\n"),
            },
        )
    }
}
