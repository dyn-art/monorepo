use dyn_composition::modules::node::components::mixins::{DimensionMixin, RelativeTransformMixin};
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Hash, Type)]
#[serde(tag = "type")]
pub enum MixinType {
    Dimension,
    RelativeTransform,
}

#[derive(Serialize, Clone, Debug, Type)]
#[serde(tag = "type")]
pub enum MixinChange {
    // Shared
    Dimension(DimensionMixin),

    // Node
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
