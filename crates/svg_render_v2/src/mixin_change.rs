use dyn_composition::modules::node::components::{
    mixins::{
        BlendMixin, ChildrenMixin, DimensionMixin, NodeCompositionMixin, PaintCompositionMixin,
        PathMixin, RelativeTransformMixin,
    },
    types::SolidPaint,
};
use serde::Serialize;
use specta::Type;

#[derive(Serialize, Clone, Debug, Type)]
#[serde(tag = "type")]
pub enum MixinChange {
    // Shared
    Dimension(DimensionMixin),
    Blend(BlendMixin),

    // Node
    NodeComposition(NodeCompositionMixin),
    Children(MixinChangeChildrenMixin),
    RelativeTransform(MixinChangeRelativeTransformMixin),
    Path(PathMixin),

    // Paint
    PaintComposition(PaintCompositionMixin),
    SolidPaint(SolidPaint),
}

pub trait ToMixinChange {
    fn to_mixin_change(&self) -> MixinChange;
}

impl ToMixinChange for DimensionMixin {
    fn to_mixin_change(&self) -> MixinChange {
        MixinChange::Dimension(self.clone())
    }
}

#[derive(Serialize, Clone, Debug, Type)]
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

#[derive(Serialize, Clone, Debug, Type)]
#[serde(rename_all = "camelCase")]
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
