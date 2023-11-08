use bevy_ecs::bundle::Bundle;
use serde::{Deserialize, Serialize};
use specta::Type;

use super::{
    mixins::{
        BlendMixin, ChildrenMixin, DimensionMixin, NodeCompositionMixin, RectangleCornerMixin,
        RelativeTransformMixin,
    },
    types::{Frame, Group, Node, NodeType, Rectangle},
};

#[derive(Bundle, Debug, Serialize, Deserialize, Clone, Type)]
pub struct FrameNodeBundle {
    #[serde(default = "frame_node_bundle_node_default")]
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

    #[serde(rename = "blendMixin")]
    pub blend_mixin: BlendMixin,
}

fn frame_node_bundle_node_default() -> Node {
    Node {
        node_type: NodeType::Frame,
    }
}

impl Default for FrameNodeBundle {
    fn default() -> Self {
        Self {
            node: frame_node_bundle_node_default(),
            frame: Frame::default(),
            rectangle_corner_mixin: RectangleCornerMixin::default(),
            children_mixin: ChildrenMixin::default(),
            composition_mixin: NodeCompositionMixin::default(),
            relative_transform: RelativeTransformMixin::default(),
            dimension: DimensionMixin::default(),
            blend_mixin: BlendMixin::default(),
        }
    }
}

#[derive(Bundle, Debug, Serialize, Deserialize, Clone, Type)]
pub struct GroupNodeBundle {
    #[serde(default = "group_node_bundle_node_default")]
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

    #[serde(rename = "blendMixin")]
    pub blend_mixin: BlendMixin,
}

fn group_node_bundle_node_default() -> Node {
    Node {
        node_type: NodeType::Group,
    }
}

impl Default for GroupNodeBundle {
    fn default() -> Self {
        Self {
            node: group_node_bundle_node_default(),
            group: Group::default(),
            children_mixin: ChildrenMixin::default(),
            composition_mixin: NodeCompositionMixin::default(),
            relative_transform: RelativeTransformMixin::default(),
            dimension: DimensionMixin::default(),
            blend_mixin: BlendMixin::default(),
        }
    }
}

#[derive(Bundle, Debug, Serialize, Deserialize, Clone, Type)]
pub struct RectangleNodeBundle {
    #[serde(default = "rectangle_node_bundle_node_default")]
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

    #[serde(rename = "blendMixin")]
    pub blend_mixin: BlendMixin,
}

fn rectangle_node_bundle_node_default() -> Node {
    Node {
        node_type: NodeType::Rectangle,
    }
}

impl Default for RectangleNodeBundle {
    fn default() -> Self {
        Self {
            node: rectangle_node_bundle_node_default(),
            recangle: Rectangle::default(),
            rectangle_corner_mixin: RectangleCornerMixin::default(),
            composition_mixin: NodeCompositionMixin::default(),
            relative_transform: RelativeTransformMixin::default(),
            dimension: DimensionMixin::default(),
            blend_mixin: BlendMixin::default(),
        }
    }
}
