use glam::{Mat3, Vec2};

pub fn extract_transform_data(matrix: &Mat3) -> (f32, Vec2, Vec2) {
    let a = matrix.x_axis.x;
    let b = matrix.y_axis.x;
    let d = matrix.x_axis.y;
    let e = matrix.y_axis.y;
    let tx = matrix.z_axis.x;
    let ty = matrix.z_axis.y;

    // Calculate rotation
    let rotation = f32::atan2(b, a);

    // Calculate scale
    let scale_x = Vec2::new(a, d).length();
    let scale_y = Vec2::new(b, e).length();

    return (rotation, Vec2::new(scale_x, scale_y), Vec2::new(tx, ty));
}

pub fn rotate_point(point: Vec2, center: Vec2, angle: f32) -> Vec2 {
    Vec2::new(
        (point.x - center.x) * angle.cos() - (point.y - center.y) * angle.sin() + center.x,
        (point.x - center.x) * angle.sin() + (point.y - center.y) * angle.cos() + center.y,
    )
}
