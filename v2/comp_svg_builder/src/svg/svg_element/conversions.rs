use std::fmt::{Error, Write};

use super::{
    attributes::{SvgAttributeColor, SvgPathAttribute, SvgTransformAttribute},
    styles::{SvgBlendModeStyle, SvgStyleColor},
};
use bevy_transform::components::Transform;
use dyn_comp_common::{
    common::{BlendMode, Color},
    math::convert_rh_to_lh,
};
use glam::Mat3;
use tiny_skia_path::{Path, PathSegment};

impl From<&Transform> for SvgTransformAttribute {
    fn from(transform: &Transform) -> Self {
        let mat4 = convert_rh_to_lh(transform.compute_matrix());

        // https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/transform
        // https://docs.aspose.com/svg/net/drawing-basics/transformation-matrix/
        //
        //   x y z (axis)
        // | a c tx |
        // | b d ty |
        // | 0 0 1 |
        //
        // from
        //
        // Mat4 {
        // x_axis: Vec4(a, b, 0.0, 0.0),
        // y_axis: Vec4(c, d, 0.0, 0.0),
        // z_axis: Vec4(0.0, 0.0, 1.0, 0.0),
        // w_axis: Vec4(tx, ty, 0.0, 1.0)
        // }
        SvgTransformAttribute::Matrix {
            a: mat4.x_axis.x,
            b: mat4.x_axis.y,
            c: mat4.y_axis.x,
            d: mat4.y_axis.y,
            tx: mat4.w_axis.x,
            ty: mat4.w_axis.y,
        }
    }
}

impl From<&Mat3> for SvgTransformAttribute {
    fn from(mat3: &Mat3) -> Self {
        // https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/transform
        // https://docs.aspose.com/svg/net/drawing-basics/transformation-matrix/
        //
        //   x y z (axis)
        // | a c tx |
        // | b d ty |
        // | 0 0 1 |
        // from
        //
        // Mat3 {
        // x_axis: Vec4(a, b, 0.0),
        // y_axis: Vec4(c, d, 0.0),
        // z_axis: Vec4(tx, ty, 1.0),
        // }
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
