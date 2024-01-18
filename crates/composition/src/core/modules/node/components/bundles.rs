use bevy_ecs::bundle::Bundle;
use serde::{Deserialize, Serialize};
use specta::Type;

use super::{
    mixins::{
        BlendMixin, ChildrenMixin, DimensionMixin, FillMixin, NodeCompositionMixin, NodeMetaMixin,
        RectangleCornerMixin, RelativeTransformMixin,
    },
    types::{Frame, Group, Node, NodeType, Rectangle, Text},
};

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
#[serde(tag = "type")]
pub enum NodeBundle {
    Frame(FrameNodeBundle),
    Group(GroupNodeBundle),
    Rectangle(RectangleNodeBundle),
    Text(TextNodeBundle),
}

// =============================================================================
// Frame
// =============================================================================

#[derive(Bundle, Debug, Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct FrameNodeBundle {
    #[serde(default = "default_frame_node_bundle")]
    pub node: Node,

    #[serde(flatten)]
    pub meta: NodeMetaMixin,

    #[serde(flatten)]
    pub frame: Frame,

    #[serde(default)]
    pub rectangle_corner_mixin: RectangleCornerMixin,

    #[serde(rename = "children")]
    pub children_mixin: ChildrenMixin,

    #[serde(default)]
    pub composition_mixin: NodeCompositionMixin,

    pub relative_transform: RelativeTransformMixin,

    #[serde(rename = "dimension")]
    pub dimension_mixin: DimensionMixin,

    #[serde(default)]
    pub blend_mixin: BlendMixin,

    #[serde(rename = "fill", default)]
    pub fill_mixin: FillMixin,
}

#[inline]
fn default_frame_node_bundle() -> Node {
    Node {
        node_type: NodeType::Frame,
    }
}

impl Default for FrameNodeBundle {
    fn default() -> Self {
        Self {
            node: default_frame_node_bundle(),
            frame: Frame::default(),
            meta: NodeMetaMixin::default(),
            rectangle_corner_mixin: RectangleCornerMixin::default(),
            children_mixin: ChildrenMixin::default(),
            composition_mixin: NodeCompositionMixin::default(),
            relative_transform: RelativeTransformMixin::default(),
            dimension_mixin: DimensionMixin::default(),
            blend_mixin: BlendMixin::default(),
            fill_mixin: FillMixin::default(),
        }
    }
}

// =============================================================================
// Group
// =============================================================================

#[derive(Bundle, Debug, Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct GroupNodeBundle {
    #[serde(default = "default_group_node_bundle")]
    pub node: Node,

    #[serde(flatten)]
    pub group: Group,

    #[serde(flatten)]
    pub meta: NodeMetaMixin,

    #[serde(rename = "children")]
    pub children_mixin: ChildrenMixin,

    #[serde(default)]
    pub composition_mixin: NodeCompositionMixin,

    pub relative_transform: RelativeTransformMixin,

    #[serde(rename = "dimension")]
    pub dimension_mixin: DimensionMixin,

    #[serde(default)]
    pub blend_mixin: BlendMixin,
}

#[inline]
fn default_group_node_bundle() -> Node {
    Node {
        node_type: NodeType::Group,
    }
}

impl Default for GroupNodeBundle {
    fn default() -> Self {
        Self {
            node: default_group_node_bundle(),
            group: Group::default(),
            meta: NodeMetaMixin::default(),
            children_mixin: ChildrenMixin::default(),
            composition_mixin: NodeCompositionMixin::default(),
            relative_transform: RelativeTransformMixin::default(),
            dimension_mixin: DimensionMixin::default(),
            blend_mixin: BlendMixin::default(),
        }
    }
}

// =============================================================================
// Rectangle
// =============================================================================

#[derive(Bundle, Debug, Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct RectangleNodeBundle {
    #[serde(default = "default_rectangle_node_bundle")]
    pub node: Node,

    #[serde(flatten)]
    pub recangle: Rectangle,

    #[serde(flatten)]
    pub meta: NodeMetaMixin,

    #[serde(default)]
    pub rectangle_corner_mixin: RectangleCornerMixin,

    #[serde(default)]
    pub composition_mixin: NodeCompositionMixin,

    pub relative_transform: RelativeTransformMixin,

    #[serde(rename = "dimension")]
    pub dimension_mixin: DimensionMixin,

    #[serde(default)]
    pub blend_mixin: BlendMixin,

    #[serde(rename = "fill", default)]
    pub fill_mixin: FillMixin,
}

#[inline]
fn default_rectangle_node_bundle() -> Node {
    Node {
        node_type: NodeType::Rectangle,
    }
}

impl Default for RectangleNodeBundle {
    fn default() -> Self {
        Self {
            node: default_rectangle_node_bundle(),
            recangle: Rectangle::default(),
            meta: NodeMetaMixin::default(),
            rectangle_corner_mixin: RectangleCornerMixin::default(),
            composition_mixin: NodeCompositionMixin::default(),
            relative_transform: RelativeTransformMixin::default(),
            dimension_mixin: DimensionMixin::default(),
            blend_mixin: BlendMixin::default(),
            fill_mixin: FillMixin::default(),
        }
    }
}

// =============================================================================
// Text
// =============================================================================

#[derive(Bundle, Debug, Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct TextNodeBundle {
    #[serde(default = "default_text_node_bundle")]
    pub node: Node,

    #[serde(flatten)]
    pub text: Text,

    #[serde(flatten)]
    pub meta: NodeMetaMixin,

    #[serde(default)]
    pub composition_mixin: NodeCompositionMixin,

    pub relative_transform: RelativeTransformMixin,

    #[serde(rename = "dimension")]
    pub dimension_mixin: DimensionMixin,

    #[serde(default)]
    pub blend_mixin: BlendMixin,

    #[serde(rename = "fill", default)]
    pub fill_mixin: FillMixin, // TODO: Needs adjustment to work with text segments
}

#[inline]
fn default_text_node_bundle() -> Node {
    Node {
        node_type: NodeType::Text,
    }
}

impl Default for TextNodeBundle {
    fn default() -> Self {
        Self {
            node: default_text_node_bundle(),
            text: Text::default(),
            meta: NodeMetaMixin::default(),
            composition_mixin: NodeCompositionMixin::default(),
            relative_transform: RelativeTransformMixin::default(),
            dimension_mixin: DimensionMixin::default(),
            blend_mixin: BlendMixin::default(),
            fill_mixin: FillMixin::default(),
        }
    }
}
