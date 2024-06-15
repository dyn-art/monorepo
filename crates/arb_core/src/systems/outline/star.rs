use bevy_ecs::{
    entity::Entity,
    query::{Changed, Or},
    system::{Commands, Query},
};
use dyn_arb_bundles::components::{
    mixins::{PathMixin, SizeMixin, WindingRule},
    nodes::StarArbNode,
};
use std::f32::consts::PI;
use tiny_skia_path::PathBuilder;

pub fn outline_star(
    mut commands: Commands,
    query: Query<
        (Entity, &StarArbNode, &SizeMixin),
        Or<(Changed<StarArbNode>, Changed<SizeMixin>)>,
    >,
) {
    for (entity, star, SizeMixin(size)) in query.iter() {
        if star.point_count < 3 {
            continue;
        }

        let radius = size.to_vec2() / 2.0;
        let inner_radius = radius * star.inner_radius_ratio;

        let mut path_builder = PathBuilder::new();

        for i in 0..star.point_count {
            // Outer vertex
            let angle = 2.0 * PI / star.point_count as f32 * i as f32 - PI / 2.0;
            let (x, y) = (
                radius.x * angle.cos() + radius.x,
                radius.y * angle.sin() + radius.y,
            );

            // Inner vertex
            let inner_angle = angle + PI / star.point_count as f32;
            let (inner_x, inner_y) = (
                inner_radius.x * inner_angle.cos() + radius.x,
                inner_radius.y * inner_angle.sin() + radius.y,
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

        // Insert or update the PathMixin component for the entity
        if let Some(path) = path_builder.finish() {
            commands.entity(entity).insert(PathMixin {
                path,
                winding_rule: WindingRule::Nonzero,
            });
        }
    }
}
