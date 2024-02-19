use bevy_ecs::prelude::*;
use smallvec::SmallVec;

use crate::shared::{BreakLineOn, HorizontalTextAlignment, TextSpan, VerticalTextAlignment};

#[derive(Component, Debug, Default, Copy, Clone)]
pub struct CompNode;

/// Defines a layout container, similar to an HTML `<div>`, for hierarchical organization.
#[derive(Component, Debug, Default, Copy, Clone)]
pub struct FrameCompNode {
    /// Whether the frame clips content outside its bounds. `true` enables clipping.
    pub clip_content: bool,
}

/// Groups related nodes, akin to a layer folder, auto-sized and positioned by its content.
#[derive(Component, Debug, Default, Copy, Clone)]
pub struct GroupCompNode;

/// A rectangle shape node for graphical compositions.
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
