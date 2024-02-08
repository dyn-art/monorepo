use dyn_composition::modules::node::components::{
    mixins::{
        BlendMixin, ChildrenMixin, DimensionMixin, GradientStopsMixin, ImageContentMixin,
        NodeCompositionMixin, PaintCompositionMixin, PathMixin, RelativeTransformMixin,
        SkiaPathsMixin,
    },
    types::SolidPaint,
};

use crate::components::{SVGGradientPaint, SVGImagePaint};

#[derive(Clone, Debug)]
pub enum MixinChange {
    // Shared
    Dimension(DimensionMixin),
    Blend(BlendMixin),

    // Node
    NodeComposition(NodeCompositionMixin),
    Children(MixinChangeChildrenMixin),
    RelativeTransform(MixinChangeRelativeTransformMixin),
    Path(PathMixin),
    SkiaPaths(SkiaPathsMixin),

    // Paint
    PaintComposition(PaintCompositionMixin),
    SolidPaint(SolidPaint),
    ImagePaint(SVGImagePaint),
    ImageContent(ImageContentMixin),
    GradientPaint(SVGGradientPaint),
    GradientStopsMixin(GradientStopsMixin),
}

pub trait ToMixinChange {
    fn to_mixin_change(&self) -> MixinChange;
}

impl ToMixinChange for DimensionMixin {
    fn to_mixin_change(&self) -> MixinChange {
        MixinChange::Dimension(self.clone())
    }
}

#[derive(Clone, Debug)]
pub struct MixinChangeChildrenMixin {
    pub children: ChildrenMixin,
}

impl ToMixinChange for ChildrenMixin {
    fn to_mixin_change(&self) -> MixinChange {
        MixinChange::Children(MixinChangeChildrenMixin {
            children: self.clone(),
        })
    }
}

#[derive(Clone, Debug)]
pub struct MixinChangeRelativeTransformMixin {
    pub relative_transform: RelativeTransformMixin,
}

impl ToMixinChange for RelativeTransformMixin {
    fn to_mixin_change(&self) -> MixinChange {
        MixinChange::RelativeTransform(MixinChangeRelativeTransformMixin {
            relative_transform: self.clone(),
        })
    }
}

impl ToMixinChange for NodeCompositionMixin {
    fn to_mixin_change(&self) -> MixinChange {
        MixinChange::NodeComposition(self.clone())
    }
}

impl ToMixinChange for BlendMixin {
    fn to_mixin_change(&self) -> MixinChange {
        MixinChange::Blend(self.clone())
    }
}

impl ToMixinChange for PathMixin {
    fn to_mixin_change(&self) -> MixinChange {
        MixinChange::Path(self.clone())
    }
}

impl ToMixinChange for SkiaPathsMixin {
    fn to_mixin_change(&self) -> MixinChange {
        MixinChange::SkiaPaths(self.clone())
    }
}

impl ToMixinChange for PaintCompositionMixin {
    fn to_mixin_change(&self) -> MixinChange {
        MixinChange::PaintComposition(self.clone())
    }
}

impl ToMixinChange for SolidPaint {
    fn to_mixin_change(&self) -> MixinChange {
        MixinChange::SolidPaint(self.clone())
    }
}

impl ToMixinChange for SVGImagePaint {
    fn to_mixin_change(&self) -> MixinChange {
        MixinChange::ImagePaint(self.clone())
    }
}

impl ToMixinChange for ImageContentMixin {
    fn to_mixin_change(&self) -> MixinChange {
        MixinChange::ImageContent(self.clone())
    }
}

impl ToMixinChange for SVGGradientPaint {
    fn to_mixin_change(&self) -> MixinChange {
        MixinChange::GradientPaint(self.clone())
    }
}

impl ToMixinChange for GradientStopsMixin {
    fn to_mixin_change(&self) -> MixinChange {
        MixinChange::GradientStopsMixin(self.clone())
    }
}
