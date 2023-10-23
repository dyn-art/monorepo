use bevy_ecs::bundle::Bundle;

use super::{
    mixins::{BlendMixin, ChildrenMixin, CompositionMixin, LayoutMixin, RectangleCornerMixin},
    types::{Frame, Group, Node, Rectangle},
};

#[derive(Bundle, Debug)]
pub struct FrameNodeBundle {
    pub node_mixin: Node,
    pub frame_mixin: Frame,
    pub rectangle_corner_mixin: RectangleCornerMixin,
    pub children_mixin: ChildrenMixin,
    pub composition_mixin: CompositionMixin,
    pub layout_mixin: LayoutMixin,
    pub blend_mixin: BlendMixin,
}

impl Default for FrameNodeBundle {
    fn default() -> Self {
        Self {
            node_mixin: Node::default(),
            frame_mixin: Frame::default(),
            rectangle_corner_mixin: RectangleCornerMixin::default(),
            children_mixin: ChildrenMixin::default(),
            composition_mixin: CompositionMixin::default(),
            layout_mixin: LayoutMixin::default(),
            blend_mixin: BlendMixin::default(),
        }
    }
}

#[derive(Bundle, Debug)]
pub struct GroupNodeBundle {
    pub node_mixin: Node,
    pub group_mixin: Group,
    pub children_mixin: ChildrenMixin,
    pub composition_mixin: CompositionMixin,
    pub layout_mixin: LayoutMixin,
    pub blend_mixin: BlendMixin,
}

impl Default for GroupNodeBundle {
    fn default() -> Self {
        Self {
            node_mixin: Node::default(),
            group_mixin: Group::default(),
            children_mixin: ChildrenMixin::default(),
            composition_mixin: CompositionMixin::default(),
            layout_mixin: LayoutMixin::default(),
            blend_mixin: BlendMixin::default(),
        }
    }
}

#[derive(Bundle, Debug)]
pub struct RectangleNodeBundle {
    pub node_mixin: Node,
    pub recangle_mixin: Rectangle,
    pub rectangle_corner_mixin: RectangleCornerMixin,
    pub composition_mixin: CompositionMixin,
    pub layout_mixin: LayoutMixin,
    pub blend_mixin: BlendMixin,
}

impl Default for RectangleNodeBundle {
    fn default() -> Self {
        Self {
            node_mixin: Node::default(),
            recangle_mixin: Rectangle::default(),
            rectangle_corner_mixin: RectangleCornerMixin::default(),
            composition_mixin: CompositionMixin::default(),
            layout_mixin: LayoutMixin::default(),
            blend_mixin: BlendMixin::default(),
        }
    }
}
