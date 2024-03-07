use crate::common::{BlendMode, CornerRadii, Fill, Opacity, Size, Visibility};
use bevy_ecs::{component::Component, entity::Entity};
use dyn_comp_asset::asset_id::ImageId;
use smallvec::SmallVec;
use std::collections::HashSet;

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
pub struct VisibilityMixin(pub Visibility);

/// Controls the opacity of an entity, ranging from 0.0 (fully transparent) to 1.0 (fully opaque).
#[derive(Component, Debug, Default, Copy, Clone)]
pub struct OpacityMixin(pub Opacity);

/// Represents a Bezier path for drawing shape.
#[derive(Component, Debug, Clone)]
pub struct PathMixin(pub tiny_skia_path::Path);

/// Defines fill styles for a shape.
#[derive(Component, Debug, Default, Clone)]
pub struct FillsMixin(pub SmallVec<[Fill; 2]>);

/// Represents a Bezier path for drawing stroke.
#[derive(Component, Debug, Clone)]
pub struct StrokeMixin(pub tiny_skia_path::Stroke);

/// Configures stroke properties for drawing paths.
#[derive(Component, Debug, Clone)]
pub struct StrokePathMixin(pub tiny_skia_path::Path);

/// Defines fill styles for a shape stroke.
#[derive(Component, Debug, Default, Clone)]
pub struct StrokeFillsMixin(pub SmallVec<[Fill; 2]>);

/// Holds references to the parent entities of this entity in the paint context.
#[derive(Component, Debug, Default, Clone)]
pub struct PaintParentMixin(pub HashSet<Entity>);

#[derive(Component, Debug, Default, Clone)]
pub struct ImageAssetMixin(pub Option<ImageId>);
