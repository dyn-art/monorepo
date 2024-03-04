use super::{attributes::SvgTransformAttribute, styles::SvgBlendMode};
use bevy_transform::components::Transform;
use dyn_comp_types::common::BlendMode;
use glam::EulerRot;

impl From<&Transform> for SvgTransformAttribute {
    fn from(transform: &Transform) -> Self {
        // Extract the 2D rotation angle (Z axis) from the quaternion
        let angle = transform.rotation.to_euler(EulerRot::XYZ).2;
        let cos_a = angle.cos();
        let sin_a = angle.sin();

        // Extract scale and ensure default scale is 1,1 if scale is 0,0 indicating no scaling applied
        let sx = if transform.scale.x == 0.0 {
            1.0
        } else {
            transform.scale.x
        };
        let sy = if transform.scale.y == 0.0 {
            1.0
        } else {
            transform.scale.y
        };

        let tx = transform.translation.x;
        let ty = transform.translation.y;

        // Create the SVG transformation matrix
        // This matrix combines rotation and scale, then applies translation
        // https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/transform
        // | a c e |
        // | b d f |
        // | 0 0 1 |
        SvgTransformAttribute::Matrix {
            a: cos_a * sx,
            b: sin_a * sy,
            c: -sin_a * sx,
            d: cos_a * sy,
            tx,
            ty,
        }
    }
}

impl From<&BlendMode> for SvgBlendMode {
    fn from(blend_mode: &BlendMode) -> Self {
        match blend_mode {
            BlendMode::Normal => SvgBlendMode::Normal,
            BlendMode::Multiply => SvgBlendMode::Multiply,
            BlendMode::Screen => SvgBlendMode::Screen,
            BlendMode::Overlay => SvgBlendMode::Overlay,
            BlendMode::Darken => SvgBlendMode::Darken,
            BlendMode::Lighten => SvgBlendMode::Lighten,
            BlendMode::ColorDodge => SvgBlendMode::ColorDodge,
            BlendMode::ColorBurn => SvgBlendMode::ColorBurn,
            BlendMode::HardLight => SvgBlendMode::HardLight,
            BlendMode::SoftLight => SvgBlendMode::SoftLight,
            BlendMode::Difference => SvgBlendMode::Difference,
            BlendMode::Exclusion => SvgBlendMode::Exclusion,
            BlendMode::Hue => SvgBlendMode::Hue,
            BlendMode::Saturation => SvgBlendMode::Saturation,
            BlendMode::Color => SvgBlendMode::Color,
            BlendMode::Luminosity => SvgBlendMode::Luminosity,
        }
    }
}
