use dyn_composition::modules::node::components::mixins::BlendMode;

use super::styles::SVGBlendMode;
use dyn_composition::modules::node::components::mixins::{Anchor, AnchorCommand};
use glam::{Mat3, Vec2};

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
