pub mod components;
pub mod events;
pub mod mapper;
pub mod properties;
pub mod reference_id;
pub mod utils;

use crate::components::{
    mixins::{CornerRadiiMixin, ImageAssetMixin, PathMixin, SizeMixin},
    nodes::{
        CompNode, EllipseCompNode, FrameCompNode, PolygonCompNode, RectangleCompNode, StarCompNode,
        TextCompNode, VectorCompNode,
    },
    paints::{CompPaint, GradientCompPaint, ImageCompPaint, SolidCompPaint},
    styles::{DropShadowCompStyle, StrokeCompStyle},
};
use bevy_ecs::{
    bundle::Bundle,
    entity::Entity,
    system::{Commands, EntityCommands},
};
use bevy_transform::{components::Transform, TransformBundle};
use components::{
    mixins::{
        AbsoluteLayoutElementMixin, BlendMode, BlendModeMixin, LayoutElement, OpacityMixin,
        PaintChildMixin, StaticLayoutElementMixin, StaticLayoutParent, StaticLayoutParentMixin,
        VisibilityMixin, WindingRule,
    },
    nodes::{CompNodeVariant, EllipseArcData},
    paints::{CompPaintVariant, GradientColorStop, GradientVariant, ImageScaleMode},
    styles::{CompStyle, CompStyleVariant, FillCompStyle},
};
use dyn_attributed_string::layout::{
    HorizontalTextAlignment, LineWrap, TextSizingMode, VerticalTextAlignment,
};
use dyn_comp_asset::{
    asset::{Asset, AssetContent, AssetContentType},
    asset_id::{AssetId, ImageId},
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
use std::collections::HashMap;

// =============================================================================
// Node
// =============================================================================

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(tag = "type")
)]
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
pub struct FrameCompNodeBundle {
    pub node: CompNode,
    pub frame: FrameCompNode,
    pub transform: TransformBundle,
    pub size: SizeMixin,
    pub corner_radii: CornerRadiiMixin,
    pub visibility: VisibilityMixin,
    pub blend_mode: BlendModeMixin,
    pub opacity: OpacityMixin,
    // https://github.com/Nilirad/bevy_prototype_lyon/issues/207
    // pub styles: StylesMixin,
    // pub children: Children,
}

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct FrameNode {
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub id: Option<ReferenceId>,
    #[cfg_attr(feature = "serde_support", serde(default = "default_as_false"))]
    pub clip_content: bool,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub layout_parent: Option<StaticLayoutParent>,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub corner_radii: CornerRadii,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub translation: Vec2,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub rotation_deg: Angle,
    pub size: Size,
    #[cfg_attr(feature = "serde_support", serde(default = "default_as_true"))]
    pub visible: bool,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub blend_mode: BlendMode,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub opacity: Opacity,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub layout_element: LayoutElement,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub styles: Vec<Style>,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub children: Vec<ReferenceIdOrEntity>,
}

impl FrameNode {
    pub fn to_bundle(&self) -> FrameCompNodeBundle {
        FrameCompNodeBundle {
            node: CompNode {
                variant: CompNodeVariant::Frame,
            },
            frame: FrameCompNode {
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
pub struct RectangleCompNodeBundle {
    pub node: CompNode,
    pub rectangle: RectangleCompNode,
    pub transform: TransformBundle,
    pub size: SizeMixin,
    pub corner_radii: CornerRadiiMixin,
    pub visibility: VisibilityMixin,
    pub blend_mode: BlendModeMixin,
    pub opacity: OpacityMixin,
    // https://github.com/Nilirad/bevy_prototype_lyon/issues/207
    // pub styles: StylesMixin,
}

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct RectangleNode {
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub id: Option<ReferenceId>,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub corner_radii: CornerRadii,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub translation: Vec2,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub rotation_deg: Angle,
    pub size: Size,
    #[cfg_attr(feature = "serde_support", serde(default = "default_as_true"))]
    pub visible: bool,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub blend_mode: BlendMode,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub opacity: Opacity,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub layout_element: LayoutElement,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub styles: Vec<Style>,
}

impl RectangleNode {
    pub fn to_bundle(&self) -> RectangleCompNodeBundle {
        RectangleCompNodeBundle {
            node: CompNode {
                variant: CompNodeVariant::Rectangle,
            },
            rectangle: RectangleCompNode::default(),
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
pub struct EllipseCompNodeBundle {
    pub node: CompNode,
    pub ellipse: EllipseCompNode,
    pub transform: TransformBundle,
    pub size: SizeMixin,
    pub visibility: VisibilityMixin,
    pub blend_mode: BlendModeMixin,
    pub opacity: OpacityMixin,
    // https://github.com/Nilirad/bevy_prototype_lyon/issues/207
    // pub styles: StylesMixin,
}

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct EllipseNode {
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub id: Option<ReferenceId>,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub starting_angle: f32,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub ending_angle: f32,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub inner_radius_ratio: f32,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub translation: Vec2,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub rotation_deg: Angle,
    pub size: Size,
    #[cfg_attr(feature = "serde_support", serde(default = "default_as_true"))]
    pub visible: bool,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub blend_mode: BlendMode,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub opacity: Opacity,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub layout_element: LayoutElement,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub styles: Vec<Style>,
}

impl EllipseNode {
    pub fn to_bundle(&self) -> EllipseCompNodeBundle {
        EllipseCompNodeBundle {
            node: CompNode {
                variant: CompNodeVariant::Ellipse,
            },
            ellipse: EllipseCompNode {
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
pub struct StarCompNodeBundle {
    pub node: CompNode,
    pub star: StarCompNode,
    pub transform: TransformBundle,
    pub size: SizeMixin,
    pub visibility: VisibilityMixin,
    pub blend_mode: BlendModeMixin,
    pub opacity: OpacityMixin,
    // https://github.com/Nilirad/bevy_prototype_lyon/issues/207
    // pub styles: StylesMixin,
}

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct StarNode {
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub id: Option<ReferenceId>,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub inner_radius_ratio: f32,
    #[cfg_attr(feature = "serde_support", serde(default = "default_star_point_count"))]
    pub point_count: u8,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub translation: Vec2,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub rotation_deg: Angle,
    pub size: Size,
    #[cfg_attr(feature = "serde_support", serde(default = "default_as_true"))]
    pub visible: bool,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub blend_mode: BlendMode,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub opacity: Opacity,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub layout_element: LayoutElement,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub styles: Vec<Style>,
}

impl StarNode {
    pub fn to_bundle(&self) -> StarCompNodeBundle {
        StarCompNodeBundle {
            node: CompNode {
                variant: CompNodeVariant::Star,
            },
            star: StarCompNode {
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
pub struct PolygonCompNodeBundle {
    pub node: CompNode,
    pub polygon: PolygonCompNode,
    pub transform: TransformBundle,
    pub size: SizeMixin,
    pub visibility: VisibilityMixin,
    pub blend_mode: BlendModeMixin,
    pub opacity: OpacityMixin,
    // https://github.com/Nilirad/bevy_prototype_lyon/issues/207
    // pub styles: StylesMixin,
}

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct PolygonNode {
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub id: Option<ReferenceId>,
    #[cfg_attr(
        feature = "serde_support",
        serde(default = "default_polygon_point_count")
    )]
    pub point_count: u8,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub translation: Vec2,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub rotation_deg: Angle,
    pub size: Size,
    #[cfg_attr(feature = "serde_support", serde(default = "default_as_true"))]
    pub visible: bool,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub blend_mode: BlendMode,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub opacity: Opacity,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub layout_element: LayoutElement,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub styles: Vec<Style>,
}

impl PolygonNode {
    pub fn to_bundle(&self) -> PolygonCompNodeBundle {
        PolygonCompNodeBundle {
            node: CompNode {
                variant: CompNodeVariant::Polygon,
            },
            polygon: PolygonCompNode {
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
pub struct TextCompNodeBundle {
    pub node: CompNode,
    pub text: TextCompNode,
    pub transform: TransformBundle,
    pub size: SizeMixin,
    pub visibility: VisibilityMixin,
    pub blend_mode: BlendModeMixin,
    pub opacity: OpacityMixin,
    // https://github.com/Nilirad/bevy_prototype_lyon/issues/207
    // pub styles: StylesMixin,
}

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct TextNode {
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub id: Option<ReferenceId>,
    pub text: String,
    pub attributes: Vec<TextAttributeInterval>,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub line_wrap: LineWrap,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub horizontal_text_alignment: HorizontalTextAlignment,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub vertical_text_alignment: VerticalTextAlignment,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub sizing_mode: TextSizingMode,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub translation: Vec2,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub rotation_deg: Angle,
    pub size: Size,
    #[cfg_attr(feature = "serde_support", serde(default = "default_as_true"))]
    pub visible: bool,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub blend_mode: BlendMode,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub opacity: Opacity,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub layout_element: LayoutElement,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub styles: Vec<Style>,
}

impl TextNode {
    pub fn to_bundle(&self) -> TextCompNodeBundle {
        TextCompNodeBundle {
            node: CompNode {
                variant: CompNodeVariant::Text,
            },
            text: TextCompNode {
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
    pub node: CompNode,
    pub vector: VectorCompNode,
    pub transform: TransformBundle,
    pub size: SizeMixin,
    pub visibility: VisibilityMixin,
    pub blend_mode: BlendModeMixin,
    pub opacity: OpacityMixin,
    pub path: PathMixin,
    // https://github.com/Nilirad/bevy_prototype_lyon/issues/207
    // pub styles: StylesMixin,
}

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct VectorNode {
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub id: Option<ReferenceId>,
    pub path: String,
    pub winding_rule: WindingRule,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub translation: Vec2,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub rotation_deg: Angle,
    pub size: Size,
    #[cfg_attr(feature = "serde_support", serde(default = "default_as_true"))]
    pub visible: bool,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub blend_mode: BlendMode,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub opacity: Opacity,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub layout_element: LayoutElement,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub styles: Vec<Style>,
}

impl VectorNode {
    pub fn to_bundle(&self) -> VectorNodeBundle {
        VectorNodeBundle {
            node: CompNode {
                variant: CompNodeVariant::Vector,
            },
            path: PathMixin {
                path: string_to_tiny_skia_path(&self.path).unwrap(),
                winding_rule: self.winding_rule,
            },
            vector: VectorCompNode,
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
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(tag = "type")
)]
pub enum Paint {
    Solid(SolidPaint),
    Image(ImagePaint),
    Gradient(GradientPaint),
}

#[derive(Bundle, Debug)]
pub struct SolidPaintBundle {
    pub paint: CompPaint,
    pub solid: SolidCompPaint,
    // https://github.com/Nilirad/bevy_prototype_lyon/issues/207
    // pub parent: PaintParentMixin,
}

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct SolidPaint {
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub id: Option<ReferenceId>,
    pub color: Color,
}

impl SolidPaint {
    pub fn to_bundle(&self) -> SolidPaintBundle {
        SolidPaintBundle {
            paint: CompPaint {
                variant: CompPaintVariant::Solid,
            },
            solid: SolidCompPaint { color: self.color },
        }
    }

    pub fn spawn<'a>(&self, commands: &'a mut Commands) -> EntityCommands<'a> {
        commands.spawn(self.to_bundle())
    }
}

#[derive(Bundle, Debug)]
pub struct ImagePaintBundle {
    pub paint: CompPaint,
    pub image: ImageCompPaint,
    pub asset: ImageAssetMixin,
    // https://github.com/Nilirad/bevy_prototype_lyon/issues/207
    // pub parent: PaintParentMixin,
}

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct ImagePaint {
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub id: Option<ReferenceId>,
    pub image_id: ReferenceIdOrImageId,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub scale_mode: ImageScaleMode,
}

impl ImagePaint {
    pub fn to_bundle(&self, maybe_image_id: Option<ImageId>) -> ImagePaintBundle {
        ImagePaintBundle {
            paint: CompPaint {
                variant: CompPaintVariant::Image,
            },
            image: ImageCompPaint {
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
    pub paint: CompPaint,
    pub gradient: GradientCompPaint,
    // https://github.com/Nilirad/bevy_prototype_lyon/issues/207
    // pub parent: PaintParentMixin,
}

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct GradientPaint {
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub id: Option<ReferenceId>,
    pub variant: GradientVariant,
    pub stops: Vec<GradientColorStop>,
}

impl GradientPaint {
    pub fn to_bundle(&self) -> GradientPaintBundle {
        GradientPaintBundle {
            paint: CompPaint {
                variant: CompPaintVariant::Gradient,
            },
            gradient: GradientCompPaint {
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
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(tag = "type")
)]
pub enum Style {
    Fill(FillStyle),
    Stroke(StrokeStyle),
    DropShadow(DropShadowStyle),
}

#[derive(Bundle, Debug)]
pub struct FillStyleBundle {
    pub style: CompStyle,
    pub fill: FillCompStyle,
    pub paint: PaintChildMixin,
    pub visibility: VisibilityMixin,
    pub blend_mode: BlendModeMixin,
    pub opacity: OpacityMixin,
    // https://github.com/Nilirad/bevy_prototype_lyon/issues/207
    // pub parent: StyleParentMixin,
}

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct FillStyle {
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub id: Option<ReferenceId>,
    pub paint_id: ReferenceIdOrEntity,
    #[cfg_attr(feature = "serde_support", serde(default = "default_as_true"))]
    pub visible: bool,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub blend_mode: BlendMode,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub opacity: Opacity,
}

impl FillStyle {
    pub fn to_bundle(&self, paint_entity: Entity) -> FillStyleBundle {
        FillStyleBundle {
            style: CompStyle {
                variant: CompStyleVariant::Fill,
            },
            fill: FillCompStyle,
            paint: PaintChildMixin(paint_entity),
            visibility: VisibilityMixin(self.visible),
            blend_mode: BlendModeMixin(self.blend_mode),
            opacity: OpacityMixin(self.opacity),
        }
    }

    pub fn spawn<'a>(
        &self,
        commands: &'a mut Commands,
        paint_entity: Entity,
    ) -> EntityCommands<'a> {
        commands.spawn(self.to_bundle(paint_entity))
    }
}

#[derive(Bundle, Debug)]
pub struct StrokeStyleBundle {
    pub style: CompStyle,
    pub stroke: StrokeCompStyle,
    pub paint: PaintChildMixin,
    pub visibility: VisibilityMixin,
    pub blend_mode: BlendModeMixin,
    pub opacity: OpacityMixin,
    // https://github.com/Nilirad/bevy_prototype_lyon/issues/207
    // pub stroke_path: StrokePathMixin,
    // pub parent: StyleParentMixin,
}

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct StrokeStyle {
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub id: Option<ReferenceId>,
    pub width: Abs,
    pub paint_id: ReferenceIdOrEntity,
    #[cfg_attr(feature = "serde_support", serde(default = "default_as_true"))]
    pub visible: bool,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub blend_mode: BlendMode,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub opacity: Opacity,
}

impl StrokeStyle {
    pub fn to_bundle(&self, paint_entity: Entity) -> StrokeStyleBundle {
        StrokeStyleBundle {
            style: CompStyle {
                variant: CompStyleVariant::Fill,
            },
            stroke: StrokeCompStyle {
                stroke: tiny_skia_path::Stroke {
                    width: self.width.to_pt(),
                    ..Default::default()
                },
            },
            paint: PaintChildMixin(paint_entity),
            visibility: VisibilityMixin(self.visible),
            blend_mode: BlendModeMixin(self.blend_mode),
            opacity: OpacityMixin(self.opacity),
        }
    }

    pub fn spawn<'a>(
        &self,
        commands: &'a mut Commands,
        paint_entity: Entity,
    ) -> EntityCommands<'a> {
        commands.spawn(self.to_bundle(paint_entity))
    }
}

#[derive(Bundle, Debug)]
pub struct DropShadowStyleBundle {
    pub style: CompStyle,
    pub dorp_shadow: DropShadowCompStyle,
    pub visibility: VisibilityMixin,
    pub blend_mode: BlendModeMixin,
    pub opacity: OpacityMixin,
    // https://github.com/Nilirad/bevy_prototype_lyon/issues/207
    // pub parent: StyleParentMixin,
}

#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct DropShadowStyle {
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub id: Option<ReferenceId>,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub color: Color,
    pub position: Vec2,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub spread: Abs,
    pub blur: Abs,
    #[cfg_attr(feature = "serde_support", serde(default = "default_as_true"))]
    pub visible: bool,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub blend_mode: BlendMode,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub opacity: Opacity,
}

impl DropShadowStyle {
    pub fn to_bundle(&self) -> DropShadowStyleBundle {
        DropShadowStyleBundle {
            style: CompStyle {
                variant: CompStyleVariant::DropShadow,
            },
            dorp_shadow: DropShadowCompStyle {
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
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct AssetWithId {
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub id: Option<ReferenceId>,
    pub content: AssetContent,
    pub content_type: AssetContentType,
}

impl AssetWithId {
    pub fn into_asset(self) -> Asset {
        Asset {
            content: self.content,
            content_type: self.content_type,
        }
    }
}
