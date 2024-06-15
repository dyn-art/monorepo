pub mod components;
pub mod events;
pub mod mapper;
pub mod properties;
pub mod reference_id;
pub mod utils;

use crate::components::{
    mixins::{CornerRadiiMixin, ImageAssetMixin, PathMixin, SizeMixin},
    nodes::{
        ArbNode, EllipseArbNode, FrameArbNode, PolygonArbNode, RectangleArbNode, StarArbNode,
        TextArbNode, VectorArbNode,
    },
    paints::{ArbPaint, GradientArbPaint, ImageArbPaint, SolidArbPaint},
    styles::{DropShadowArbStyle, StrokeArbStyle},
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
    nodes::{ArbNodeVariant, EllipseArcData},
    paints::{ArbPaintVariant, GradientColorStop, GradientVariant, ImageScaleMode},
    styles::{ArbStyle, ArbStyleVariant, FillArbStyle},
};
use dyn_attributed_string::layout::{
    HorizontalTextAlignment, LineWrap, TextSizingMode, VerticalTextAlignment,
};
use dyn_arb_asset::{
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
pub struct FrameArbNodeBundle {
    pub node: ArbNode,
    pub frame: FrameArbNode,
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
    pub fn to_bundle(&self) -> FrameArbNodeBundle {
        FrameArbNodeBundle {
            node: ArbNode {
                variant: ArbNodeVariant::Frame,
            },
            frame: FrameArbNode {
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
pub struct RectangleArbNodeBundle {
    pub node: ArbNode,
    pub rectangle: RectangleArbNode,
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
    pub fn to_bundle(&self) -> RectangleArbNodeBundle {
        RectangleArbNodeBundle {
            node: ArbNode {
                variant: ArbNodeVariant::Rectangle,
            },
            rectangle: RectangleArbNode::default(),
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
pub struct EllipseArbNodeBundle {
    pub node: ArbNode,
    pub ellipse: EllipseArbNode,
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
    pub fn to_bundle(&self) -> EllipseArbNodeBundle {
        EllipseArbNodeBundle {
            node: ArbNode {
                variant: ArbNodeVariant::Ellipse,
            },
            ellipse: EllipseArbNode {
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
pub struct StarArbNodeBundle {
    pub node: ArbNode,
    pub star: StarArbNode,
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
    pub fn to_bundle(&self) -> StarArbNodeBundle {
        StarArbNodeBundle {
            node: ArbNode {
                variant: ArbNodeVariant::Star,
            },
            star: StarArbNode {
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
pub struct PolygonArbNodeBundle {
    pub node: ArbNode,
    pub polygon: PolygonArbNode,
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
    pub fn to_bundle(&self) -> PolygonArbNodeBundle {
        PolygonArbNodeBundle {
            node: ArbNode {
                variant: ArbNodeVariant::Polygon,
            },
            polygon: PolygonArbNode {
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
pub struct TextArbNodeBundle {
    pub node: ArbNode,
    pub text: TextArbNode,
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
    pub fn to_bundle(&self) -> TextArbNodeBundle {
        TextArbNodeBundle {
            node: ArbNode {
                variant: ArbNodeVariant::Text,
            },
            text: TextArbNode {
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
    pub node: ArbNode,
    pub vector: VectorArbNode,
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
            node: ArbNode {
                variant: ArbNodeVariant::Vector,
            },
            path: PathMixin {
                path: string_to_tiny_skia_path(&self.path).unwrap(),
                winding_rule: self.winding_rule,
            },
            vector: VectorArbNode,
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
    pub paint: ArbPaint,
    pub solid: SolidArbPaint,
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
            paint: ArbPaint {
                variant: ArbPaintVariant::Solid,
            },
            solid: SolidArbPaint { color: self.color },
        }
    }

    pub fn spawn<'a>(&self, commands: &'a mut Commands) -> EntityCommands<'a> {
        commands.spawn(self.to_bundle())
    }
}

#[derive(Bundle, Debug)]
pub struct ImagePaintBundle {
    pub paint: ArbPaint,
    pub image: ImageArbPaint,
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
            paint: ArbPaint {
                variant: ArbPaintVariant::Image,
            },
            image: ImageArbPaint {
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
    pub paint: ArbPaint,
    pub gradient: GradientArbPaint,
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
            paint: ArbPaint {
                variant: ArbPaintVariant::Gradient,
            },
            gradient: GradientArbPaint {
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
    pub style: ArbStyle,
    pub fill: FillArbStyle,
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
            style: ArbStyle {
                variant: ArbStyleVariant::Fill,
            },
            fill: FillArbStyle,
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
    pub style: ArbStyle,
    pub stroke: StrokeArbStyle,
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
            style: ArbStyle {
                variant: ArbStyleVariant::Fill,
            },
            stroke: StrokeArbStyle {
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
    pub style: ArbStyle,
    pub dorp_shadow: DropShadowArbStyle,
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
            style: ArbStyle {
                variant: ArbStyleVariant::DropShadow,
            },
            dorp_shadow: DropShadowArbStyle {
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
    pub fn into_lua_script(self) -> (ReferenceId, dyn_arb_lua::script::LuaScript) {
        (
            self.id,
            dyn_arb_lua::script::LuaScript {
                source: self.source.join("\n"),
            },
        )
    }
}
