use std::fmt::{Error, Write};

use super::{
    attributes::{SvgAttributeColor, SvgPathAttribute, SvgTransformAttribute},
    styles::{SvgBlendModeStyle, SvgStyleColor},
};
use bevy_transform::components::Transform;
use dyn_comp_common::common::{BlendMode, Color};
use glam::{EulerRot, Mat3};
use tiny_skia_path::{Path, PathSegment};

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

impl From<&Mat3> for SvgTransformAttribute {
    fn from(mat3: &Mat3) -> Self {
        //   x y z
        // | a d tx |
        // | b e ty |
        // | c f j |
        // https://developer.mozilla.org/en-US/docs/Web/CSS/transform-function/matrix
        Self::Matrix {
            a: mat3.x_axis.x,
            b: mat3.x_axis.y,
            c: mat3.y_axis.x,
            d: mat3.y_axis.y,
            tx: mat3.z_axis.x,
            ty: mat3.z_axis.y,
        }
    }
}

impl From<&BlendMode> for SvgBlendModeStyle {
    fn from(blend_mode: &BlendMode) -> Self {
        match blend_mode {
            BlendMode::Normal => SvgBlendModeStyle::Normal,
            BlendMode::Multiply => SvgBlendModeStyle::Multiply,
            BlendMode::Screen => SvgBlendModeStyle::Screen,
            BlendMode::Overlay => SvgBlendModeStyle::Overlay,
            BlendMode::Darken => SvgBlendModeStyle::Darken,
            BlendMode::Lighten => SvgBlendModeStyle::Lighten,
            BlendMode::ColorDodge => SvgBlendModeStyle::ColorDodge,
            BlendMode::ColorBurn => SvgBlendModeStyle::ColorBurn,
            BlendMode::HardLight => SvgBlendModeStyle::HardLight,
            BlendMode::SoftLight => SvgBlendModeStyle::SoftLight,
            BlendMode::Difference => SvgBlendModeStyle::Difference,
            BlendMode::Exclusion => SvgBlendModeStyle::Exclusion,
            BlendMode::Hue => SvgBlendModeStyle::Hue,
            BlendMode::Saturation => SvgBlendModeStyle::Saturation,
            BlendMode::Color => SvgBlendModeStyle::Color,
            BlendMode::Luminosity => SvgBlendModeStyle::Luminosity,
        }
    }
}

impl From<&Path> for SvgPathAttribute {
    fn from(path: &Path) -> Self {
        SvgPathAttribute(path_to_string(path).unwrap_or_default())
    }
}

fn path_to_string(path: &Path) -> Result<String, Error> {
    let mut s = String::new();
    for segment in path.segments() {
        match segment {
            PathSegment::MoveTo(p) => s.write_fmt(format_args!("M {} {} ", p.x, p.y))?,
            PathSegment::LineTo(p) => s.write_fmt(format_args!("L {} {} ", p.x, p.y))?,
            PathSegment::QuadTo(p0, p1) => {
                s.write_fmt(format_args!("Q {} {} {} {} ", p0.x, p0.y, p1.x, p1.y))?
            }
            PathSegment::CubicTo(p0, p1, p2) => s.write_fmt(format_args!(
                "C {} {} {} {} {} {} ",
                p0.x, p0.y, p1.x, p1.y, p2.x, p2.y
            ))?,
            PathSegment::Close => s.write_fmt(format_args!("Z "))?,
        }
    }

    s.pop(); // ' '

    return Ok(s);
}

impl From<&Color> for SvgAttributeColor {
    fn from(color: &Color) -> Self {
        SvgAttributeColor::RGB {
            red: color.red,
            green: color.green,
            blue: color.blue,
        }
    }
}

impl From<&Color> for SvgStyleColor {
    fn from(color: &Color) -> Self {
        SvgStyleColor::RGB {
            red: color.red,
            green: color.green,
            blue: color.blue,
        }
    }
}
