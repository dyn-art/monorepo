use bevy_ecs::prelude::*;
use glam::{Mat3, Vec2};
use serde::Serialize;

#[derive(Component, Serialize, Clone, Default, Debug)]
pub struct NodeMixin;

#[derive(Component, Serialize, Clone, Debug)]
pub struct ShapeMixin;

#[derive(Component, Serialize, Clone, Debug)]
pub struct FrameMixin {
    clip_content: bool,
}

impl Default for FrameMixin {
    fn default() -> Self {
        Self {
            clip_content: false,
        }
    }
}

#[derive(Component, Serialize, Clone, Default, Debug)]
pub struct GroupMixin;

#[derive(Component, Serialize, Clone, Default, Debug)]
pub struct RectangleMixin;

// =============================================================================
// Mixins
// =============================================================================

#[derive(Component, Serialize, Clone, Debug)]
pub struct RectangleCornerMixin {
    pub top_left_radius: i16,
    pub top_right_radius: i16,
    pub bottom_right_radius: i16,
    pub bottom_left_radius: i16,
}

impl Default for RectangleCornerMixin {
    fn default() -> Self {
        Self {
            top_left_radius: 0,
            top_right_radius: 0,
            bottom_right_radius: 0,
            bottom_left_radius: 0,
        }
    }
}

#[derive(Component, Serialize, Clone, Debug)]
pub struct ChildrenMixin {
    pub children: Vec<Entity>,
}

impl Default for ChildrenMixin {
    fn default() -> Self {
        Self { children: vec![] }
    }
}

#[derive(Component, Serialize, Clone, Debug)]
pub struct LayoutMixin {
    pub width: usize,
    pub height: usize,
    pub relative_transform: TransformMatrix,
}

impl Default for LayoutMixin {
    fn default() -> Self {
        Self {
            width: 100,
            height: 100,
            relative_transform: TransformMatrix::default(),
        }
    }
}

#[derive(Component, Serialize, Clone, Debug)]
pub struct CompositionMixin {
    is_visible: bool,
    is_locked: bool,
}

impl Default for CompositionMixin {
    fn default() -> Self {
        Self {
            is_visible: true,
            is_locked: false,
        }
    }
}

#[derive(Component, Serialize, Clone, Debug)]
pub struct BlendMixin {
    blend_mode: BlendMode,
    opacity: u8,
    is_mask: bool,
}

impl Default for BlendMixin {
    fn default() -> Self {
        Self {
            blend_mode: BlendMode::Normal,
            opacity: 255,
            is_mask: false,
        }
    }
}

#[derive(Component, Serialize, Clone, Debug)]
pub struct PathMixin {
    verticies: Vec<Point>,
}

// =============================================================================
// Effects
// =============================================================================

#[derive(Debug, Clone, Serialize)]
pub enum BlendMode {
    PassThrough,
    Normal,
    Darken,
    Multiply,
    LinearBurn,
    ColorBurn,
    Lighten,
    Screen,
    LinearDodge,
    ColorDodge,
    Overlay,
    SoftLight,
    HardLight,
    Difference,
    Exclusion,
    Hue,
    Saturation,
    Color,
    Luminosity,
}

// =============================================================================
// Base
// =============================================================================

#[derive(Debug, Clone, Default)]
pub struct TransformMatrix(pub Mat3);

impl Serialize for TransformMatrix {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeTuple;

        let mut seq = serializer.serialize_tuple(6)?;
        seq.serialize_element(&self.0.x_axis.x)?;
        seq.serialize_element(&self.0.x_axis.y)?;
        seq.serialize_element(&self.0.x_axis.z)?;
        seq.serialize_element(&self.0.y_axis.x)?;
        seq.serialize_element(&self.0.y_axis.y)?;
        seq.serialize_element(&self.0.y_axis.z)?;
        seq.end()
    }
}

#[derive(Debug, Clone, Default)]
pub struct Point(pub Vec2);

impl Serialize for Point {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeTuple;

        let mut seq = serializer.serialize_tuple(2)?;
        seq.serialize_element(&self.0.x)?;
        seq.serialize_element(&self.0.y)?;
        seq.end()
    }
}
