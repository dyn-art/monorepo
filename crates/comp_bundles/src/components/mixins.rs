use bevy_ecs::{component::Component, entity::Entity};
use dyn_attributed_string::AttributedString;
use dyn_comp_asset::asset_id::ImageId;
use dyn_utils::properties::{corner_radii::CornerRadii, opacity::Opacity, size::Size};
use smallvec::SmallVec;
use taffy::Style;

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
pub struct LayoutTreeNodeId(pub taffy::NodeId);

pub trait ToTaffyStyle {
    fn to_style(&self) -> taffy::Style;
}

#[derive(Component, Debug, Default, Copy, Clone)]
pub struct LeafLayoutMixin {
    pub align_self: AlignItems,
    pub justify_self: AlignItems,
}

impl ToTaffyStyle for LeafLayoutMixin {
    fn to_style(&self) -> taffy::Style {
        Style {
            align_self: Some(self.align_self.to_align_items()),
            justify_self: Some(self.justify_self.to_align_items()),
            ..Default::default()
        }
    }
}

#[derive(Component, Debug, Copy, Clone)]
pub struct ParentLayoutMixin {
    // TODO:
}

impl ToTaffyStyle for ParentLayoutMixin {
    fn to_style(&self) -> taffy::Style {
        Style::default()
    }
}

#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub enum AlignItems {
    /// Items are packed toward the start of the axis
    #[default]
    Start,
    /// Items are packed toward the end of the axis
    End,
    /// Items are packed towards the flex-relative start of the axis.
    ///
    /// For flex containers with flex_direction RowReverse or ColumnReverse this is equivalent
    /// to End. In all other cases it is equivalent to Start.
    FlexStart,
    /// Items are packed towards the flex-relative end of the axis.
    ///
    /// For flex containers with flex_direction RowReverse or ColumnReverse this is equivalent
    /// to Start. In all other cases it is equivalent to End.
    FlexEnd,
    /// Items are packed along the center of the cross axis
    Center,
    /// Items are aligned such as their baselines align
    Baseline,
    /// Stretch to fill the container
    Stretch,
}

impl AlignItems {
    pub fn to_align_items(&self) -> taffy::AlignItems {
        match self {
            AlignItems::Start => taffy::AlignItems::Start,
            AlignItems::End => taffy::AlignItems::End,
            AlignItems::FlexStart => taffy::AlignItems::FlexStart,
            AlignItems::FlexEnd => taffy::AlignItems::FlexEnd,
            AlignItems::Center => taffy::AlignItems::Center,
            AlignItems::Baseline => taffy::AlignItems::Baseline,
            AlignItems::Stretch => taffy::AlignItems::Stretch,
        }
    }
}
