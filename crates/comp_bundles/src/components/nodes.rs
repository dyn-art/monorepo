use bevy_ecs::component::Component;
use dyn_utils::units::abs::Abs;
use smallvec::SmallVec;

#[derive(Component, Debug, Copy, Clone)]
pub struct CompNode {
    pub variant: CompNodeVariant,
}

#[derive(Debug, Copy, Clone)]
pub enum CompNodeVariant {
    Group,
    Rectangle,
    Frame,
    Ellipse,
    Polygon,
    Star,
    Text,
    Vector,
}

/// Defines a layout container, similar to an HTML `<div>`, for hierarchical organization.
#[derive(Component, Debug, Default, Copy, Clone)]
pub struct FrameCompNode {
    /// Whether the frame clips content outside its bounds. `true` enables clipping.
    pub clip_content: bool,
}

/// Groups related nodes, akin to a layer folder, auto-sized and positioned by its content.
#[derive(Component, Debug, Default, Copy, Clone)]
pub struct GroupCompNode;

/// A rectangle shape node.
#[derive(Component, Debug, Default, Copy, Clone)]
pub struct RectangleCompNode;

/// An ellipse shape node, where a circle is a special case with equal width and height.
#[derive(Component, Debug, Default, Copy, Clone)]
pub struct EllipseCompNode {
    /// Arc data for the ellipse, defining start/end angles and inner radius ratio.
    pub arc_data: EllipseArcData,
}

/// Arc data for ellipses, including start/end angles and inner to outer radius ratio.
#[derive(Debug, Default, Copy, Clone)]
pub struct EllipseArcData {
    pub starting_angle: f32,
    pub ending_angle: f32,
    /// Ratio of inner to outer radius, with 0 being a full ellipse.
    pub inner_radius_ratio: f32,
}

/// A star shape node with customizable point count and inner to outer radius ratio.
#[derive(Component, Debug, Default, Copy, Clone)]
pub struct StarCompNode {
    /// The number of outer points. Minimum value is 3.
    pub point_count: u8,
    /// Defines sharpness of star points.
    pub inner_radius_ratio: f32,
}

/// A regular polygon shape node with three or more sides.
#[derive(Component, Debug, Default, Copy, Clone)]
pub struct PolygonCompNode {
    /// The number of polygon sides, minimum 3.
    pub point_count: u8,
}

/// A text shape node with customizable style and alignment properties.
#[derive(Component, Debug, Default, Clone)]
pub struct TextCompNode {
    /// Text spans with individual styles.
    pub spans: SmallVec<[TextSpan; 2]>,
    /// Horizontal alignment within the container.
    pub horizontal_text_alignment: HorizontalTextAlignment,
    /// Vertical alignment within the container.
    pub vertical_text_alignment: VerticalTextAlignment,
    /// Line breaking behavior.
    pub linebreak_behavior: BreakLineOn,
}

/// A styled text segment.
#[derive(Debug, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub struct TextSpan {
    /// Text content.
    pub text: String,
    /// Font metadata.
    pub font: FontMetadata,
    /// Style properties.
    pub style: TextStyle,
}

#[derive(Debug, Default, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct FontMetadata {
    /// The font family to which this font belongs.
    pub family: String,
    /// The style of the font, such as italic or normal.
    pub style: FontStyle,
    /// The weight of the font, typically ranging from 100 (thin) to 900 (black).
    pub weight: u16,
}

#[derive(Debug, Default, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub enum FontStyle {
    /// A face that is neither italic not obliqued.
    #[default]
    Normal,
    /// A form that is generally cursive in nature.
    Italic,
    /// A typically-sloped version of the regular face.
    Oblique,
}

/// Style properties for a text segment.
#[derive(Debug, Default, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type),
    serde(rename_all = "camelCase")
)]
pub struct TextStyle {
    /// Glyph height in pixels, may scale with window.
    pub font_size: f32,
    /// Character spacing.
    pub letter_spacing: LetterSpacing,
    /// Line spacing.
    pub line_height: LineHeight,
}

#[derive(Debug, Default, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub enum HorizontalTextAlignment {
    #[default]
    Left,
    Center,
    Right,
    Justified,
}

#[derive(Debug, Default, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub enum VerticalTextAlignment {
    Top,
    #[default]
    Center,
    Bottom,
    Justified,
}

#[derive(Debug, Default, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub enum LetterSpacing {
    #[default]
    Auto,
    Fixed(Abs),
}

#[derive(Debug, Default, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub enum LineHeight {
    #[default]
    Auto,
    Fixed(Abs),
}

#[derive(Debug, Default, Copy, Clone)]
#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, specta::Type)
)]
pub enum BreakLineOn {
    #[default]
    WordBoundary,
    AnyCharacter,
    NoWrap,
}

/// A vector shape node.
#[derive(Component, Debug, Default, Clone)]
pub struct VectorCompNode;
