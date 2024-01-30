use bevy_ecs::{
    entity::Entity,
    query::{Changed, With},
    system::{Commands, Query},
};
use glam::Vec2;

use crate::modules::node::components::{
    mixins::{DimensionMixin, PathMixin, PreviousDimensionMixin},
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

                // Update each anchor's position in the path
                for anchor in &mut path_mixin.vertices {
                    anchor.position =
                        Vec2::new(anchor.position.x * scale_x, anchor.position.y * scale_y);
                    // Handle specific AnchorCommands if needed
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
