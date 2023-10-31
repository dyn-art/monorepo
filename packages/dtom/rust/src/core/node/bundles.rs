use bevy_ecs::bundle::Bundle;
use serde::{Deserialize, Serialize};
use specta::Type;

use super::{
    mixins::{BlendMixin, ChildrenMixin, CompositionMixin, LayoutMixin, RectangleCornerMixin},
    types::{Frame, Group, Node, NodeType, Rectangle},
};

#[derive(Bundle, Debug, Serialize, Deserialize, Clone, Type)]
pub struct FrameNodeBundle {
    pub node: Node,
    pub frame: Frame,
    pub rectangle_corner_mixin: RectangleCornerMixin,
    pub children_mixin: ChildrenMixin,
    pub composition_mixin: CompositionMixin,
    pub layout_mixin: LayoutMixin,
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
            composition_mixin: CompositionMixin::default(),
            layout_mixin: LayoutMixin::default(),
            blend_mixin: BlendMixin::default(),
        }
    }
}

#[derive(Bundle, Debug, Serialize, Deserialize, Clone, Type)]
pub struct GroupNodeBundle {
    pub node: Node,
    pub group: Group,
    pub children_mixin: ChildrenMixin,
    pub composition_mixin: CompositionMixin,
    pub layout_mixin: LayoutMixin,
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
            composition_mixin: CompositionMixin::default(),
            layout_mixin: LayoutMixin::default(),
            blend_mixin: BlendMixin::default(),
        }
    }
}

#[derive(Bundle, Debug, Serialize, Deserialize, Clone, Type)]
pub struct RectangleNodeBundle {
    pub node: Node,
    pub recangle: Rectangle,
    pub rectangle_corner_mixin: RectangleCornerMixin,
    pub composition_mixin: CompositionMixin,
    pub layout_mixin: LayoutMixin,
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
            composition_mixin: CompositionMixin::default(),
            layout_mixin: LayoutMixin::default(),
            blend_mixin: BlendMixin::default(),
        }
    }
}
