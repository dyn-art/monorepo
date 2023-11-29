use glam::{Mat3, Vec2};

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
