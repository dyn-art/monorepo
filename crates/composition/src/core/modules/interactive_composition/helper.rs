use glam::{Mat3, Vec2, Vec3};
use log::info;

pub fn extract_transform_data(matrix: &Mat3) -> (f32, Vec2, Vec2) {
    let a = matrix.x_axis.x;
    let b = matrix.y_axis.x;
    let d = matrix.x_axis.y;
    let e = matrix.y_axis.y;
    let tx = matrix.z_axis.x;
    let ty = matrix.z_axis.y;

    let rotation_in_radians = f32::atan2(b, a);
    let scale_x = Vec2::new(a, d).length();
    let scale_y = Vec2::new(b, e).length();

    return (
        rotation_in_radians,
        Vec2::new(scale_x, scale_y),
        Vec2::new(tx, ty),
    );
}

pub fn rotate_point(point: Vec2, pivot: Vec2, angle_in_radians: f32) -> Vec2 {
    Vec2::new(
        (point.x - pivot.x) * angle_in_radians.cos() - (point.y - pivot.y) * angle_in_radians.sin()
            + pivot.x,
        (point.x - pivot.x) * angle_in_radians.sin()
            + (point.y - pivot.y) * angle_in_radians.cos()
            + pivot.y,
    )
}

/// Applies a rotation to a given transformation matrix (`relative_transform`)
/// around a specified pivot point in 2D space.
///
/// The pivot point for the rotation is defined relative to the `relative_transform` matrix.
/// This means that the rotation will occur around a point on the transformed object itself,
/// rather than the overall scene's origin. By default, if the `pivot_point` is (0, 0),
/// it refers to the top-left corner of the object as transformed by `relative_transform`.
///
/// This behavior aligns with how transformations are typically handled in SVG and CSS,
/// where transformations (including rotation) are applied relative to an element's local
/// coordinate system, which by default has its origin at the element's top-left corner.
///
/// Arguments:
/// * `relative_transform`: A `Mat3` matrix representing the initial transformation
///    (including position, rotation, skew) of the object in 2D space.
/// * `angle_in_radians`: The angle of rotation in radians.
/// * `pivot_point`: A `Vec2` vector representing the pivot point for rotation,
///    relative to the `relative_transform`.
///
/// Returns:
/// * A `Mat3` matrix representing the transformed state after applying the rotation.
///
/// Example:
/// ```
/// let transform = Mat3::identity(); // Some initial transformation
/// let angle = std::f32::consts::PI / 4.0; // 45 degrees in radians
/// let pivot = Vec2::new(1.0, 1.0); // Pivot point
/// let rotated_transform = apply_rotation(transform, angle, pivot);
/// ```
///
/// Reference:
/// * [Understanding SVG Matrix Transformations](https://www.youtube.com/watch?v=nu2MR1RoFsA)
pub fn apply_rotation(relative_transform: Mat3, angle_in_radians: f32, pivot_point: Vec2) -> Mat3 {
    // Translation matrices for moving to the pivot point and back to the origin
    let translate_to_pivot = Mat3::from_translation(pivot_point);
    let translate_to_origin = Mat3::from_translation(-pivot_point);

    // Rotation matrix around the origin
    let cos = angle_in_radians.cos();
    let sin = angle_in_radians.sin();
    let rotation_matrix =
        Mat3::from_cols_array_2d(&[[cos, -sin, 0.0], [sin, cos, 0.0], [0.0, 0.0, 1.0]]);

    // Combine translation and rotation matrices for rotation around pivot point
    let rotation_around_pivot = translate_to_pivot * rotation_matrix * translate_to_origin;

    return relative_transform * rotation_around_pivot;
}

// TODO: Couldn't figure out how to do that without rotating back
pub fn set_rotation(relative_transform: Mat3, angle_in_radians: f32, pivot_point: Vec2) -> Mat3 {
    let (current_angle, _, _) = extract_transform_data(&relative_transform);
    let reset_rotation_transform = apply_rotation(relative_transform, -current_angle, pivot_point);
    return apply_rotation(reset_rotation_transform, angle_in_radians, pivot_point);
}
