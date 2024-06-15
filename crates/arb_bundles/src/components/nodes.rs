use crate::properties::TextAttributeInterval;
use bevy_ecs::component::Component;
use dyn_attributed_string::layout::{
    HorizontalTextAlignment, LineWrap, TextSizingMode, VerticalTextAlignment,
};
use smallvec::SmallVec;
use std::f32::consts::PI;

#[derive(Component, Debug, Copy, Clone)]
pub struct ArbNode {
    pub variant: ArbNodeVariant,
}

#[derive(Debug, Copy, Clone)]
pub enum ArbNodeVariant {
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
pub struct FrameArbNode {
    /// Whether the frame clips content outside its bounds. `true` enables clipping.
    pub clip_content: bool,
}

/// A rectangle shape node.
#[derive(Component, Debug, Default, Copy, Clone)]
pub struct RectangleArbNode;

/// An ellipse shape node, where a circle is a special case with equal width and height.
#[derive(Component, Debug, Default, Copy, Clone)]
pub struct EllipseArbNode {
    /// Arc data for the ellipse, defining start/end angles and inner radius ratio.
    pub arc_data: EllipseArcData,
}

/// Arc data for ellipses, including start/end angles and inner to outer radius ratio.
#[derive(Debug, Copy, Clone)]
pub struct EllipseArcData {
    pub starting_angle: f32,
    pub ending_angle: f32,
    /// Ratio of inner to outer radius, with 0 being a full ellipse.
    pub inner_radius_ratio: f32,
}

impl Default for EllipseArcData {
    fn default() -> Self {
        Self {
            starting_angle: 0.0,
            ending_angle: 2.0 * PI,
            inner_radius_ratio: 0.0,
        }
    }
}

/// A star shape node with customizable point count and inner to outer radius ratio.
#[derive(Component, Debug, Copy, Clone)]
pub struct StarArbNode {
    /// The number of outer points. Minimum value is 3.
    pub point_count: u8,
    /// Defines sharpness of star points.
    pub inner_radius_ratio: f32,
}

impl Default for StarArbNode {
    fn default() -> Self {
        Self {
            point_count: 5,
            inner_radius_ratio: 0.5,
        }
    }
}

/// A regular polygon shape node with three or more sides.
#[derive(Component, Debug, Copy, Clone)]
pub struct PolygonArbNode {
    /// The number of polygon sides, minimum 3.
    pub point_count: u8,
}

impl Default for PolygonArbNode {
    fn default() -> Self {
        Self { point_count: 3 }
    }
}

/// A text shape node with customizable style and alignment properties.
#[derive(Component, Debug, Default, Clone)]
pub struct TextArbNode {
    pub text: String,
    pub attributes: SmallVec<[TextAttributeInterval; 2]>,
    pub line_wrap: LineWrap,
    pub horizontal_text_alignment: HorizontalTextAlignment,
    pub vertical_text_alignment: VerticalTextAlignment,
    pub sizing_mode: TextSizingMode,
}

/// A vector shape node.
#[derive(Component, Debug, Default, Clone)]
pub struct VectorArbNode;
