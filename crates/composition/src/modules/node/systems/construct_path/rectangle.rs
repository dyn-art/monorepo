use std::cmp::min;

use bevy_ecs::{
    entity::Entity,
    query::{Changed, Or},
    system::{Commands, Query},
};
use glam::Vec2;

use crate::modules::node::components::mixins::{
    Anchor, AnchorCommand, DimensionMixin, PathMixin, RectangleCornerMixin,
};

pub fn construct_rectangle_path(
    mut commands: Commands,
    query: Query<
        (Entity, &RectangleCornerMixin, &DimensionMixin),
        Or<(Changed<RectangleCornerMixin>, Changed<DimensionMixin>)>,
    >,
) {
    for (entity, corners, dimension) in query.iter() {
        let mut path = PathMixin {
            vertices: Vec::new(),
        };
        let max_radius = min(dimension.width as u32, dimension.height as u32) / 2;

        let min_radius = |radius: u8| -> f32 { min(radius as i32, max_radius as i32) as f32 };

        // Move to start point, considering the top left radius
        path.vertices.push(Anchor {
            position: Vec2::new(min_radius(corners.top_left_radius), 0.0),
            command: AnchorCommand::MoveTo,
        });

        // Top right corner
        path.vertices.push(Anchor {
            position: Vec2::new(
                dimension.width as f32 - min_radius(corners.top_right_radius),
                0.0,
            ),
            command: AnchorCommand::LineTo,
        });

        if corners.top_right_radius > 0 {
            path.vertices.push(Anchor {
                position: Vec2::new(dimension.width as f32, min_radius(corners.top_right_radius)),
                command: AnchorCommand::ArcTo {
                    radius: Vec2::splat(min_radius(corners.top_right_radius)),
                    x_axis_rotation: 0.0,
                    large_arc_flag: false,
                    sweep_flag: true,
                },
            });
        }

        // Bottom right corner
        path.vertices.push(Anchor {
            position: Vec2::new(
                dimension.width as f32,
                dimension.height as f32 - min_radius(corners.bottom_right_radius),
            ),
            command: AnchorCommand::LineTo,
        });

        if corners.bottom_right_radius > 0 {
            path.vertices.push(Anchor {
                position: Vec2::new(
                    dimension.width as f32 - min_radius(corners.bottom_right_radius),
                    dimension.height as f32,
                ),
                command: AnchorCommand::ArcTo {
                    radius: Vec2::splat(min_radius(corners.bottom_right_radius)),
                    x_axis_rotation: 0.0,
                    large_arc_flag: false,
                    sweep_flag: true,
                },
            });
        }

        // Bottom left corner
        path.vertices.push(Anchor {
            position: Vec2::new(
                min_radius(corners.bottom_left_radius),
                dimension.height as f32,
            ),
            command: AnchorCommand::LineTo,
        });

        if corners.bottom_left_radius > 0 {
            path.vertices.push(Anchor {
                position: Vec2::new(
                    0.0,
                    dimension.height as f32 - min_radius(corners.bottom_left_radius),
                ),
                command: AnchorCommand::ArcTo {
                    radius: Vec2::splat(min_radius(corners.bottom_left_radius)),
                    x_axis_rotation: 0.0,
                    large_arc_flag: false,
                    sweep_flag: true,
                },
            });
        }

        // Back to top left corner
        path.vertices.push(Anchor {
            position: Vec2::new(0.0, min_radius(corners.top_left_radius)),
            command: AnchorCommand::LineTo,
        });

        if corners.top_left_radius > 0 {
            path.vertices.push(Anchor {
                position: Vec2::new(min_radius(corners.top_left_radius), 0.0),
                command: AnchorCommand::ArcTo {
                    radius: Vec2::splat(min_radius(corners.top_left_radius)),
                    x_axis_rotation: 0.0,
                    large_arc_flag: false,
                    sweep_flag: true,
                },
            });
        }

        // Close the path
        path.vertices.push(Anchor {
            position: Vec2::ZERO,
            command: AnchorCommand::ClosePath,
        });

        // Insert or update the PathMixin component for the entity
        commands.entity(entity).insert(path);
    }
}
