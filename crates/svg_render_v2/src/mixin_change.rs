use dyn_composition::core::modules::node::components::mixins::{
    ChildrenMixin, DimensionMixin, RelativeTransformMixin,
};
use serde::Serialize;
use specta::Type;

#[derive(Serialize, Clone, Debug, Type)]
#[serde(tag = "type")]
pub enum MixinChange {
    Dimension(DimensionMixin),
    Children(MixinChangeChildrenMixin),
    RelativeTransform(MixinChangeRelativeTransformMixin),
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
