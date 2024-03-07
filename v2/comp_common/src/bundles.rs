use crate::{
    mixins::{
        BlendModeMixin, CornerRadiiMixin, ImageAssetMixin, OpacityMixin, SizeMixin, VisibilityMixin,
    },
    nodes::{
        CompNode, EllipseCompNode, FrameCompNode, GroupCompNode, PolygonCompNode,
        RectangleCompNode, StarCompNode, TextCompNode,
    },
    paints::{CompPaint, ImageCompPaint, SolidCompPaint},
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
    pub blend_mode: BlendModeMixin,
    pub opacity: OpacityMixin,
    // https://github.com/Nilirad/bevy_prototype_lyon/issues/207
    // pub fill: FillMixin,
    // pub stroke: StrokeMixin,
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

#[derive(Bundle, Debug)]
pub struct RectangleCompNodeBundle {
    pub node: CompNode,
    pub rectangle: RectangleCompNode,
    pub transform: Transform,
    pub size: SizeMixin,
    pub corner_radii: CornerRadiiMixin,
    pub visibility: VisibilityMixin,
    pub blend_mode: BlendModeMixin,
    pub opacity: OpacityMixin,
    // https://github.com/Nilirad/bevy_prototype_lyon/issues/207
    // pub fill: FillMixin,
    // pub stroke: StrokeMixin,
}

#[derive(Bundle, Debug)]
pub struct EllipseCompNodeBundle {
    pub node: CompNode,
    pub ellipse: EllipseCompNode,
    pub transform: Transform,
    pub size: SizeMixin,
    pub visibility: VisibilityMixin,
    pub blend_mode: BlendModeMixin,
    pub opacity: OpacityMixin,
    // https://github.com/Nilirad/bevy_prototype_lyon/issues/207
    // pub fill: FillMixin,
    // pub stroke: StrokeMixin,
}

#[derive(Bundle, Debug)]
pub struct StarCompNodeBundle {
    pub node: CompNode,
    pub star: StarCompNode,
    pub transform: Transform,
    pub size: SizeMixin,
    pub visibility: VisibilityMixin,
    pub blend_mode: BlendModeMixin,
    pub opacity: OpacityMixin,
    // https://github.com/Nilirad/bevy_prototype_lyon/issues/207
    // pub fill: FillMixin,
    // pub stroke: StrokeMixin,
}

#[derive(Bundle, Debug)]
pub struct PolygonCompNodeBundle {
    pub node: CompNode,
    pub polygon: PolygonCompNode,
    pub transform: Transform,
    pub size: SizeMixin,
    pub visibility: VisibilityMixin,
    pub blend_mode: BlendModeMixin,
    pub opacity: OpacityMixin,
    // https://github.com/Nilirad/bevy_prototype_lyon/issues/207
    // pub fill: FillMixin,
    // pub stroke: StrokeMixin,
}

#[derive(Bundle, Debug)]
pub struct TextCompNodeBundle {
    pub node: CompNode,
    pub text: TextCompNode,
    pub transform: Transform,
    pub size: SizeMixin,
    pub visibility: VisibilityMixin,
    pub blend_mode: BlendModeMixin,
    pub opacity: OpacityMixin,
    // https://github.com/Nilirad/bevy_prototype_lyon/issues/207
    // pub fill: FillMixin,
    // pub stroke: StrokeMixin,
}

#[derive(Bundle, Debug)]
pub struct SolidPaintBundle {
    pub paint: CompPaint,
    pub solid: SolidCompPaint,
    pub blend_mode: BlendModeMixin,
    pub opacity: OpacityMixin,
}

#[derive(Bundle, Debug)]
pub struct ImagePaintBundle {
    pub paint: CompPaint,
    pub image: ImageCompPaint,
    pub asset: ImageAssetMixin,
    pub blend_mode: BlendModeMixin,
    pub opacity: OpacityMixin,
}

#[derive(Bundle, Debug)]
pub struct GradientPaintBundle {
    // TODO
}
