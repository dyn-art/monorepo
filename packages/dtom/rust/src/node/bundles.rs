use bevy_ecs::bundle::Bundle;

use super::mixins::{
    BlendMixin, ChildrenMixin, CompositionMixin, FrameMixin, GroupMixin, LayoutMixin, NodeMixin,
    RectangleCornerMixin, RectangleMixin,
};

#[derive(Bundle, Debug)]
pub struct FrameNodeBundle {
    pub node_mixin: NodeMixin,
    pub frame_mixin: FrameMixin,
    pub rectangle_corner_mixin: RectangleCornerMixin,
    pub children_mixin: ChildrenMixin,
    pub composition_mixin: CompositionMixin,
    pub layout_mixin: LayoutMixin,
    pub blend_mixin: BlendMixin,
}

impl Default for FrameNodeBundle {
    fn default() -> Self {
        Self {
            node_mixin: NodeMixin::default(),
            frame_mixin: FrameMixin::default(),
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
    pub node_mixin: NodeMixin,
    pub group_mixin: GroupMixin,
    pub children_mixin: ChildrenMixin,
    pub composition_mixin: CompositionMixin,
    pub layout_mixin: LayoutMixin,
    pub blend_mixin: BlendMixin,
}

impl Default for GroupNodeBundle {
    fn default() -> Self {
        Self {
            node_mixin: NodeMixin::default(),
            group_mixin: GroupMixin::default(),
            children_mixin: ChildrenMixin::default(),
            composition_mixin: CompositionMixin::default(),
            layout_mixin: LayoutMixin::default(),
            blend_mixin: BlendMixin::default(),
        }
    }
}

#[derive(Bundle, Debug)]
pub struct RectangleNodeBundle {
    pub node_mixin: NodeMixin,
    pub recangle_mixin: RectangleMixin,
    pub rectangle_corner_mixin: RectangleCornerMixin,
    pub composition_mixin: CompositionMixin,
    pub layout_mixin: LayoutMixin,
    pub blend_mixin: BlendMixin,
}

impl Default for RectangleNodeBundle {
    fn default() -> Self {
        Self {
            node_mixin: NodeMixin::default(),
            recangle_mixin: RectangleMixin::default(),
            rectangle_corner_mixin: RectangleCornerMixin::default(),
            composition_mixin: CompositionMixin::default(),
            layout_mixin: LayoutMixin::default(),
            blend_mixin: BlendMixin::default(),
        }
    }
}
