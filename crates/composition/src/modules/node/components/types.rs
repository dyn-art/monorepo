use bevy_ecs::component::Component;
use glam::Mat3;
use serde::{Deserialize, Serialize};
use specta::Type;

/// Marks the root node within the composition or scene.
///
/// This component is intended to be used with only one entity in the world
/// to represent the starting point of the composition.
/// It is important to note that there is no automatic enforcement
/// at the Bevy framework level to ensure the uniqueness of this component.
/// As such, maintaining the singularity of this component must be managed
/// through game logic or specific programming measures to prevent multiple instances.
#[derive(Component, Serialize, Deserialize, Clone, Default, Debug, Type)]
pub struct Root;

// =============================================================================
// Node
// =============================================================================

/// Represents a basic node in the composition.
#[derive(Component, Serialize, Deserialize, Clone, Debug, Type)]
pub struct Node {
    /// Represents the specific type of the node, such as `Rectangle`, `Ellipse`, `Star`, etc.
    /// This field is redundant but neccessary to distinguish different nodes in the rendering process,
    /// without a big overhead like a separate system for each node type/variant.
    /// Note that the NodeType should be equivalent to the 'NodeBundle' enum
    /// and when creating a new `NodeBundle` always use the default of that specific bundle!
    pub node_type: NodeType,
}

#[derive(Serialize, Deserialize, Clone, Debug, Type, Eq, PartialEq)]
pub enum NodeType {
    Group,
    Rectangle,
    Frame,
    Text,
    Vector,
    Polygon,
    Ellipse,
    Star,
}

// =============================================================================
// Frame Node
// =============================================================================

/// Acts as a container used to define a layout hierarchy.
/// It functions similarly to an HTML `<div>` element.
/// This is distinct from a `GroupNode`, which is more akin to a folder for layers in its use and functionality.
#[derive(Component, Serialize, Deserialize, Clone, Default, Debug, Type)]
pub struct FrameNode {
    #[serde(default)]
    _frame_node: Option<()>,

    /// Indicates whether the frame clips its content to its bounding box.
    /// When set to `true`, content that extends beyond the frame's boundaries will be clipped.
    /// When `false`, content can extend beyond the frame's boundaries without being clipped.
    #[serde(default = "default_clip_content", rename = "clipContent")]
    clip_content: bool,
}

#[inline]
fn default_clip_content() -> bool {
    true
}

// =============================================================================
// Group Node
// =============================================================================

/// Serves as a container used to semantically group related nodes,
/// analogous to a folder in a layers panel.
/// This is in contrast to the `Frame` node, which is used to define layout and is
/// more akin to an HTML `<div>` element.
///
/// Groups are automatically positioned and sized to accommodate their content.
/// As a result, while it is possible to move or resize a `Group`, be aware that its
/// position and size are subject to change in response to modifications of its content.
#[derive(Component, Serialize, Deserialize, Clone, Default, Debug, Type)]
pub struct GroupNode {
    #[serde(default)]
    _group_node: Option<()>,
}

// =============================================================================
// Rectangle Node
// =============================================================================

/// Represents a basic shape node for a rectangle.
/// It is a fundamental building block used to create and manipulate rectangular shapes
/// within the composition.
#[derive(Component, Serialize, Deserialize, Clone, Default, Debug, Type)]
pub struct RectangleNode {
    #[serde(default)]
    _rectangle_node: Option<()>,
}

// =============================================================================
// Ellipse Node
// =============================================================================

/// Represents a basic shape node for an ellipse.
/// Note that a circle is a special case of an ellipse where the width equals the height.
#[derive(Component, Serialize, Deserialize, Clone, Default, Debug, Type)]
pub struct EllipseNode {
    #[serde(default)]
    _ellipse_node: Option<()>,

    /// Contains the arc data for the ellipse,
    /// which includes the starting angle, ending angle, and the inner radius ratio.
    /// These properties are used to create arcs and donuts shapes.
    #[serde(default, rename = "arcData")]
    pub arc_data: EllipseArcData,
}

/// Represents the arc data for an ellipse.
/// This includes properties for defining the sweep of the ellipse and its inner radius,
/// which are used in UI elements to create various elliptical shapes.
#[derive(Serialize, Deserialize, Clone, Default, Debug, Type)]
#[serde(rename_all = "camelCase")]
pub struct EllipseArcData {
    /// The starting angle of the ellipse's arc.
    pub starting_angle: f32,

    /// The ending angle of the ellipse's arc.
    pub ending_angle: f32,

    /// The ratio of the inner radius to the outer radius of the ellipse.
    /// A value of 0 indicates a full ellipse, while higher values create a 'donut' shape.
    pub inner_radius_ratio: f32,
}

// =============================================================================
// Star Node
// =============================================================================

/// Represents a basic shape node for a star with a set number of points.
#[derive(Component, Serialize, Deserialize, Clone, Debug, Type)]
pub struct StarNode {
    #[serde(default)]
    _star_node: Option<()>,

    /// The number of "spikes", or outer points of the star.
    /// This value must be an integer greater than or equal to 3.
    #[serde(default = "default_star_point_count", rename = "pointCount")]
    pub point_count: u8,

    /// The ratio of the inner radius to the outer radius of the star.
    /// This value is used to define the sharpness of the star's points.
    #[serde(rename = "innerRadiusRatio")]
    pub inner_radius_ratio: f32,
}

impl Default for StarNode {
    fn default() -> Self {
        Self {
            _star_node: None,
            point_count: default_star_point_count(),
            inner_radius_ratio: 0.0,
        }
    }
}

#[inline]
fn default_star_point_count() -> u8 {
    5
}

// =============================================================================
// Polygon Node
// =============================================================================

/// Represents a basic shape node for a regular convex polygon with three or more sides.
#[derive(Component, Serialize, Deserialize, Clone, Debug, Type)]
pub struct PolygonNode {
    #[serde(default)]
    _polygon_node: Option<()>,

    /// The number of sides of the polygon.
    /// This value must be an integer greater than or equal to 3.
    #[serde(default = "default_polygon_point_count", rename = "pointCount")]
    pub point_count: u8,
}

impl Default for PolygonNode {
    fn default() -> Self {
        Self {
            _polygon_node: None,
            point_count: default_polygon_point_count(),
        }
    }
}

#[inline]
fn default_polygon_point_count() -> u8 {
    3
}

// =============================================================================
// Text Node
// =============================================================================

/// Represents a text node with customizable style and layout properties.
#[derive(Component, Serialize, Deserialize, Clone, Default, Debug, Type)]
pub struct TextNode {
    #[serde(default)]
    _text_node: Option<()>,

    /// Sections of the text, each with its own style.
    pub segments: Vec<TextSegment>,

    /// Horizontal alignment of the text within its container.
    #[serde(default, rename = "horizontalTextAlignment")]
    pub horizontal_text_alignment: HorizontalTextAlignment,

    /// Vertical alignment of the text within its container.
    #[serde(default, rename = "verticalTextAlignment")]
    pub vertical_text_alignment: VerticalTextAlignment,

    /// Behavior of text line breaking at the bounds of its container.
    #[serde(default, rename = "linebreakBehavior")]
    pub linebreak_behavior: BreakLineOn,
}

/// A segment of text with a specific style.
#[derive(Serialize, Deserialize, Clone, Debug, Type)]
pub struct TextSegment {
    /// Text content of the segment.
    pub value: String,
    /// Style properties applied to this segment.
    pub style: TextStyle,
}

/// Style properties for a text segment, defining its appearance.
#[derive(Serialize, Deserialize, Clone, Debug, Type)]
#[serde(rename_all = "camelCase")]
pub struct TextStyle {
    /// Height of rasterized glyphs in pixels, influenced by window scale.
    pub font_size: u32,

    /// Primary font identifier.
    pub font_id: u64,

    /// Spacing between characters.
    #[serde(default)]
    pub letter_spacing: LetterSpacing,

    /// Vertical spacing between lines of text.
    #[serde(default)]
    pub line_height: LineHeight,
}

/// Horizontal alignment options for text within its container.
#[derive(Serialize, Deserialize, Clone, Default, Debug, Type)]
pub enum HorizontalTextAlignment {
    /// Aligns text to the left side of its container.
    #[default]
    Left,
    /// Centers text horizontally within its container.
    Center,
    /// Aligns text to the right side of its container.
    Right,
    /// Justifies text across the container width.
    Justified,
}

/// Vertical alignment options for text within its container.
#[derive(Serialize, Deserialize, Clone, Default, Debug, Type)]
pub enum VerticalTextAlignment {
    /// Aligns text to the top of its container.
    Top,
    /// Centers text vertically within its container.
    #[default]
    Center,
    /// Aligns text to the bottom of its container.
    Bottom,
}

/// Options for spacing between characters in a text segment.
#[derive(Serialize, Deserialize, Clone, Default, Debug, Type)]
pub enum LetterSpacing {
    /// Automatic spacing based on font metrics.
    #[default]
    Auto,
    /// Fixed spacing in pixels.
    Pixels(u8),
    /// Spacing as a percentage of font size.
    Percent(u8),
}

/// Options for controlling line height in text.
#[derive(Serialize, Deserialize, Clone, Default, Debug, Type)]
pub enum LineHeight {
    /// Automatic line height based on font metrics.
    #[default]
    Auto,
    /// Fixed line height in pixels.
    Pixels(u8),
    /// Line height as a percentage of font size.
    Percent(u8),
}

/// Defines how text should break lines within its container.
#[derive(Serialize, Deserialize, Clone, Default, Debug, Type)]
pub enum BreakLineOn {
    /// Breaks lines at word boundaries using the Unicode Line Breaking Algorithm.
    #[default]
    WordBoundary,
    /// Breaks lines at any character, possibly splitting words.
    AnyCharacter,
    /// Disables automatic line breaking. Respects explicit line breaks like '\n'.
    NoWrap,
}

// =============================================================================
// Vector Node
// =============================================================================

/// Represents a basic vector. It is the most general representation of a shape.
#[derive(Component, Serialize, Deserialize, Clone, Default, Debug, Type)]
pub struct VectorNode {
    #[serde(default)]
    _vector_node: Option<()>,
}

// =============================================================================
// Paint
// =============================================================================

/// Represents a basic paint in the composition.
#[derive(Component, Serialize, Deserialize, Clone, Debug, Type)]
pub struct Paint {
    /// Represents the specific type of the paint, such as `Solid`, `Image`, `Gradient`, etc.
    /// This field is redundant but neccessary to distinguish different paints in the rendering process,
    /// without a big overhead like a separate system for each paint type/variant.
    /// Note that the PaintType should be equivalent to the 'PaintBundle' enum
    /// and when creating a new `PaintBundle` always use the default of that specific bundle!
    pub paint_type: PaintType,
}

#[derive(Serialize, Deserialize, Clone, Debug, Type, Eq, PartialEq)]
pub enum PaintType {
    Solid,
    Gradient,
    Image,
}

// =============================================================================
// Solid Paint
// =============================================================================

#[derive(Component, Serialize, Deserialize, Clone, Default, Debug, Type)]
pub struct SolidPaint {
    #[serde(default)]
    _solid_paint: Option<()>,

    /// The color of the paint, represented as an RGB array
    /// where each component ranges from 0 to 255.
    pub color: (u8, u8, u8),
}

// =============================================================================
// Image Paint
// =============================================================================

#[derive(Component, Serialize, Deserialize, Clone, Default, Debug, Type)]
pub struct ImagePaint {
    #[serde(default)]
    _image_paint: Option<()>,

    /// Defines the scale mode of the image.
    #[serde(default, rename = "scaleMode")]
    pub scale_mode: ImagePaintScaleMode,
}

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
#[serde(tag = "type")]
pub enum ImagePaintScaleMode {
    /// Fills the area completely with the image.
    Fill {
        #[serde(default)]
        _image_fill_paint: Option<()>,
    },

    /// Fits the image within the area while maintaining its aspect ratio.
    Fit {
        #[serde(default)]
        _image_fit_paint: Option<()>,
    },

    /// Crops the image to fill the area.
    Crop {
        #[serde(default)]
        _image_crop_paint: Option<()>,

        #[serde(default)]
        transform: Mat3,
    },

    /// Tiles the image within the area.
    #[serde(rename_all = "camelCase")]
    Tile {
        #[serde(default)]
        _image_tile_paint: Option<()>,
        #[serde(default)]
        rotation: f32,
        scaling_factor: f32,
    },
}

impl Default for ImagePaintScaleMode {
    fn default() -> Self {
        Self::Fill {
            _image_fill_paint: None,
        }
    }
}

// =============================================================================
// Gradient Paint
// =============================================================================

#[derive(Component, Serialize, Deserialize, Clone, Default, Debug, Type)]
pub struct GradientPaint {
    #[serde(default)]
    _gradient_paint: Option<()>,

    /// Specifies the variant of the gradient.
    #[serde(default)]
    pub variant: GradientPaintVariant,
}

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
#[serde(tag = "type")]
pub enum GradientPaintVariant {
    Linear {
        #[serde(default)]
        transform: Mat3,
    },
    Radial {
        #[serde(default)]
        transform: Mat3,
    },
}

impl Default for GradientPaintVariant {
    fn default() -> Self {
        Self::Linear {
            transform: Mat3::default(),
        }
    }
}
