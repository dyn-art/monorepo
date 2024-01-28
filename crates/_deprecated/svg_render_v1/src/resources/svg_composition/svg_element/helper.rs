use dyn_composition::modules::node::components::mixins::{Anchor, AnchorCommand};
use glam::{Mat3, Vec2};

use super::attributes::{SVGPathCommand, SVGTransformAttribute};

pub fn mat3_to_svg_transform(transform: &Mat3) -> SVGTransformAttribute {
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

pub fn construct_svg_path(vertices: &[Anchor]) -> Vec<SVGPathCommand> {
    vertices
        .iter()
        .filter_map(|anchor| {
            let Vec2 { x, y } = anchor.position;
            match &anchor.command {
                AnchorCommand::MoveTo => Some(SVGPathCommand::MoveTo { x, y }),
                AnchorCommand::LineTo => Some(SVGPathCommand::LineTo { x, y }),
                AnchorCommand::ClosePath => Some(SVGPathCommand::ClosePath),
                AnchorCommand::ArcTo {
                    radius,
                    x_axis_rotation,
                    large_arc_flag,
                    sweep_flag,
                } => {
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
                } => {
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
            }
        })
        .collect()
}
