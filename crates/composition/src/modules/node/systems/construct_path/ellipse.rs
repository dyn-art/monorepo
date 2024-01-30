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
        let start_angle = ellipse.arc_data.starting_angle;
        let end_angle = ellipse.arc_data.ending_angle;

        // Calculate start and end points for the ellipse
        let start_x = rx * start_angle.cos();
        let start_y = ry * start_angle.sin();
        let end_x = rx * end_angle.cos();
        let end_y = ry * end_angle.sin();

        let large_arc_flag = (end_angle - start_angle).abs() >= PI;
        let sweep_flag = start_angle <= end_angle;

        let mut vertices = Vec::new();

        // Move to start point
        vertices.push(Anchor {
            position: Vec2::new(start_x, start_y),
            command: AnchorCommand::MoveTo,
        });

        // Draw the first arc
        vertices.push(Anchor {
            position: Vec2::new(end_x, end_y),
            command: AnchorCommand::ArcTo {
                radius: Vec2::new(rx, ry),
                x_axis_rotation: 0.0, // Ellipses usually don't have axis rotation
                large_arc_flag,
                sweep_flag,
            },
        });

        // If the arc spans more than 180 degrees, add a second arc
        if large_arc_flag {
            vertices.push(Anchor {
                position: Vec2::new(start_x, start_y),
                command: AnchorCommand::ArcTo {
                    radius: Vec2::new(rx, ry),
                    x_axis_rotation: 0.0,
                    large_arc_flag: false,
                    sweep_flag,
                },
            });
        }

        // Close the path
        vertices.push(Anchor {
            position: Vec2::new(start_x, start_y),
            command: AnchorCommand::ClosePath,
        });

        // Insert or update the PathMixin component for the entity
        commands.entity(entity).insert(PathMixin { vertices });
    }
}
