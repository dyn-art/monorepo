use crate::common::{BlendMode, CornerRadii, Opacity, Size};
use bevy_ecs::{component::Component, entity::Entity};
use dyn_comp_asset::asset_id::ImageId;
use smallvec::SmallVec;

/// Marks an entity as the root or top-level entity.
#[derive(Component, Debug, Default, Copy, Clone)]
pub struct Root;

/// Represents an entity's dimensions with width and height.
#[derive(Component, Debug, Default, Copy, Clone)]
pub struct SizeMixin(pub Size);

/// Defines corner radii for rectangular entities, specifying each corner's radius.
#[derive(Component, Debug, Default, Copy, Clone)]
pub struct CornerRadiiMixin(pub CornerRadii);

/// Specifies an entity's blend mode for color blending with underlying colors.
#[derive(Component, Debug, Default, Copy, Clone)]
pub struct BlendModeMixin(pub BlendMode);

/// Controls an entity's visibility state.
#[derive(Component, Debug, Default, Copy, Clone)]
pub struct VisibilityMixin(pub bool);

/// Controls the opacity of an entity, ranging from 0.0 (fully transparent) to 1.0 (fully opaque).
#[derive(Component, Debug, Default, Copy, Clone)]
pub struct OpacityMixin(pub Opacity);

/// Represents a Bezier path for drawing shape.
#[derive(Component, Debug, Clone)]
pub struct PathMixin(pub tiny_skia_path::Path);

/// Configures stroke properties for drawing paths.
#[derive(Component, Debug, Clone)]
pub struct StrokePathMixin(pub tiny_skia_path::Path);

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
