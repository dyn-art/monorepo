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
    #[serde(default)]
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

impl Default for FrameNodeBundle {
    fn default() -> Self {
        Self {
            node: Node {
                node_type: NodeType::Frame,
            },
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
    #[serde(default)]
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

impl Default for GroupNodeBundle {
    fn default() -> Self {
        Self {
            node: Node {
                node_type: NodeType::Group,
            },
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
    #[serde(default)]
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

impl Default for RectangleNodeBundle {
    fn default() -> Self {
        Self {
            node: Node {
                node_type: NodeType::Rectangle,
            },
            recangle: Rectangle::default(),
            rectangle_corner_mixin: RectangleCornerMixin::default(),
            composition_mixin: NodeCompositionMixin::default(),
            relative_transform: RelativeTransformMixin::default(),
            dimension: DimensionMixin::default(),
            blend_mixin: BlendMixin::default(),
        }
    }
}
