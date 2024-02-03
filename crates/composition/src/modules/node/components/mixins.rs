use bevy_ecs::prelude::*;
use glam::{Mat3, Vec2};
use serde::{Deserialize, Serialize};
use specta::Type;

// =============================================================================
// Node Meta Mixin
// =============================================================================

#[derive(Component, Serialize, Deserialize, Clone, Debug, Type)]
pub struct NodeMetaMixin {
    /// The name of the node.
    /// This is an optional field and can be used to label the node with a descriptive name,
    /// such as 'Cool Node'.
    /// If not provided, it defaults to `None`.
    #[serde(default)]
    pub name: Option<String>,
}

impl Default for NodeMetaMixin {
    fn default() -> Self {
        Self { name: None }
    }
}

// =============================================================================
// Rectangle Corner Mixin
// =============================================================================

/// Provides corner radius properties for rectangle like nodes.
#[derive(Component, Serialize, Deserialize, Clone, Debug, Type)]
#[serde(rename_all = "camelCase")]
pub struct RectangleCornerMixin {
    /// The radius in pixels for rounding the top left corner of the node.
    /// This value determines how curved the top left corner will be.
    #[serde(default = "default_corner_radius")]
    pub top_left_radius: u8,

    /// The radius in pixels for rounding the top right corner of the node.
    /// This value influences the curvature of the top right corner.
    #[serde(default = "default_corner_radius")]
    pub top_right_radius: u8,

    /// The radius in pixels for rounding the bottom right corner of the node.
    /// Adjusts the curve of the bottom right corner.
    #[serde(default = "default_corner_radius")]
    pub bottom_right_radius: u8,

    /// The radius in pixels for rounding the bottom left corner of the node.
    /// Modifies the roundness of the bottom left corner.
    #[serde(default = "default_corner_radius")]
    pub bottom_left_radius: u8,
}

impl Default for RectangleCornerMixin {
    fn default() -> Self {
        Self {
            top_left_radius: default_corner_radius(),
            top_right_radius: default_corner_radius(),
            bottom_right_radius: default_corner_radius(),
            bottom_left_radius: default_corner_radius(),
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
#[derive(Component, Serialize, Deserialize, Clone, Default, Debug, Type)]
pub struct ChildrenMixin(pub Vec<Entity>);

// =============================================================================
// Dimension Mixin
// =============================================================================

/// Represents the dimensional properties of a node, specifically its width and height.
#[derive(Component, Serialize, Deserialize, Clone, Debug, Type)]
pub struct DimensionMixin {
    /// The width of the node, measured in units (likely pixels).
    /// This field specifies the horizontal dimension of the node.
    pub width: f32,

    /// The height of the node, measured in units (likely pixels).
    /// This field determines the vertical dimension of the node.
    pub height: f32,
}

#[derive(Component, Debug)]
pub struct PreviousDimensionMixin {
    pub width: f32,
    pub height: f32,
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
#[derive(Component, Serialize, Deserialize, Clone, Default, Debug, Type)]
pub struct RelativeTransformMixin(pub Mat3);

// =============================================================================
// Absolute Transform Mixin
// =============================================================================

/// Represents the absolute position and orientation of a node within the composition's coordinate system.
/// This mixin stores the transformation as a 3x3 matrix (Mat3),
/// which includes translation, rotation, and skew.
///
/// Note: This transformation does not include scaling.
/// For scaling, refer to the `DimensionMixin`.
#[derive(Component, Serialize, Deserialize, Clone, Default, Debug, Type)]
pub struct AbsoluteTransformMixin(pub Mat3);

// =============================================================================
// Node Composition Mixin
// =============================================================================

/// Contains properties related to the composition settings of a node.
#[derive(Component, Serialize, Deserialize, Clone, Debug, Type)]
#[serde(rename_all = "camelCase")]
pub struct NodeCompositionMixin {
    /// Determines the visibility of the node.
    #[serde(default = "default_is_visible")]
    pub is_visible: bool,

    /// Indicates whether the node is locked or not.
    /// A locked node restricts certain user interactions,
    /// such as selecting or dragging on the canvas.
    #[serde(default = "default_is_locked")]
    pub is_locked: bool,
}

impl Default for NodeCompositionMixin {
    fn default() -> Self {
        Self {
            is_visible: default_is_visible(),
            is_locked: default_is_locked(),
        }
    }
}

// =============================================================================
// Blend Mixin
// =============================================================================

/// Defines blending properties for a node.
#[derive(Component, Serialize, Deserialize, Clone, Debug, Type)]
#[serde(rename_all = "camelCase")]
pub struct BlendMixin {
    /// Specifies the blend mode for the node.
    /// Blend mode determines how the node's color blends with colors beneath it.
    #[serde(default)]
    pub blend_mode: BlendMode,

    /// The opacity of the node,
    /// ranging from 0.0 (completely transparent) to 1.0 (completely opaque).
    #[serde(default = "default_opacity")]
    pub opacity: f32,

    /// Indicates whether the node is used as a mask.
    #[serde(default = "default_is_mask")]
    pub is_mask: bool,
}

impl Default for BlendMixin {
    fn default() -> Self {
        Self {
            blend_mode: BlendMode::default(),
            opacity: default_opacity(),
            is_mask: default_is_mask(),
        }
    }
}

// =============================================================================
// Path Mixin
// =============================================================================

/// Represents a path in a graphical composition, defined by a series of vertices.
/// Each vertex is an anchor point, and the path is constructed by connecting these points.
#[derive(Component, Serialize, Deserialize, Clone, Default, Debug, Type)]
pub struct PathMixin {
    /// A collection of `Anchor` points that define the shape of the path.
    /// These vertices determine the path's outline through various commands.
    pub vertices: Vec<Anchor>,
}

/// Represents an anchor point in a path, defining a specific location and command.
#[derive(Serialize, Deserialize, Clone, Debug, Type)]
pub struct Anchor {
    /// The command associated with the anchor point,
    /// defining how the path should proceed from this point.
    pub command: AnchorCommand,
}

impl Anchor {
    pub fn get_position(&self) -> Option<Vec2> {
        match self.command {
            AnchorCommand::ArcTo { position, .. }
            | AnchorCommand::CurveTo { position, .. }
            | AnchorCommand::LineTo { position }
            | AnchorCommand::MoveTo { position } => Some(position),
            _ => None,
        }
    }

    pub fn set_position(&mut self, value: Vec2) {
        match &mut self.command {
            AnchorCommand::ArcTo { position, .. }
            | AnchorCommand::CurveTo { position, .. }
            | AnchorCommand::LineTo { position }
            | AnchorCommand::MoveTo { position } => {
                *position = value;
            }
            _ => {}
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Type)]
#[serde(tag = "type")]
pub enum AnchorCommand {
    /// Moves the path to a new location without drawing anything.
    MoveTo {
        /// The position of the anchor point in 2D space.
        position: Vec2,
    },

    /// Draws a straight line from the current position to the anchor point.
    LineTo {
        /// The position of the anchor point in 2D space.
        position: Vec2,
    },

    /// Draws a curve to the anchor point using two control points.
    #[serde(rename_all = "camelCase")]
    CurveTo {
        /// The position of the anchor point in 2D space.
        position: Vec2,

        /// The first control point for the curve.
        control_point_1: Vec2,

        /// The second control point for the curve.
        control_point_2: Vec2,
    },

    /// Draws an arc to the anchor point.
    #[serde(rename_all = "camelCase")]
    ArcTo {
        /// The position of the anchor point in 2D space.
        position: Vec2,

        /// The radius of the arc in 2D space.
        radius: Vec2,

        /// The rotation of the arc's x-axis, in degrees.
        x_axis_rotation: f32,

        /// A flag to determine if the arc should be the larger of the two possible arcs.
        large_arc_flag: bool,

        /// A flag to determine the direction of the arc sweep.
        sweep_flag: bool,
    },

    /// Closes the path by drawing a line to the start point.
    ClosePath,
}

// =============================================================================
// Fill Mixin
// =============================================================================

/// Manages the fill properties of a graphical object.
#[derive(Component, Serialize, Deserialize, Clone, Default, Debug, Type)]
#[serde(rename_all = "camelCase")]
pub struct FillMixin {
    /// A collection of `Paint` objects,
    /// each defining a different aspect of how the object is filled.
    #[serde(default)]
    pub paint_ids: Vec<Entity>,
}

// =============================================================================
// Paint Composition Mixin
// =============================================================================

/// Contains properties related to the composition settings of a paint.
#[derive(Component, Serialize, Deserialize, Clone, Debug, Type)]
#[serde(rename_all = "camelCase")]
pub struct PaintCompositionMixin {
    /// Determines the visibility of the paint.
    #[serde(default = "default_is_visible")]
    pub is_visible: bool,
}

impl Default for PaintCompositionMixin {
    fn default() -> Self {
        Self {
            is_visible: default_is_visible(),
        }
    }
}

// =============================================================================
// Image Content Mixin
// =============================================================================

#[derive(Component, Serialize, Deserialize, Clone, Debug, Type)]
pub struct ImageContentMixin {
    /// The width of the image in pixels.
    pub width: f32,

    /// The height of the image in pixels.
    pub height: f32,

    /// The actual content of the image.
    pub content: ImageContent,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Clone, Debug, Type)]
#[serde(tag = "type")]
pub enum ImageContent {
    /// Image content stored as binary data.
    #[serde(rename_all = "camelCase")]
    Binary {
        content: Vec<u8>,
        content_type: ContentType,
    },

    /// Image content referenced by a URL.
    ///
    /// This variant is only supported when the `resolve-url` feature is enabled.
    #[serde(rename_all = "camelCase")]
    Url {
        url: String,
        content_type: ContentType,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash, Copy, Type)]
pub enum ContentType {
    JPEG,
    PNG,
    SVG,
}

impl ContentType {
    pub const fn mime_type(&self) -> &'static str {
        match self {
            ContentType::JPEG => "image/jpeg",
            ContentType::PNG => "image/png",
            ContentType::SVG => "image/svg+xml",
        }
    }
}

// =============================================================================
// Gradient Stops Mixin
// =============================================================================

#[derive(Component, Serialize, Deserialize, Clone, Debug, Type)]
pub struct GradientStopsMixin {
    /// A list of color stops defining the gradient.
    #[serde(rename = "gradientStops")]
    pub gradient_stops: Vec<ColorStop>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
pub struct ColorStop {
    /// The position of the color stop in the gradient, ranging from 0.0 to 1.0.
    pub position: f32,

    /// The color of the stop, represented as an RGB array.
    pub color: (u8, u8, u8),
}

// =============================================================================
// Effects
// =============================================================================

#[derive(Default, Serialize, Deserialize, Clone, Debug, Type)]
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

// =============================================================================
// Defaults
// =============================================================================

#[inline]
fn default_opacity() -> f32 {
    1.0
}

#[inline]
fn default_is_visible() -> bool {
    true
}

#[inline]
fn default_is_locked() -> bool {
    false
}

#[inline]
fn default_is_mask() -> bool {
    false
}

#[inline]
fn default_corner_radius() -> u8 {
    0
}
