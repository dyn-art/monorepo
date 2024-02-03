use std::f32::consts::PI;

use bevy_ecs::{
    entity::Entity,
    query::{Changed, Or},
    system::{Commands, Query},
};
use glam::Vec2;

use crate::modules::node::components::{
    mixins::{Anchor, AnchorCommand, DimensionMixin, PathMixin},
    types::StarNode,
};

pub fn construct_star_path(
    mut commands: Commands,
    query: Query<
        (Entity, &StarNode, &DimensionMixin),
        Or<(Changed<StarNode>, Changed<DimensionMixin>)>,
    >,
) {
    for (entity, star, dimension) in query.iter() {
        // Ensure we have a valid star
        if star.point_count < 3 {
            continue;
        }

        let radius_x = dimension.width / 2.0;
        let radius_y = dimension.height / 2.0;
        let inner_radius_x = radius_x * star.inner_radius_ratio;
        let inner_radius_y = radius_y * star.inner_radius_ratio;
        let mut vertices = Vec::new();

        for i in 0..star.point_count {
            // Outer vertex
            let angle = 2.0 * PI / star.point_count as f32 * i as f32 - PI / 2.0;
            let x = radius_x + radius_x * angle.cos();
            let y = radius_y + radius_y * angle.sin();

            // Inner vertex
            let inner_angle = angle + PI / star.point_count as f32;
            let inner_x = radius_x + inner_radius_x * inner_angle.cos();
            let inner_y = radius_y + inner_radius_y * inner_angle.sin();

            if i == 0 {
                // MoveTo for the first outer vertex
                vertices.push(Anchor {
                    command: AnchorCommand::MoveTo {
                        position: Vec2::new(x, y),
                    },
                });
            } else {
                // LineTo the next outer vertex
                vertices.push(Anchor {
                    command: AnchorCommand::LineTo {
                        position: Vec2::new(x, y),
                    },
                });
            }

            // LineTo the corresponding inner vertex
            vertices.push(Anchor {
                command: AnchorCommand::LineTo {
                    position: Vec2::new(inner_x, inner_y),
                },
            });
        }

        // Close the path to the first outer vertex
        vertices.push(Anchor {
            command: AnchorCommand::ClosePath,
        });

        // Insert or update the PathMixin component for the entity
        commands.entity(entity).insert(PathMixin { vertices });
    }
}
