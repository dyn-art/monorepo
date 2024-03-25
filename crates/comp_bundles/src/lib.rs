pub mod components;
pub mod events;
pub mod math;
pub mod viewport;

use bevy_ecs::bundle::Bundle;
use bevy_transform::TransformBundle;
use components::{
    mixins::{
        BlendModeMixin, CornerRadiiMixin, ImageAssetMixin, OpacityMixin, PaintChildMixin,
        PathMixin, SizeMixin, VisibilityMixin,
    },
    nodes::{
        CompNode, EllipseCompNode, FrameCompNode, GroupCompNode, PolygonCompNode,
        RectangleCompNode, StarCompNode, TextCompNode, VectorCompNode,
    },
    paints::{CompPaint, GradientCompPaint, ImageCompPaint, SolidCompPaint},
    styles::{CompStyle, FillCompStyle, StrokeCompStyle},
};

#[derive(Bundle, Debug)]
pub struct FrameCompNodeBundle {
    pub node: CompNode,
    pub frame: FrameCompNode,
    pub transform: TransformBundle,
    pub size: SizeMixin,
    pub corner_radii: CornerRadiiMixin,
    pub visibility: VisibilityMixin,
    pub blend_mode: BlendModeMixin,
    pub opacity: OpacityMixin,
    // https://github.com/Nilirad/bevy_prototype_lyon/issues/207
    // pub styles: StylesMixin,
    // pub children: Children,
}

#[derive(Bundle, Debug)]
pub struct GroupCompNodeBundle {
    pub node: CompNode,
    pub group: GroupCompNode,
    pub transform: TransformBundle,
    pub visibility: VisibilityMixin,
    pub blend_mode: BlendModeMixin,
    pub opacity: OpacityMixin,
    // https://github.com/Nilirad/bevy_prototype_lyon/issues/207
    // pub children: Children,
}

#[derive(Bundle, Debug)]
pub struct RectangleCompNodeBundle {
    pub node: CompNode,
    pub rectangle: RectangleCompNode,
    pub transform: TransformBundle,
    pub size: SizeMixin,
    pub corner_radii: CornerRadiiMixin,
    pub visibility: VisibilityMixin,
    pub blend_mode: BlendModeMixin,
    pub opacity: OpacityMixin,
    // https://github.com/Nilirad/bevy_prototype_lyon/issues/207
    // pub styles: StylesMixin,
}

#[derive(Bundle, Debug)]
pub struct EllipseCompNodeBundle {
    pub node: CompNode,
    pub ellipse: EllipseCompNode,
    pub transform: TransformBundle,
    pub size: SizeMixin,
    pub visibility: VisibilityMixin,
    pub blend_mode: BlendModeMixin,
    pub opacity: OpacityMixin,
    // https://github.com/Nilirad/bevy_prototype_lyon/issues/207
    // pub styles: StylesMixin,
}

#[derive(Bundle, Debug)]
pub struct StarCompNodeBundle {
    pub node: CompNode,
    pub star: StarCompNode,
    pub transform: TransformBundle,
    pub size: SizeMixin,
    pub visibility: VisibilityMixin,
    pub blend_mode: BlendModeMixin,
    pub opacity: OpacityMixin,
    // https://github.com/Nilirad/bevy_prototype_lyon/issues/207
    // pub styles: StylesMixin,
}

#[derive(Bundle, Debug)]
pub struct PolygonCompNodeBundle {
    pub node: CompNode,
    pub polygon: PolygonCompNode,
    pub transform: TransformBundle,
    pub size: SizeMixin,
    pub visibility: VisibilityMixin,
    pub blend_mode: BlendModeMixin,
    pub opacity: OpacityMixin,
    // https://github.com/Nilirad/bevy_prototype_lyon/issues/207
    // pub styles: StylesMixin,
}

#[derive(Bundle, Debug)]
pub struct TextCompNodeBundle {
    pub node: CompNode,
    pub text: TextCompNode,
    pub transform: TransformBundle,
    pub size: SizeMixin,
    pub visibility: VisibilityMixin,
    pub blend_mode: BlendModeMixin,
    pub opacity: OpacityMixin,
    // https://github.com/Nilirad/bevy_prototype_lyon/issues/207
    // pub styles: StylesMixin,
}

#[derive(Bundle, Debug)]
pub struct VectorNodeBundle {
    pub node: CompNode,
    pub vector: VectorCompNode,
    pub transform: TransformBundle,
    pub size: SizeMixin,
    pub visibility: VisibilityMixin,
    pub blend_mode: BlendModeMixin,
    pub opacity: OpacityMixin,
    pub path: PathMixin,
    // https://github.com/Nilirad/bevy_prototype_lyon/issues/207
    // pub styles: StylesMixin,
}

#[derive(Bundle, Debug)]
pub struct SolidPaintBundle {
    pub paint: CompPaint,
    pub solid: SolidCompPaint,
    // https://github.com/Nilirad/bevy_prototype_lyon/issues/207
    // pub parent: PaintParentMixin,
}

#[derive(Bundle, Debug)]
pub struct ImagePaintBundle {
    pub paint: CompPaint,
    pub image: ImageCompPaint,
    pub asset: ImageAssetMixin,
    // https://github.com/Nilirad/bevy_prototype_lyon/issues/207
    // pub parent: PaintParentMixin,
}

#[derive(Bundle, Debug)]
pub struct GradientPaintBundle {
    pub paint: CompPaint,
    pub gradient: GradientCompPaint,
    // https://github.com/Nilirad/bevy_prototype_lyon/issues/207
    // pub parent: PaintParentMixin,
}

#[derive(Bundle, Debug)]
pub struct FillStyleBundle {
    pub style: CompStyle,
    pub fill: FillCompStyle,
    pub paint: PaintChildMixin,
    pub visibility: VisibilityMixin,
    pub blend_mode: BlendModeMixin,
    pub opacity: OpacityMixin,
    // https://github.com/Nilirad/bevy_prototype_lyon/issues/207
    // pub parent: StyleParentMixin,
}

#[derive(Bundle, Debug)]
pub struct StrokeStyleBundle {
    pub style: CompStyle,
    pub stroke: StrokeCompStyle,
    pub paint: PaintChildMixin,
    pub visibility: VisibilityMixin,
    pub blend_mode: BlendModeMixin,
    pub opacity: OpacityMixin,
    // https://github.com/Nilirad/bevy_prototype_lyon/issues/207
    // pub stroke_path: StrokePathMixin,
    // pub parent: StyleParentMixin,
}
