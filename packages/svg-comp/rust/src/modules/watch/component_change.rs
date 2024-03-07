use bevy_transform::components::Transform;
use dyn_comp_common::{common::Size, mixins::SizeMixin};

#[derive(Debug, Clone, serde::Serialize, specta::Type)]
#[serde(tag = "type")]
pub enum ComponentChange {
    Size { size: Size },
    Transform {},
}

pub trait ToComponentChange {
    fn to_component_change(&self) -> ComponentChange;
}

impl ToComponentChange for SizeMixin {
    fn to_component_change(&self) -> ComponentChange {
        ComponentChange::Size { size: self.0 }
    }
}

impl ToComponentChange for Transform {
    fn to_component_change(&self) -> ComponentChange {
        ComponentChange::Transform {}
    }
}
