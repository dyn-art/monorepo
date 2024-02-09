use dyn_composition::modules::node::components::mixins::BlendMode;

use super::styles::SVGBlendMode;
use dyn_composition::modules::node::components::mixins::{Anchor, AnchorCommand};
use glam::{Mat3, Vec2};
use std::fmt::Write;
use strict_num::ApproxEqUlps;
use tiny_skia_path::PathSegment;

use super::attributes::{SVGPathCommand, SVGTransformAttribute};

pub fn map_mat3_to_svg_transform(transform: &Mat3) -> SVGTransformAttribute {
    //   x y z
    // | a d tx |
    // | b e ty |
    // | c f j |
    // https://developer.mozilla.org/en-US/docs/Web/CSS/transform-function/matrix
    SVGTransformAttribute::Matrix {
        a: transform.x_axis.x,
        b: transform.x_axis.y,
        c: transform.y_axis.x,
        d: transform.y_axis.y,
        tx: transform.z_axis.x,
        ty: transform.z_axis.y,
    }
}

static POW_VEC: &[f32] = &[
    1.0,
    10.0,
    100.0,
    1_000.0,
    10_000.0,
    100_000.0,
    1_000_000.0,
    10_000_000.0,
    100_000_000.0,
    1_000_000_000.0,
    10_000_000_000.0,
    100_000_000_000.0,
    1_000_000_000_000.0,
];

/// Approximate zero equality comparisons.
pub trait ApproxZeroUlps: ApproxEqUlps {
    /// Checks if the number is approximately zero.
    fn approx_zero_ulps(&self, ulps: <Self::Flt as strict_num::Ulps>::U) -> bool;
}

impl ApproxZeroUlps for f32 {
    fn approx_zero_ulps(&self, ulps: i32) -> bool {
        self.approx_eq_ulps(&0.0, ulps)
    }
}

impl ApproxZeroUlps for f64 {
    fn approx_zero_ulps(&self, ulps: i64) -> bool {
        self.approx_eq_ulps(&0.0, ulps)
    }
}

fn write_num(num: f32, buf: &mut String, precision: u8) {
    if num.fract().approx_zero_ulps(4) {
        write!(buf, "{} ", num as i32).unwrap();
        return;
    }

    let v = (num * POW_VEC[precision as usize]).round() / POW_VEC[precision as usize];
    write!(buf, "{} ", v).unwrap();
}

pub fn map_skia_path_to_svg_path_string(path: &tiny_skia_path::Path) -> String {
    let coordinates_precision: u8 = 8;
    let mut buf = String::new();

    for seg in path.segments() {
        match seg {
            PathSegment::MoveTo(p) => {
                write!(buf, "M ").unwrap();
                write_num(p.x, &mut buf, coordinates_precision);
                write_num(p.y, &mut buf, coordinates_precision);
            }
            PathSegment::LineTo(p) => {
                write!(buf, "L ").unwrap();
                write_num(p.x, &mut buf, coordinates_precision);
                write_num(p.y, &mut buf, coordinates_precision);
            }
            PathSegment::QuadTo(p1, p) => {
                write!(buf, "Q ").unwrap();
                write_num(p1.x, &mut buf, coordinates_precision);
                write_num(p1.y, &mut buf, coordinates_precision);
                write_num(p.x, &mut buf, coordinates_precision);
                write_num(p.y, &mut buf, coordinates_precision);
            }
            PathSegment::CubicTo(p1, p2, p) => {
                write!(buf, "C ").unwrap();
                write_num(p1.x, &mut buf, coordinates_precision);
                write_num(p1.y, &mut buf, coordinates_precision);
                write_num(p2.x, &mut buf, coordinates_precision);
                write_num(p2.y, &mut buf, coordinates_precision);
                write_num(p.x, &mut buf, coordinates_precision);
                write_num(p.y, &mut buf, coordinates_precision);
            }
            PathSegment::Close => {
                buf.push_str("Z");
            }
        }
    }

    buf
}

pub fn map_anchors_to_svg_path_string(vertices: &[Anchor]) -> String {
    vertices
        .iter()
        .map(|anchor| match &anchor.command {
            AnchorCommand::MoveTo { position } => format!("M{} {}", position.x, position.y),
            AnchorCommand::LineTo { position } => format!("L{} {}", position.x, position.y),
            AnchorCommand::ClosePath => String::from("Z"),
            AnchorCommand::ArcTo {
                radius,
                x_axis_rotation,
                large_arc_flag,
                sweep_flag,
                position,
            } => {
                let Vec2 { x, y } = *position;
                let Vec2 { x: rx, y: ry } = *radius;
                format!(
                    "A{} {} {} {} {} {} {}",
                    rx, ry, x_axis_rotation, *large_arc_flag as u8, *sweep_flag as u8, x, y
                )
            }
            AnchorCommand::CurveTo {
                control_point_1,
                control_point_2,
                position,
            } => {
                let Vec2 { x, y } = *position;
                let Vec2 { x: cx1, y: cy1 } = *control_point_1;
                let Vec2 { x: cx2, y: cy2 } = *control_point_2;
                format!("C{} {} {} {} {} {}", cx1, cy1, cx2, cy2, x, y)
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn map_anchors_to_svg_path_commands(vertices: &[Anchor]) -> Vec<SVGPathCommand> {
    vertices
        .iter()
        .filter_map(|anchor| match &anchor.command {
            AnchorCommand::MoveTo { position } => Some(SVGPathCommand::MoveTo {
                x: position.x,
                y: position.y,
            }),
            AnchorCommand::LineTo { position } => Some(SVGPathCommand::LineTo {
                x: position.x,
                y: position.y,
            }),
            AnchorCommand::ClosePath => Some(SVGPathCommand::ClosePath),
            AnchorCommand::ArcTo {
                radius,
                x_axis_rotation,
                large_arc_flag,
                sweep_flag,
                position,
            } => {
                let Vec2 { x, y } = *position;
                let Vec2 { x: rx, y: ry } = *radius;
                Some(SVGPathCommand::ArcTo {
                    rx,
                    ry,
                    x_axis_rotation: *x_axis_rotation,
                    large_arc_flag: *large_arc_flag,
                    sweep_flag: *sweep_flag,
                    x,
                    y,
                })
            }
            AnchorCommand::CurveTo {
                control_point_1,
                control_point_2,
                position,
            } => {
                let Vec2 { x, y } = *position;
                let Vec2 { x: cx1, y: cy1 } = *control_point_1;
                let Vec2 { x: cx2, y: cy2 } = *control_point_2;
                Some(SVGPathCommand::CurveTo {
                    cx1,
                    cy1,
                    cx2,
                    cy2,
                    x,
                    y,
                })
            }
        })
        .collect()
}

pub fn map_path_commands_to_string(commands: &[SVGPathCommand]) -> String {
    commands
                .iter()
                .map(|command| match command {
                    SVGPathCommand::MoveTo { x, y } => format!("M{x} {y}"),
                    SVGPathCommand::LineTo { x, y } => format!("L{x} {y}"),
                    SVGPathCommand::CurveTo {
                        cx1,
                        cy1,
                        cx2,
                        cy2,
                        x,
                        y,
                    } => {
                        format!("C{cx1} {cy1} {cx2} {cy2} {x} {y}")
                    }
                    SVGPathCommand::ArcTo {
                        rx,
                        ry,
                        x_axis_rotation,
                        large_arc_flag,
                        sweep_flag,
                        x,
                        y,
                    } => {
                        let parsed_large_arc_flag = *large_arc_flag as u8;
                        let parsed_sweep_flag = *sweep_flag as u8;
                        format!(
                        "A{rx} {ry} {x_axis_rotation} {parsed_large_arc_flag} {parsed_sweep_flag} {x} {y}"
                    )},
                    SVGPathCommand::ClosePath => "Z".to_string(),
                })
                .collect::<Vec<_>>()
                .join(" ")
}

pub fn map_blend_mode(blend_mode: &BlendMode) -> SVGBlendMode {
    match blend_mode {
        BlendMode::Normal => SVGBlendMode::Normal,
        BlendMode::Multiply => SVGBlendMode::Multiply,
        BlendMode::Screen => SVGBlendMode::Screen,
        BlendMode::Overlay => SVGBlendMode::Overlay,
        BlendMode::Darken => SVGBlendMode::Darken,
        BlendMode::Lighten => SVGBlendMode::Lighten,
        BlendMode::ColorDodge => SVGBlendMode::ColorDodge,
        BlendMode::ColorBurn => SVGBlendMode::ColorBurn,
        BlendMode::HardLight => SVGBlendMode::HardLight,
        BlendMode::SoftLight => SVGBlendMode::SoftLight,
        BlendMode::Difference => SVGBlendMode::Difference,
        BlendMode::Exclusion => SVGBlendMode::Exclusion,
        BlendMode::Hue => SVGBlendMode::Hue,
        BlendMode::Saturation => SVGBlendMode::Saturation,
        BlendMode::Color => SVGBlendMode::Color,
        BlendMode::Luminosity => SVGBlendMode::Luminosity,
    }
}
