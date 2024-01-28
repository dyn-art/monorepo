use dyn_composition::modules::node::components::{
    mixins::{
        BlendMixin, ChildrenMixin, DimensionMixin, FillMixin, GradientStopsMixin,
        ImageContentMixin, NodeCompositionMixin, PaintCompositionMixin, PathMixin,
        RectangleCornerMixin, RelativeTransformMixin,
    },
    types::{GradientPaint, ImagePaint, SolidPaint},
};
use serde::Serialize;
use specta::Type;

// =============================================================================
// Node
// =============================================================================

#[derive(Serialize, Clone, Debug, Type)]
#[serde(tag = "type")]
pub enum NodeMixinChange {
    RectangleCorner(RectangleCornerMixin),
    Children(MixinChangeChildrenMixin),
    Dimension(DimensionMixin),
    RelativeTransform(MixinChangeRelativeTransformMixin),
    NodeComposition(NodeCompositionMixin),
    Blend(BlendMixin),
    Path(PathMixin),
    Fill(FillMixin),
}

pub trait ToNodeMixinChange {
    fn to_mixin_change(&self) -> NodeMixinChange;
}

/// Represents the change in the ChildrenMixin.
///
/// This struct separates `ChildrenMixin` due to a type conflict between Rust and TypeScript.
/// In Rust, `ChildrenMixin` is a `Vec<Entity>`, but in TypeScript, it's represented as `Entity[]`.
/// The TypeScript representation can't merge with an object type like
/// `({type: 'Children'} & Entity[])` without conflict.
#[derive(Serialize, Clone, Debug, Type)]
pub struct MixinChangeChildrenMixin {
    pub children: ChildrenMixin,
}

impl ToNodeMixinChange for ChildrenMixin {
    fn to_mixin_change(&self) -> NodeMixinChange {
        NodeMixinChange::Children(MixinChangeChildrenMixin {
            children: self.clone(),
        })
    }
}

impl ToNodeMixinChange for DimensionMixin {
    fn to_mixin_change(&self) -> NodeMixinChange {
        NodeMixinChange::Dimension(self.clone())
    }
}

#[derive(Serialize, Clone, Debug, Type)]
pub struct MixinChangeRelativeTransformMixin {
    #[serde(rename = "relativeTransform")]
    pub relative_transform: RelativeTransformMixin,
}

impl ToNodeMixinChange for RelativeTransformMixin {
    fn to_mixin_change(&self) -> NodeMixinChange {
        NodeMixinChange::RelativeTransform(MixinChangeRelativeTransformMixin {
            relative_transform: self.clone(),
        })
    }
}

impl ToNodeMixinChange for NodeCompositionMixin {
    fn to_mixin_change(&self) -> NodeMixinChange {
        NodeMixinChange::NodeComposition(self.clone())
    }
}

impl ToNodeMixinChange for BlendMixin {
    fn to_mixin_change(&self) -> NodeMixinChange {
        NodeMixinChange::Blend(self.clone())
    }
}

impl ToNodeMixinChange for PathMixin {
    fn to_mixin_change(&self) -> NodeMixinChange {
        NodeMixinChange::Path(self.clone())
    }
}

impl ToNodeMixinChange for RectangleCornerMixin {
    fn to_mixin_change(&self) -> NodeMixinChange {
        NodeMixinChange::RectangleCorner(self.clone())
    }
}

// =============================================================================
// Paint
// =============================================================================

#[derive(Serialize, Clone, Debug, Type)]
#[serde(tag = "type")]
pub enum PaintMixinChange {
    Dimension(DimensionMixin),
    Blend(BlendMixin),
    PaintComposition(PaintCompositionMixin),
    ImageContent(ImageContentMixin),
    SolidPaint(SolidPaint),
    ImagePaint(ImagePaint),
    GradientPaint(GradientPaint),
    GradientStopsMixin(GradientStopsMixin),
}

impl ToPaintMixinChange for BlendMixin {
    fn to_mixin_change(&self) -> PaintMixinChange {
        PaintMixinChange::Blend(self.clone())
    }
}

impl ToPaintMixinChange for DimensionMixin {
    fn to_mixin_change(&self) -> PaintMixinChange {
        PaintMixinChange::Dimension(self.clone())
    }
}

pub trait ToPaintMixinChange {
    fn to_mixin_change(&self) -> PaintMixinChange;
}

impl ToPaintMixinChange for PaintCompositionMixin {
    fn to_mixin_change(&self) -> PaintMixinChange {
        PaintMixinChange::PaintComposition(self.clone())
    }
}

impl ToPaintMixinChange for ImageContentMixin {
    fn to_mixin_change(&self) -> PaintMixinChange {
        PaintMixinChange::ImageContent(self.clone())
    }
}

impl ToPaintMixinChange for SolidPaint {
    fn to_mixin_change(&self) -> PaintMixinChange {
        PaintMixinChange::SolidPaint(self.clone())
    }
}

impl ToPaintMixinChange for ImagePaint {
    fn to_mixin_change(&self) -> PaintMixinChange {
        PaintMixinChange::ImagePaint(self.clone())
    }
}

impl ToPaintMixinChange for GradientPaint {
    fn to_mixin_change(&self) -> PaintMixinChange {
        PaintMixinChange::GradientPaint(self.clone())
    }
}

impl ToPaintMixinChange for GradientStopsMixin {
    fn to_mixin_change(&self) -> PaintMixinChange {
        PaintMixinChange::GradientStopsMixin(self.clone())
    }
}
