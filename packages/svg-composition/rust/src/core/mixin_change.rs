use std::ops::Deref;

use bevy_hierarchy::Children;
use dyn_composition::core::modules::node::components::mixins::{
    BlendMixin, ChildrenMixin, DimensionMixin, FillMixin, NodeCompositionMixin, PathMixin,
    RectangleCornerMixin, RelativeTransformMixin,
};
use serde::Serialize;
use specta::Type;

#[derive(Serialize, Clone, Debug, Type)]
#[serde(tag = "type")]
pub enum MixinChange {
    RectangleCorner(RectangleCornerMixin),
    Children(MixinChangeChildrenMixin),
    Dimension(DimensionMixin),
    RelativeTransform(MixinChangeRelativeTransformMixin),
    Composition(NodeCompositionMixin),
    Blend(BlendMixin),
    Path(PathMixin),
    Fill(FillMixin),
}

pub trait ToMixinChange {
    fn to_mixin_change(&self) -> MixinChange;
}

/// Represents the change in the ChildrenMixin.
///
/// This struct separates `ChildrenMixin` due to a type conflict between Rust and TypeScript.
/// In Rust, `ChildrenMixin` is a `Vec<Entity>`, but in TypeScript, it's represented as `Entity[]`.
/// The TypeScript representation can't merge with an object type like
/// `({type: 'Children'} & Entity[])` without conflict.
#[derive(Serialize, Clone, Debug, Type)]
pub struct MixinChangeChildrenMixin {
    children: ChildrenMixin,
}

impl ToMixinChange for Children {
    fn to_mixin_change(&self) -> MixinChange {
        MixinChange::Children(MixinChangeChildrenMixin {
            children: ChildrenMixin(self.deref().to_vec()),
        })
    }
}

impl ToMixinChange for DimensionMixin {
    fn to_mixin_change(&self) -> MixinChange {
        MixinChange::Dimension(self.clone())
    }
}

#[derive(Serialize, Clone, Debug, Type)]
pub struct MixinChangeRelativeTransformMixin {
    #[serde(rename = "relativeTransform")]
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
        MixinChange::Composition(self.clone())
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

impl ToMixinChange for RectangleCornerMixin {
    fn to_mixin_change(&self) -> MixinChange {
        MixinChange::RectangleCorner(self.clone())
    }
}

impl ToMixinChange for FillMixin {
    fn to_mixin_change(&self) -> MixinChange {
        MixinChange::Fill(self.clone())
    }
}
