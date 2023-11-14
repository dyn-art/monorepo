use dyn_composition::core::modules::node::components::mixins::{Anchor, AnchorCommand};
use glam::{Mat3, Vec2};

pub fn transform_to_css_matrix(transform: Mat3) -> String {
    //   x y z
    // | a d tx |
    // | b e ty |
    // | c f j |
    let a = transform.x_axis.x;
    let b = transform.x_axis.y;
    let d = transform.y_axis.x;
    let e = transform.y_axis.y;
    let tx = transform.z_axis.x;
    let ty = transform.z_axis.y;

    return format!("matrix({a}, {b}, {d}, {e}, {tx}, {ty})");
}

pub fn construct_svg_path(vertices: &Vec<Anchor>) -> String {
    // Map path verticies to SVG path commands
    let path_commands: Vec<String> = vertices
        .iter()
        .filter_map(|anchor| {
            let Vec2 { x, y } = anchor.position;
            match &anchor.command {
                AnchorCommand::MoveTo => Some(format!("M {} {}", x, y)),
                AnchorCommand::LineTo => Some(format!("L {} {}", x, y)),
                AnchorCommand::ClosePath => Some("Z".to_string()),
                AnchorCommand::ArcTo {
                    radius,
                    x_axis_rotation,
                    large_arc_flag,
                    sweep_flag,
                } => {
                    let Vec2 { x: rx, y: ry } = *radius;
                    Some(format!(
                        "A {} {} {} {} {} {} {}",
                        rx, ry, x_axis_rotation, *large_arc_flag as u8, *sweep_flag as u8, x, y
                    ))
                }
                AnchorCommand::CurveTo {
                    control_point_1,
                    control_point_2,
                } => {
                    let Vec2 { x: cx1, y: cy1 } = *control_point_1;
                    let Vec2 { x: cx2, y: cy2 } = *control_point_2;
                    Some(format!("C {} {} {} {} {} {}", cx1, cy1, cx2, cy2, x, y))
                }
            }
        })
        .collect();

    return path_commands.join(" ");
}
