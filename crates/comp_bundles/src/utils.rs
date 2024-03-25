use bevy_transform::components::Transform;
use glam::{EulerRot, Mat4, Quat, Vec3};

pub fn transform_to_z_rotation_rad(transform: &Transform) -> f32 {
    transform.rotation.to_euler(EulerRot::XYZ).2
}
