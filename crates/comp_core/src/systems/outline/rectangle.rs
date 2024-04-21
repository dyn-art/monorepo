use bevy_ecs::{
    entity::Entity,
    query::{Changed, Or},
    system::{Commands, Query},
};
use dyn_comp_bundles::components::mixins::{CornerRadiiMixin, PathMixin, SizeMixin, WindingRule};
use tiny_skia_path::PathBuilder;

// TODO: Round corner for all shapes
// https://plnkr.co/edit/kGnGGyoOCKil02k04snu

// https://stackoverflow.com/questions/10177985/svg-rounded-corner
pub fn outline_rectangle(
    mut commands: Commands,
    query: Query<
        (Entity, &CornerRadiiMixin, &SizeMixin),
        Or<(Changed<CornerRadiiMixin>, Changed<SizeMixin>)>,
    >,
) {
    for (entity, CornerRadiiMixin(corner_radii), SizeMixin(size)) in query.iter() {
        let (width, height) = size.to_tuple();
        let max_radius = (width.min(height)) / 2.0;
        let min_radius = |radius: f32| -> f32 { radius.min(max_radius) };

        let tl_radius = min_radius(corner_radii.tl().to_deg());
        let tr_radius = min_radius(corner_radii.tr().to_deg());
        let br_radius = min_radius(corner_radii.br().to_deg());
        let bl_radius = min_radius(corner_radii.bl().to_deg());

        let mut path_builder = PathBuilder::new();

        // Start from top left, considering top left radius
        if tl_radius > 0.0 {
            path_builder.move_to(tl_radius, 0.0);
        } else {
            path_builder.move_to(0.0, 0.0);
        }

        // Top edge
        path_builder.line_to(width - tr_radius, 0.0);

        // Top right corner
        if tr_radius > 0.0 {
            path_builder.quad_to(width, 0.0, width, tr_radius);
        }

        // Right edge
        path_builder.line_to(width, height - br_radius);

        // Bottom right corner
        if br_radius > 0.0 {
            path_builder.quad_to(width, height, width - br_radius, height);
        }

        // Bottom edge
        path_builder.line_to(bl_radius, height);

        // Bottom left corner
        if bl_radius > 0.0 {
            path_builder.quad_to(0.0, height, 0.0, height - bl_radius);
        }

        // Left edge and close path back to start
        path_builder.line_to(0.0, tl_radius);
        if tl_radius > 0.0 {
            path_builder.quad_to(0.0, 0.0, tl_radius, 0.0);
        }

        // Close the path
        path_builder.close();

        // Insert or update the PathMixin component for the entity
        if let Some(path) = path_builder.finish() {
            commands.entity(entity).insert(PathMixin {
                path,
                winding_rule: WindingRule::Nonzero,
            });
        }
    }
}
