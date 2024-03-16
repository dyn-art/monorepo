use std::f32::consts::PI;

use bevy_ecs::{
    entity::Entity,
    query::{Changed, Or},
    system::{Commands, Query},
};
use glam::Vec2;

use crate::modules::node::components::{
    mixins::{Anchor, AnchorCommand, DimensionMixin, PathMixin},
    types::PolygonNode,
};

pub fn construct_polygon_path(
    mut commands: Commands,
    query: Query<
        (Entity, &PolygonNode, &DimensionMixin),
        Or<(Changed<PolygonNode>, Changed<DimensionMixin>)>,
    >,
) {
    for (entity, polygon, dimension) in query.iter() {
        // Ensure we have a valid polygon
        if polygon.point_count < 3 {
            continue;
        }

        let radius_x = dimension.width / 2.0;
        let radius_y = dimension.height / 2.0;
        let mut vertices = Vec::new();

        for i in 0..polygon.point_count {
            let angle = 2.0 * PI / polygon.point_count as f32 * i as f32 - PI / 2.0;
            let x = radius_x * angle.cos() + radius_x;
            let y = radius_y * angle.sin() + radius_y;

            if i == 0 {
                // MoveTo for the first vertex
                vertices.push(Anchor {
                    command: AnchorCommand::MoveTo {
                        position: Vec2::new(x, y),
                    },
                });
            } else {
                // LineTo for subsequent vertices
                vertices.push(Anchor {
                    command: AnchorCommand::LineTo {
                        position: Vec2::new(x, y),
                    },
                });
            }
        }

        // Close the path
        vertices.push(Anchor {
            command: AnchorCommand::ClosePath,
        });

        // Insert or update the PathMixin component for the entity
        commands.entity(entity).insert(PathMixin { vertices });
    }
}
