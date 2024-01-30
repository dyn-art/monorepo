use std::f32::consts::PI;

use bevy_ecs::{
    entity::Entity,
    query::{Changed, Or},
    system::{Commands, Query},
};
use glam::Vec2;

use crate::modules::node::components::{
    mixins::{Anchor, AnchorCommand, DimensionMixin, PathMixin},
    types::EllipseNode,
};

pub fn construct_ellipse_path(
    mut commands: Commands,
    query: Query<
        (Entity, &EllipseNode, &DimensionMixin),
        Or<(Changed<EllipseNode>, Changed<DimensionMixin>)>,
    >,
) {
    for (entity, ellipse, dimension) in query.iter() {
        let rx = dimension.width / 2.0;
        let ry = dimension.height / 2.0;
        let cx = rx;
        let cy = ry;
        let start_angle = ellipse.arc_data.starting_angle;
        let end_angle = ellipse.arc_data.ending_angle;

        let large_arc_flag = (end_angle - start_angle).abs() >= PI;
        let sweep_flag_outer = end_angle - start_angle < PI * 2.0;

        let start_x = cx + rx * start_angle.cos();
        let start_y = cy + ry * start_angle.sin();
        let end_x = cx + rx * end_angle.cos();
        let end_y = cy + ry * end_angle.sin();

        let mut vertices = Vec::new();

        // Move to the start point of the ellipse
        vertices.push(Anchor {
            position: Vec2::new(start_x, start_y),
            command: AnchorCommand::MoveTo,
        });

        // Draw the arc of the ellipse
        vertices.push(Anchor {
            position: Vec2::new(end_x, end_y),
            command: AnchorCommand::ArcTo {
                radius: Vec2::new(rx, ry),
                x_axis_rotation: 0.0,
                large_arc_flag,
                sweep_flag: sweep_flag_outer,
            },
        });

        // Close the path
        vertices.push(Anchor {
            position: Vec2::new(start_x, start_y),
            command: AnchorCommand::ClosePath,
        });

        // Insert or update the PathMixin component for the entity
        commands.entity(entity).insert(PathMixin { vertices });
    }
}
