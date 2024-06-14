use bevy_ecs::system::Query;
use bevy_hierarchy::Parent;
use bevy_transform::components::{GlobalTransform, Transform};
use dyn_utils::units::{auto_length::AutoLength, length::Length};
use glam::{EulerRot, Vec3};

pub fn transform_to_z_rotation_rad(transform: &Transform) -> f32 {
    transform.rotation.to_euler(EulerRot::XYZ).2
}

pub fn get_parent_global_transfrom<'a>(
    maybe_parent: Option<&'a Parent>,
    global_transform_query: &'a Query<&GlobalTransform>,
) -> Option<&'a GlobalTransform> {
    maybe_parent.and_then(|parent| global_transform_query.get(parent.get()).ok())
}

/// Transforms a global point to local space relative to the given parent entity's transform.
///
/// This function applies the full transformation (including translation) of the parent's
/// `GlobalTransform` to the given point, effectively converting the point from global
/// coordinates to the parent's local coordinate space.
pub fn global_to_local_point3(
    global_point: Vec3,
    maybe_parent_global_transfrom: Option<&GlobalTransform>,
) -> Vec3 {
    if let Some(parent_global_transform) = maybe_parent_global_transfrom {
        return parent_global_transform
            .compute_matrix()
            .inverse()
            .transform_point3(global_point);
    }
    // Fallback to the global point if the parent's global transform cannot be accessed
    else {
        return global_point;
    }
}

/// Transforms a global vector to local space relative to the given parent entity's transform.
///
/// This function applies only the rotation and scaling (not the translation) part of the
/// parent's `GlobalTransform` to the given vector.
pub fn global_to_local_vector3(
    global_point: Vec3,
    maybe_parent_global_transfrom: Option<&GlobalTransform>,
) -> Vec3 {
    if let Some(parent_global_transform) = maybe_parent_global_transfrom {
        return parent_global_transform
            .compute_matrix()
            .inverse()
            .transform_vector3(global_point);
    }
    // Fallback to the global point if the parent's global transform cannot be accessed
    else {
        return global_point;
    }
}

pub fn length_to_taffy(value: Length) -> taffy::LengthPercentage {
    match value {
        Length::Abs { value } => taffy::LengthPercentage::Length(value.to_pt()),
        Length::Ratio { value } => taffy::LengthPercentage::Percent(value.get()),
    }
}

pub fn auto_length_to_taffy(value: AutoLength) -> taffy::LengthPercentageAuto {
    match value {
        AutoLength::Abs { value } => taffy::LengthPercentageAuto::Length(value.to_pt()),
        AutoLength::Ratio { value } => taffy::LengthPercentageAuto::Percent(value.get()),
        AutoLength::Auto => taffy::LengthPercentageAuto::Auto,
    }
}
