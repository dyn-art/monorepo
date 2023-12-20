use bevy_ecs::component::Component;
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
    pub node_type: NodeType,

    /// The name of the node.
    /// This is an optional field and can be used to label the node with a descriptive name,
    /// such as 'Cool Node'.
    /// If not provided, it defaults to `None`.
    pub name: Option<String>,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            node_type: NodeType::None,
            name: None,
        }
    }
}

#[derive(Default, Serialize, Deserialize, Clone, Debug, Type)]
pub enum NodeType {
    #[default]
    None,
    Group,
    Rectangle,
    Frame,
    Text,
}

// =============================================================================
// Frame
// =============================================================================

/// Acts as a container used to define a layout hierarchy.
/// It functions similarly to an HTML `<div>` element.
/// This is distinct from a `GroupNode`, which is more akin to a folder for layers in its use and functionality.
#[derive(Component, Serialize, Deserialize, Clone, Debug, Type)]
pub struct Frame {
    /// Indicates whether the frame clips its content to its bounding box.
    /// When set to `true`, content that extends beyond the frame's boundaries will be clipped.
    /// When `false`, content can extend beyond the frame's boundaries without being clipped.
    #[serde(rename = "clipContent")]
    clip_content: bool,
}

impl Default for Frame {
    fn default() -> Self {
        Self {
            clip_content: false,
        }
    }
}

// =============================================================================
// Group
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
pub struct Group;

// =============================================================================
// Rectangle
// =============================================================================

/// Represents a basic shape node for a rectangle.
/// It is a fundamental building block used to create and manipulate rectangular shapes
/// within the composition.
#[derive(Component, Serialize, Deserialize, Clone, Default, Debug, Type)]
pub struct Rectangle;

// =============================================================================
// Ellipse
// =============================================================================

/// Represents a basic shape node for an ellipse.
/// Note that a circle is a special case of an ellipse where the width equals the height.
#[derive(Component, Serialize, Deserialize, Clone, Default, Debug, Type)]
pub struct Ellipse {
    /// Contains the arc data for the ellipse,
    /// which includes the starting angle, ending angle, and the inner radius ratio.
    /// These properties are used to create arcs and donuts shapes.
    #[serde(rename = "arcData")]
    pub arc_data: EllipseArcData,
}

/// Represents the arc data for an ellipse.
/// This includes properties for defining the sweep of the ellipse and its inner radius,
/// which are used in UI elements to create various elliptical shapes.
#[derive(Default, Serialize, Deserialize, Clone, Debug, Type)]
pub struct EllipseArcData {
    /// The starting angle of the ellipse's arc.
    #[serde(rename = "startingAngle")]
    pub starting_angle: f32,

    /// The ending angle of the ellipse's arc.
    #[serde(rename = "endingAngle")]
    pub ending_angle: f32,

    /// The ratio of the inner radius to the outer radius of the ellipse.
    /// A value of 0 indicates a full ellipse, while higher values create a 'donut' shape.
    #[serde(rename = "innerRadiusRatio")]
    pub inner_radius_ratio: f32,
}

// =============================================================================
// Star
// =============================================================================

/// Represents a basic shape node for a star with a set number of points.
#[derive(Component, Serialize, Deserialize, Clone, Default, Debug, Type)]
pub struct Star {
    /// The number of "spikes", or outer points of the star.
    /// This value must be an integer greater than or equal to 3.
    #[serde(rename = "pointCount")]
    pub point_count: u8,

    /// The ratio of the inner radius to the outer radius of the star.
    /// This value is used to define the sharpness of the star's points.
    #[serde(rename = "innerRadiusRatio")]
    pub inner_radius_ratio: f32,
}

// =============================================================================
// Polygon
// =============================================================================

/// Represents a basic shape node for a regular convex polygon with three or more sides.
#[derive(Component, Serialize, Deserialize, Clone, Default, Debug, Type)]
pub struct Polygon {
    /// The number of sides of the polygon.
    /// This value must be an integer greater than or equal to 3.
    #[serde(rename = "pointCount")]
    pub point_count: u8,
}

// =============================================================================
// Text
// =============================================================================

/// Represents a text node with customizable style and layout properties.
#[derive(Component, Serialize, Deserialize, Clone, Default, Debug, Type)]
pub struct Text {
    /// Sections of the text, each with its own style.
    pub segments: Vec<TextSegment>,

    /// Horizontal alignment of the text within its container.
    #[serde(default)]
    #[serde(rename = "horizontalTextAlignment")]
    pub horizontal_text_alignment: HorizontalTextAlignment,

    /// Vertical alignment of the text within its container.
    #[serde(default)]
    #[serde(rename = "verticalTextAlignment")]
    pub vertical_text_alignment: VerticalTextAlignment,

    /// Behavior of text line breaking at the bounds of its container.
    #[serde(default)]
    #[serde(rename = "linebreakBehaviour")]
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
#[derive(Serialize, Deserialize, Clone, Default, Debug, Type)]
pub struct TextStyle {
    /// Height of rasterized glyphs in pixels, influenced by window scale.
    #[serde(rename = "fontSize")]
    pub font_size: u32,

    /// Primary font identifier.
    #[serde(rename = "fontId")]
    pub font_id: u64,

    /// Spacing between characters.
    #[serde(default)]
    #[serde(rename = "letterSpacing")]
    pub letter_spacing: LetterSpacing,

    /// Vertical spacing between lines of text.
    #[serde(default)]
    #[serde(rename = "lineHeight")]
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
