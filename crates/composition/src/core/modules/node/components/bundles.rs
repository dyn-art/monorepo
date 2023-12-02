use bevy_ecs::bundle::Bundle;
use serde::{Deserialize, Serialize};
use specta::Type;

use super::{
    mixins::{
        BlendMixin, ChildrenMixin, DimensionMixin, FillMixin, NodeCompositionMixin,
        RectangleCornerMixin, RelativeTransformMixin,
    },
    types::{Frame, Group, Node, NodeType, Rectangle, Text},
};

// =============================================================================
// Frame
// =============================================================================

#[derive(Bundle, Debug, Serialize, Deserialize, Clone, Type)]
pub struct FrameNodeBundle {
    #[serde(default = "default_frame_node_bundle")]
    pub node: Node,

    #[serde(default)]
    pub frame: Frame,

    #[serde(default)]
    #[serde(rename = "rectangleCornerMixin")]
    pub rectangle_corner_mixin: RectangleCornerMixin,

    #[serde(rename = "children")]
    pub children_mixin: ChildrenMixin,

    #[serde(default)]
    #[serde(rename = "compositionMixin")]
    pub composition_mixin: NodeCompositionMixin,

    #[serde(rename = "relativeTransform")]
    pub relative_transform: RelativeTransformMixin,

    #[serde(rename = "dimension")]
    pub dimension: DimensionMixin,

    #[serde(default)]
    #[serde(rename = "blendMixin")]
    pub blend_mixin: BlendMixin,

    #[serde(default)]
    #[serde(rename = "fill")]
    pub fill_mixin: FillMixin,
}

#[inline]
fn default_frame_node_bundle() -> Node {
    Node {
        node_type: NodeType::Frame,
        name: None,
    }
}

impl Default for FrameNodeBundle {
    fn default() -> Self {
        Self {
            node: default_frame_node_bundle(),
            frame: Frame::default(),
            rectangle_corner_mixin: RectangleCornerMixin::default(),
            children_mixin: ChildrenMixin::default(),
            composition_mixin: NodeCompositionMixin::default(),
            relative_transform: RelativeTransformMixin::default(),
            dimension: DimensionMixin::default(),
            blend_mixin: BlendMixin::default(),
            fill_mixin: FillMixin::default(),
        }
    }
}

// =============================================================================
// Group
// =============================================================================

#[derive(Bundle, Debug, Serialize, Deserialize, Clone, Type)]
pub struct GroupNodeBundle {
    #[serde(default = "default_group_node_bundle")]
    pub node: Node,

    #[serde(default)]
    pub group: Group,

    #[serde(rename = "children")]
    pub children_mixin: ChildrenMixin,

    #[serde(default)]
    #[serde(rename = "compositionMixin")]
    pub composition_mixin: NodeCompositionMixin,

    #[serde(rename = "relativeTransform")]
    pub relative_transform: RelativeTransformMixin,

    #[serde(rename = "dimension")]
    pub dimension: DimensionMixin,

    #[serde(default)]
    #[serde(rename = "blendMixin")]
    pub blend_mixin: BlendMixin,

    #[serde(default)]
    #[serde(rename = "fill")]
    pub fill_mixin: FillMixin,
}

#[inline]
fn default_group_node_bundle() -> Node {
    Node {
        node_type: NodeType::Group,
        name: None,
    }
}

impl Default for GroupNodeBundle {
    fn default() -> Self {
        Self {
            node: default_group_node_bundle(),
            group: Group::default(),
            children_mixin: ChildrenMixin::default(),
            composition_mixin: NodeCompositionMixin::default(),
            relative_transform: RelativeTransformMixin::default(),
            dimension: DimensionMixin::default(),
            blend_mixin: BlendMixin::default(),
            fill_mixin: FillMixin::default(),
        }
    }
}

// =============================================================================
// Rectangle
// =============================================================================

#[derive(Bundle, Debug, Serialize, Deserialize, Clone, Type)]
pub struct RectangleNodeBundle {
    #[serde(default = "default_rectangle_node_bundle")]
    pub node: Node,

    #[serde(default)]
    pub recangle: Rectangle,

    #[serde(default)]
    #[serde(rename = "rectangleCornerMixin")]
    pub rectangle_corner_mixin: RectangleCornerMixin,

    #[serde(default)]
    #[serde(rename = "compositionMixin")]
    pub composition_mixin: NodeCompositionMixin,

    #[serde(rename = "relativeTransform")]
    pub relative_transform: RelativeTransformMixin,

    #[serde(rename = "dimension")]
    pub dimension: DimensionMixin,

    #[serde(default)]
    #[serde(rename = "blendMixin")]
    pub blend_mixin: BlendMixin,

    #[serde(default)]
    #[serde(rename = "fill")]
    pub fill_mixin: FillMixin,
}

#[inline]
fn default_rectangle_node_bundle() -> Node {
    Node {
        node_type: NodeType::Rectangle,
        name: None,
    }
}

impl Default for RectangleNodeBundle {
    fn default() -> Self {
        Self {
            node: default_rectangle_node_bundle(),
            recangle: Rectangle::default(),
            rectangle_corner_mixin: RectangleCornerMixin::default(),
            composition_mixin: NodeCompositionMixin::default(),
            relative_transform: RelativeTransformMixin::default(),
            dimension: DimensionMixin::default(),
            blend_mixin: BlendMixin::default(),
            fill_mixin: FillMixin::default(),
        }
    }
}

// =============================================================================
// Text
// =============================================================================

#[derive(Bundle, Debug, Serialize, Deserialize, Clone, Type)]
pub struct TextNodeBundle {
    #[serde(default = "default_text_node_bundle")]
    pub node: Node,

    pub text: Text,

    #[serde(default)]
    #[serde(rename = "compositionMixin")]
    pub composition_mixin: NodeCompositionMixin,

    #[serde(rename = "relativeTransform")]
    pub relative_transform: RelativeTransformMixin,

    #[serde(rename = "dimension")]
    pub dimension: DimensionMixin,

    #[serde(default)]
    #[serde(rename = "blendMixin")]
    pub blend_mixin: BlendMixin,

    #[serde(default)]
    #[serde(rename = "fill")]
    pub fill_mixin: FillMixin, // TODO: Needs adjustment to work with text sections
}

#[inline]
fn default_text_node_bundle() -> Node {
    Node {
        node_type: NodeType::Text,
        name: None,
    }
}

impl Default for TextNodeBundle {
    fn default() -> Self {
        Self {
            node: default_rectangle_node_bundle(),
            text: Text::default(),
            composition_mixin: NodeCompositionMixin::default(),
            relative_transform: RelativeTransformMixin::default(),
            dimension: DimensionMixin::default(),
            blend_mixin: BlendMixin::default(),
            fill_mixin: FillMixin::default(),
        }
    }
}
