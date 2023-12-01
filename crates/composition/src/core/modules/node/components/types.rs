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

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
pub enum NodeType {
    None,
    Group,
    Rectangle,
    Frame,
}

impl Default for NodeType {
    fn default() -> Self {
        Self::None
    }
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
#[derive(Component, Serialize, Deserialize, Clone, Debug, Type)]
pub struct Ellipse {
    /// Contains the arc data for the ellipse,
    /// which includes the starting angle, ending angle, and the inner radius ratio.
    /// These properties are used to create arcs and donuts shapes.
    pub arc_data: EllipseArcData,
}

/// Represents the arc data for an ellipse.
/// This includes properties for defining the sweep of the ellipse and its inner radius,
/// which are used in UI elements to create various elliptical shapes.
#[derive(Serialize, Deserialize, Clone, Debug, Type)]
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
// Star
// =============================================================================

/// Represents a basic shape node for a star with a set number of points.
#[derive(Component, Serialize, Deserialize, Clone, Debug, Type)]
pub struct Star {
    /// The number of "spikes", or outer points of the star.
    /// This value must be an integer greater than or equal to 3.
    pub point_count: u8,

    /// The ratio of the inner radius to the outer radius of the star.
    /// This value is used to define the sharpness of the star's points.
    pub inner_radius_ratio: f32,
}

// =============================================================================
// Polygon
// =============================================================================

/// Represents a basic shape node for a regular convex polygon with three or more sides.
#[derive(Component, Serialize, Deserialize, Clone, Debug, Type)]
pub struct Polygon {
    /// The number of sides of the polygon.
    /// This value must be an integer greater than or equal to 3.
    pub point_count: u8,
}
