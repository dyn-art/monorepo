use bevy_ecs::{
    entity::Entity,
    query::{Changed, Or},
    system::{Commands, Query},
};
use dyn_comp_common::{
    common::{CornerRadii, Size},
    mixins::{CornerRadiiMixin, PathMixin, SizeMixin},
};
use tiny_skia_path::PathBuilder;

pub fn outline_rectangle(
    mut commands: Commands,
    query: Query<
        (Entity, &CornerRadiiMixin, &SizeMixin),
        Or<(Changed<CornerRadiiMixin>, Changed<SizeMixin>)>,
    >,
) {
    for (entity, CornerRadiiMixin(CornerRadii(corner_radii)), SizeMixin(Size(size))) in query.iter()
    {
        let max_radius = (size.x.min(size.y)) / 2.0;
        let min_radius = |radius: f32| -> f32 { radius.min(max_radius) };

        let tl_radius = min_radius(corner_radii[0]);
        let tr_radius = min_radius(corner_radii[1]);
        let br_radius = min_radius(corner_radii[2]);
        let bl_radius = min_radius(corner_radii[3]);

        let mut path_builder = PathBuilder::new();

        // Start from top left, considering top left radius
        if tl_radius > 0.0 {
            path_builder.move_to(tl_radius, 0.0);
        } else {
            path_builder.move_to(0.0, 0.0);
        }

        // Top edge
        path_builder.line_to(size.x - tr_radius, 0.0);

        // Top right corner
        if tr_radius > 0.0 {
            path_builder.quad_to(size.x, 0.0, size.x, tr_radius);
        }

        // Right edge
        path_builder.line_to(size.x, size.y - br_radius);

        // Bottom right corner
        if br_radius > 0.0 {
            path_builder.quad_to(size.x, size.y, size.x - br_radius, size.y);
        }

        // Bottom edge
        path_builder.line_to(bl_radius, size.y);

        // Bottom left corner
        if bl_radius > 0.0 {
            path_builder.quad_to(0.0, size.y, 0.0, size.y - bl_radius);
        }

        // Left edge and close path back to start
        path_builder.line_to(0.0, tl_radius);
        if tl_radius > 0.0 {
            path_builder.quad_to(0.0, 0.0, tl_radius, 0.0);
        }

        path_builder.close();

        // Insert or update the Path component for the entity
        if let Some(path) = path_builder.finish() {
            commands.entity(entity).insert(PathMixin(path));
        }
    }
}
