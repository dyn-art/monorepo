use bevy_transform::components::{GlobalTransform, Transform};
use dyn_comp_bundles::components::mixins::SizeMixin;
use dyn_utils::properties::size::Size;
use glam::{EulerRot, Vec2};

#[derive(Debug, Clone, serde::Serialize, specta::Type)]
#[serde(tag = "type")]
pub enum ComponentChange {
    Size {
        size: Size,
    },
    #[serde(rename_all = "camelCase")]
    Transform {
        rotation_deg: f32,
        translation: Vec2,
    },
    #[serde(rename_all = "camelCase")]
    GlobalTransform {
        rotation_deg: f32,
        translation: Vec2,
    },
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
        ComponentChange::Transform {
            rotation_deg: self.rotation.to_euler(EulerRot::XYZ).2.to_degrees(),
            translation: Vec2::new(self.translation.x, self.translation.y),
        }
    }
}

impl ToComponentChange for GlobalTransform {
    fn to_component_change(&self) -> ComponentChange {
        let transform = self.compute_transform();
        ComponentChange::GlobalTransform {
            rotation_deg: transform.rotation.to_euler(EulerRot::XYZ).2.to_degrees(),
            translation: Vec2::new(transform.translation.x, transform.translation.y),
        }
    }
}
