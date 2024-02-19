use bevy_ecs::{
    entity::Entity,
    query::{Changed, Or},
    system::{Commands, Query},
};
use dyn_comp_types::mixins::{CornerRadiiMixin, PathMixin, SizeMixin};

pub fn outline_rectangle(
    mut commands: Commands,
    query: Query<
        (Entity, &CornerRadiiMixin, &SizeMixin),
        Or<(Changed<CornerRadiiMixin>, Changed<SizeMixin>)>,
    >,
) {
    for (entity, CornerRadiiMixin(corner_radii), SizeMixin(size)) in query.iter() {
        let [top_left, top_right, bottom_right, bottom_left] = corner_radii.0.to_array();
        let [width, height] = size.0.to_array();

        let mut builder = tiny_skia_path::PathBuilder::new();

        // Start from top left, considering top left radius
        if top_left > 0.0 {
            builder.move_to(top_left, 0.0);
        } else {
            builder.move_to(0.0, 0.0);
        }

        // Top edge
        builder.line_to(width - top_right, 0.0);

        // Top right corner
        if top_right > 0.0 {
            builder.quad_to(width, 0.0, width, top_right);
        }

        // Right edge
        builder.line_to(width, height - bottom_right);

        // Bottom right corner
        if bottom_right > 0.0 {
            builder.quad_to(width, height, width - bottom_right, height);
        }

        // Bottom edge
        builder.line_to(bottom_left, height);

        // Bottom left corner
        if bottom_left > 0.0 {
            builder.quad_to(0.0, height, 0.0, height - bottom_left);
        }

        // Left edge and close path back to start
        builder.line_to(0.0, top_left);
        if top_left > 0.0 {
            builder.quad_to(0.0, 0.0, top_left, 0.0);
        }

        builder.close();

        // Insert or update the Path component for the entity
        if let Some(path) = builder.finish() {
            commands.entity(entity).insert(PathMixin(path));
        }
    }
}
