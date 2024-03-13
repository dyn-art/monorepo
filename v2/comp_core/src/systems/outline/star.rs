use bevy_ecs::{
    entity::Entity,
    query::{Changed, Or},
    system::{Commands, Query},
};
use dyn_comp_common::{
    common::Size,
    mixins::{PathMixin, SizeMixin},
    nodes::StarCompNode,
};
use tiny_skia_path::PathBuilder;

pub fn outline_star(
    mut commands: Commands,
    query: Query<
        (Entity, &StarCompNode, &SizeMixin),
        Or<(Changed<StarCompNode>, Changed<SizeMixin>)>,
    >,
) {
    for (entity, star, SizeMixin(Size(size))) in query.iter() {
        if star.point_count < 3 {
            continue;
        }

        let radius_x = size.x / 2.0;
        let radius_y = size.y / 2.0;
        let inner_radius_x = radius_x * star.inner_radius_ratio;
        let inner_radius_y = radius_y * star.inner_radius_ratio;

        let mut path_builder = PathBuilder::new();

        for i in 0..star.point_count {
            // Calculate angles for the outer and inner vertices
            let angle = 2.0 * std::f32::consts::PI / star.point_count as f32 * i as f32
                - std::f32::consts::PI / 2.0;
            let inner_angle = angle + std::f32::consts::PI / star.point_count as f32;

            // Calculate coordinates for the outer and inner vertices
            let (x, y) = (radius_x * angle.cos(), radius_y * angle.sin());
            let (inner_x, inner_y) = (
                inner_radius_x * inner_angle.cos(),
                inner_radius_y * inner_angle.sin(),
            );

            // Move to the first outer vertex or line to subsequent vertices
            if i == 0 {
                path_builder.move_to(x, y);
            } else {
                path_builder.line_to(x, y);
            }

            // Line to the corresponding inner vertex
            path_builder.line_to(inner_x, inner_y);
        }

        // Close the path to complete the star shape
        path_builder.close();

        // Insert or update the Path component for the entity
        if let Some(path) = path_builder.finish() {
            commands.entity(entity).insert(PathMixin(path));
        }
    }
}
