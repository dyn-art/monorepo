use crate::{
    mixins::{
        BlendModeMixin, CornerRadiiMixin, FillMixin, OpacityMixin, SizeMixin, StrokeMixin,
        VisibilityMixin,
    },
    nodes::{
        CompNode, EllipseCompNode, FrameCompNode, GroupCompNode, PolygonCompNode,
        RectangleCompNode, StarCompNode, TextCompNode,
    },
};
use bevy_ecs::bundle::Bundle;
use bevy_transform::components::Transform;

#[derive(Bundle, Debug)]
pub struct FrameCompNodeBundle {
    pub node: CompNode,
    pub frame: FrameCompNode,
    pub transform: Transform,
    pub size: SizeMixin,
    pub corner_radii: CornerRadiiMixin,
    pub visibility: VisibilityMixin,
    pub fill: FillMixin,
    pub blend_mode: BlendModeMixin,
    pub opacity: OpacityMixin,
    pub stroke: StrokeMixin,
}

#[derive(Bundle, Debug)]
pub struct GroupCompNodeBundle {
    pub node: CompNode,
    pub group: GroupCompNode,
    pub transform: Transform,
    pub visibility: VisibilityMixin,
    pub blend_mode: BlendModeMixin,
    pub opacity: OpacityMixin,
}

#[derive(Bundle, Debug, Default)]
pub struct RectangleCompNodeBundle {
    pub node: CompNode,
    pub rectangle: RectangleCompNode,
    pub transform: Transform,
    pub size: SizeMixin,
    pub corner_radii: CornerRadiiMixin,
    pub visibility: VisibilityMixin,
    pub fill: FillMixin,
    pub blend_mode: BlendModeMixin,
    pub opacity: OpacityMixin,
    pub stroke: StrokeMixin,
}

#[derive(Bundle, Debug, Default)]
pub struct EllipseCompNodeBundle {
    pub node: CompNode,
    pub ellipse: EllipseCompNode,
    pub transform: Transform,
    pub size: SizeMixin,
    pub visibility: VisibilityMixin,
    pub fill: FillMixin,
    pub blend_mode: BlendModeMixin,
    pub opacity: OpacityMixin,
    pub stroke: StrokeMixin,
}

#[derive(Bundle, Debug, Default)]
pub struct StarCompNodeBundle {
    pub node: CompNode,
    pub star: StarCompNode,
    pub transform: Transform,
    pub size: SizeMixin,
    pub visibility: VisibilityMixin,
    pub fill: FillMixin,
    pub blend_mode: BlendModeMixin,
    pub opacity: OpacityMixin,
    pub stroke: StrokeMixin,
}

#[derive(Bundle, Debug, Default)]
pub struct PolygonCompNodeBundle {
    pub node: CompNode,
    pub polygon: PolygonCompNode,
    pub transform: Transform,
    pub size: SizeMixin,
    pub visibility: VisibilityMixin,
    pub fill: FillMixin,
    pub blend_mode: BlendModeMixin,
    pub opacity: OpacityMixin,
    pub stroke: StrokeMixin,
}

#[derive(Bundle, Debug, Default)]
pub struct TextCompNodeBundle {
    pub node: CompNode,
    pub text: TextCompNode,
    pub transform: Transform,
    pub size: SizeMixin,
    pub visibility: VisibilityMixin,
    pub fill: FillMixin,
    pub blend_mode: BlendModeMixin,
    pub opacity: OpacityMixin,
    pub stroke: StrokeMixin,
}
