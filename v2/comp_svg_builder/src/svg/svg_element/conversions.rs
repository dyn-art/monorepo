use super::attributes::SvgTransformAttribute;
use bevy_transform::components::Transform;
use glam::EulerRot;

impl From<&Transform> for SvgTransformAttribute {
    fn from(transform: &Transform) -> Self {
        // Extract the 2D rotation angle (Z axis) from the quaternion
        let angle = transform.rotation.to_euler(EulerRot::XYZ).2;
        let cos_a = angle.cos();
        let sin_a = angle.sin();

        let sx = transform.scale.x;
        let sy = transform.scale.y;
        let tx = transform.translation.x;
        let ty = transform.translation.y;

        // Create the SVG transformation matrix
        // This matrix combines rotation and scale, then applies translation
        SvgTransformAttribute::Matrix {
            a: cos_a * sx,  // cos(theta) * scale_x
            b: sin_a * sx,  // sin(theta) * scale_x
            c: -sin_a * sy, // -sin(theta) * scale_y (negated to match SVG's coordinate system)
            d: cos_a * sy,  // cos(theta) * scale_y
            tx,             // Translation on the x-axis
            ty,             // Translation on the y-axis
        }
    }
}
