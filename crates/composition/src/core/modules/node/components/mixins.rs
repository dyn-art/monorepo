use bevy_ecs::prelude::*;
use glam::{Mat3, Vec2};
use serde::{Deserialize, Serialize};
use specta::Type;

// =============================================================================
// Rectangle Corner Mixin
// =============================================================================

/// Provides corner radius properties for rectangle like nodes.
#[derive(Component, Serialize, Deserialize, Clone, Debug, Type)]
pub struct RectangleCornerMixin {
    /// The radius in pixels for rounding the top left corner of the node.
    /// This value determines how curved the top left corner will be.
    #[serde(rename = "topLeftRadius")]
    pub top_left_radius: u8,

    /// The radius in pixels for rounding the top right corner of the node.
    /// This value influences the curvature of the top right corner.
    #[serde(rename = "topRightRadius")]
    pub top_right_radius: u8,

    /// The radius in pixels for rounding the bottom right corner of the node.
    /// Adjusts the curve of the bottom right corner.
    #[serde(rename = "bottomRightRadius")]
    pub bottom_right_radius: u8,

    /// The radius in pixels for rounding the bottom left corner of the node.
    /// Modifies the roundness of the bottom left corner.
    #[serde(rename = "bottomLeftRadius")]
    pub bottom_left_radius: u8,
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

// =============================================================================
// Children Mixin
// =============================================================================

/// Manages the child entities of a node in a hierarchical structure.
///
/// The children are sorted back-to-front,
/// meaning the first child in the vector is the bottommost layer in the scene,
/// and the last child is the topmost layer.
#[derive(Component, Serialize, Deserialize, Clone, Debug, Type)]
pub struct ChildrenMixin(pub Vec<Entity>);

impl Default for ChildrenMixin {
    fn default() -> Self {
        Self(vec![])
    }
}

// =============================================================================
// Dimension Mixin
// =============================================================================

/// Represents the dimensional properties of a node, specifically its width and height.
#[derive(Component, Serialize, Deserialize, Clone, Debug, Type)]
pub struct DimensionMixin {
    /// The width of the node, measured in units (likely pixels).
    /// This field specifies the horizontal dimension of the node.
    pub width: u32,

    /// The height of the node, measured in units (likely pixels).
    /// This field determines the vertical dimension of the node.
    pub height: u32,
}

impl Default for DimensionMixin {
    fn default() -> Self {
        Self {
            width: 100,
            height: 100,
        }
    }
}

// =============================================================================
// Relative Transform Mixin
// =============================================================================

/// Represents the relative position and orientation of a node within its parent's coordinate system.
/// This mixin stores the transformation as a 3x3 matrix (Mat3),
/// which includes translation, rotation, and skew.
///
/// Note: This transformation does not include scaling.
/// For scaling, refer to the `DimensionMixin`.
#[derive(Component, Serialize, Deserialize, Clone, Debug, Type)]
pub struct RelativeTransformMixin(pub Mat3);

impl Default for RelativeTransformMixin {
    fn default() -> Self {
        Self(Mat3::default())
    }
}

// =============================================================================
// Node Composition Mixin
// =============================================================================

/// Contains properties related to the composition settings of a node.
#[derive(Component, Serialize, Deserialize, Clone, Debug, Type)]
pub struct NodeCompositionMixin {
    /// Determines the visibility of the node.
    #[serde(rename = "isVisible")]
    pub is_visible: bool,

    /// Indicates whether the node is locked or not.
    /// A locked node restricts certain user interactions,
    /// such as selecting or dragging on the canvas.
    #[serde(rename = "isLocked")]
    pub is_locked: bool,
}

impl Default for NodeCompositionMixin {
    fn default() -> Self {
        Self {
            is_visible: true,
            is_locked: false,
        }
    }
}

// =============================================================================
// Blend Mixin
// =============================================================================

/// Defines blending properties for a node.
#[derive(Component, Serialize, Deserialize, Clone, Debug, Type)]
pub struct BlendMixin {
    /// Specifies the blend mode for the node.
    /// Blend mode determines how the node's color blends with colors beneath it,
    /// affecting the overall visual output.
    #[serde(rename = "blendMode")]
    pub blend_mode: BlendMode,

    /// The opacity level of the node, ranging from 0.0 to 1.0.
    /// A value of 0.0 makes the node fully transparent, while 1.0 makes it fully opaque.
    pub opacity: f32,

    /// Indicates whether the node is used as a mask.
    /// If set to `true`, the node acts as a mask, controlling the visibility of content underneath it.
    #[serde(rename = "isMask")]
    pub is_mask: bool,
}

impl Default for BlendMixin {
    fn default() -> Self {
        Self {
            blend_mode: BlendMode::Normal,
            opacity: 1.0,
            is_mask: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub enum BlendMode {
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

// =============================================================================
// Path Mixin
// =============================================================================

#[derive(Component, Serialize, Deserialize, Clone, Debug, Type)]
pub struct PathMixin {
    pub vertices: Vec<Anchor>,
}

impl Default for PathMixin {
    fn default() -> Self {
        Self { vertices: vec![] }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
pub struct Anchor {
    pub position: Vec2,
    pub command: AnchorCommand,
}

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
pub enum AnchorCommand {
    MoveTo,
    LineTo,
    CurveTo {
        #[serde(rename = "controlPoint1")]
        control_point_1: Vec2,

        #[serde(rename = "controlPoint2")]
        control_point_2: Vec2,
    },
    ArcTo {
        radius: Vec2,

        #[serde(rename = "xAxisRotation")]
        x_axis_rotation: f32,

        #[serde(rename = "largeArcFlag")]
        large_arc_flag: bool,

        #[serde(rename = "sweepFlag")]
        sweep_flag: bool,
    },
    ClosePath,
}

// =============================================================================
// Effects
// =============================================================================
