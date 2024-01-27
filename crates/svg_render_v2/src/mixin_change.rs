use dyn_composition::core::modules::node::components::mixins::{ChildrenMixin, DimensionMixin};
use serde::Serialize;
use specta::Type;

#[derive(Serialize, Clone, Debug, Type)]
#[serde(tag = "type")]
pub enum MixinChange {
    Dimension(DimensionMixin),
    Children(MixinChangeChildrenMixin),
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
