use bevy_ecs::{component::Component, entity::Entity};
use dyn_attributed_string::AttributedString;
use dyn_comp_asset::asset_id::ImageId;
use dyn_utils::properties::{corner_radii::CornerRadii, opacity::Opacity, size::Size};
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

#[derive(Component, Debug, Default, Clone, Copy)]
pub struct ConstraintsMixin(pub Constraints);

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

#[derive(Component, Debug, Default, Clone, Copy)]
pub struct ConstraintsLayoutMetricsMixin {
    pub pos: Vec3,
    pub size: Size,
    pub parent_size: Size,
}
