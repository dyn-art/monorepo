use bevy_ecs::{
    entity::Entity,
    query::{Changed, With},
    system::{Commands, Query},
};
use glam::Vec2;

use crate::modules::node::components::{
    mixins::{AnchorCommand, DimensionMixin, PathMixin, PreviousDimensionMixin},
    types::VectorNode,
};

pub fn update_vector_path(
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &mut PathMixin,
            &DimensionMixin,
            Option<&mut PreviousDimensionMixin>,
        ),
        (With<VectorNode>, Changed<DimensionMixin>),
    >,
) {
    for (entity, mut path_mixin, dimension, maybe_prev_dimension) in query.iter_mut() {
        match maybe_prev_dimension {
            Some(mut prev_dimension) => {
                // Calculate the scaling factors based on previous dimensions
                let scale_x = dimension.width / prev_dimension.width;
                let scale_y = dimension.height / prev_dimension.height;

                for anchor in &mut path_mixin.vertices {
                    // Scale the position
                    if let Some(position) = anchor.get_position() {
                        anchor.set_position(Vec2::new(position.x * scale_x, position.y * scale_y));
                    }

                    // Handle specific AnchorCommands
                    match &mut anchor.command {
                        AnchorCommand::CurveTo {
                            control_point_1,
                            control_point_2,
                            ..
                        } => {
                            *control_point_1 =
                                Vec2::new(control_point_1.x * scale_x, control_point_1.y * scale_y);
                            *control_point_2 =
                                Vec2::new(control_point_2.x * scale_x, control_point_2.y * scale_y);
                        }
                        AnchorCommand::ArcTo { radius, .. } => {
                            *radius = Vec2::new(radius.x * scale_x, radius.y * scale_y);
                        }
                        _ => {}
                    }
                }

                // Update previous dimensions
                prev_dimension.width = dimension.width;
                prev_dimension.height = dimension.height;
            }
            None => {
                // Add the PreviousDimensionMixin component
                commands.entity(entity).insert(PreviousDimensionMixin {
                    width: dimension.width,
                    height: dimension.height,
                });
            }
        }
    }
}
