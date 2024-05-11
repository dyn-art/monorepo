use crate::{
    properties::{AlignItems, AlignSelf, FlexDirection, JustifyContent, JustifySelf},
    utils::{auto_length_to_taffy, length_to_taffy},
};
use bevy_ecs::{component::Component, entity::Entity};
use dyn_attributed_string::AttributedString;
use dyn_comp_asset::asset_id::ImageId;
use dyn_utils::{
    properties::{corner_radii::CornerRadii, opacity::Opacity, rect::Rect, size::Size},
    units::{auto_length::AutoLength, axes::Axes, length::Length},
};
use glam::Vec3;
use smallvec::SmallVec;

#[derive(Component, Debug, Default, Clone, Copy)]
pub struct HierarchyLevel(pub u8);

/// Represents an entity's dimensions with width and height.
#[derive(Component, Debug, Default, Copy, Clone)]
pub struct SizeMixin(pub Size);

/// Defines corner radii for rectangular entities, specifying each corner's radius.
#[derive(Component, Debug, Default, Copy, Clone)]
pub struct CornerRadiiMixin(pub CornerRadii);

/// Specifies an entity's blend mode for color blending with underlying colors.
#[derive(Component, Debug, Default, Copy, Clone)]
pub struct BlendModeMixin(pub BlendMode);

#[derive(Debug, Default, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub enum BlendMode {
    #[default]
    Normal,
    Multiply,
    Screen,
    Overlay,
    Darken,
    Lighten,
    ColorDodge,
    ColorBurn,
    HardLight,
    SoftLight,
    Difference,
    Exclusion,
    Hue,
    Saturation,
    Color,
    Luminosity,
}

/// Controls an entity's visibility state.
#[derive(Component, Debug, Copy, Clone)]
pub struct VisibilityMixin(pub bool);

impl Default for VisibilityMixin {
    fn default() -> Self {
        Self(true)
    }
}

/// Controls the opacity of an entity, ranging from 0.0 (fully transparent) to 1.0 (fully opaque).
#[derive(Component, Debug, Default, Copy, Clone)]
pub struct OpacityMixin(pub Opacity);

/// Represents a Bezier path for drawing shape.
#[derive(Component, Debug, Clone)]
pub struct PathMixin {
    pub path: tiny_skia_path::Path,
    pub winding_rule: WindingRule,
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub enum WindingRule {
    #[default]
    Nonzero,
    Evenodd,
}

/// Configures stroke properties for drawing paths.
#[derive(Component, Debug, Clone)]
pub struct StrokePathMixin {
    pub path: tiny_skia_path::Path,
    pub winding_rule: WindingRule,
}

#[derive(Component, Debug, Default, Clone)]
pub struct StyleChildrenMixin(pub SmallVec<[Entity; 2]>);

#[derive(Component, Debug, Clone)]
pub struct StyleParentMixin(pub Entity);

#[derive(Component, Debug, Clone)]
pub struct PaintChildMixin(pub Option<Entity>);

#[derive(Component, Debug, Default, Clone)]
pub struct PaintParentMixin(pub SmallVec<[Entity; 2]>);

#[derive(Component, Debug, Default, Clone)]
pub struct ImageAssetMixin(pub Option<ImageId>);

#[derive(Component, Debug, Clone)]
pub struct AttributedStringMixin(pub AttributedString);

#[derive(Component, Debug, Copy, Clone)]
pub struct StaticLayoutNodeId(pub taffy::NodeId);

#[derive(Component, Debug, Default, Copy, Clone)]
pub struct StaticLayoutParentMixin(pub StaticLayoutParent);

#[derive(Debug, Default, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct StaticLayoutParent {
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub align_items: Option<AlignItems>,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub justify_content: Option<JustifyContent>,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub gap: Axes<Length>,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub padding: Rect<Length>,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub flex_direction: FlexDirection,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub horizontal_sizing_mode: LayoutSizingMode,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub vertical_sizing_mode: LayoutSizingMode,
}

impl StaticLayoutParent {
    pub fn to_style(&self) -> taffy::Style {
        taffy::Style {
            align_items: self.align_items.map(|v| v.into()),
            justify_content: self.justify_content.map(|v| v.into()),
            gap: taffy::Size::<taffy::LengthPercentage> {
                width: length_to_taffy(self.gap.x),
                height: length_to_taffy(self.gap.y),
            },
            padding: taffy::Rect::<taffy::LengthPercentage> {
                top: length_to_taffy(self.padding.top),
                bottom: length_to_taffy(self.padding.bottom),
                left: length_to_taffy(self.padding.left),
                right: length_to_taffy(self.padding.right),
            },
            flex_direction: self.flex_direction.into(),
            ..Default::default()
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub enum LayoutSizingMode {
    #[default]
    Fixed,
    Hug,
    Fill,
}

#[derive(Component, Debug, Default, Copy, Clone)]
pub struct AbsoluteLayoutElementMixin(pub AbsoluteLayoutElement);

#[derive(Debug, Default, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct AbsoluteLayoutElement {
    pub constraints: Constraints,
}

#[derive(Debug, Default, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct Constraints {
    pub horizontal: Constraint,
    pub vertical: Constraint,
}

#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub enum Constraint {
    #[default]
    Start,
    Center,
    End,
    Stretch,
    Scale,
}

#[derive(Component, Debug, Default, Copy, Clone)]
pub struct StaticLayoutElementMixin(pub StaticLayoutElement);

#[derive(Debug, Default, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct StaticLayoutElement {
    #[cfg_attr(feature = "serde_support", serde(default))]
    align_self: Option<AlignSelf>,
    #[cfg_attr(feature = "serde_support", serde(default))]
    justify_self: Option<JustifySelf>,
    #[cfg_attr(feature = "serde_support", serde(default))]
    margin: Rect<AutoLength>,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub horizontal_sizing_mode: LayoutSizingMode,
    #[cfg_attr(feature = "serde_support", serde(default))]
    pub vertical_sizing_mode: LayoutSizingMode,
}

impl StaticLayoutElement {
    pub fn to_style(&self) -> taffy::Style {
        taffy::Style {
            align_self: self.align_self.map(|v| v.into()),
            justify_self: self.justify_self.map(|v| v.into()),
            margin: taffy::Rect::<taffy::LengthPercentageAuto> {
                top: auto_length_to_taffy(self.margin.top),
                bottom: auto_length_to_taffy(self.margin.bottom),
                left: auto_length_to_taffy(self.margin.left),
                right: auto_length_to_taffy(self.margin.right),
            },
            ..Default::default()
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(tag = "type")
)]
pub enum LayoutElement {
    Absolute(AbsoluteLayoutElement),
    Static(StaticLayoutElement),
}

impl Default for LayoutElement {
    fn default() -> Self {
        Self::Absolute(AbsoluteLayoutElement::default())
    }
}

#[derive(Component, Debug, Default, Copy, Clone)]
pub struct PreAbsoluteLayoutProperties {
    pub translation: Vec3,
    pub size: Size,
    pub parent_size: Option<Size>,
}
