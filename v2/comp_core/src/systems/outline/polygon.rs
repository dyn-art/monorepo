use bevy_ecs::{
    entity::Entity,
    query::{Changed, Or},
    system::{Commands, Query},
};
use dyn_comp_common::{
    common::Size,
    mixins::{PathMixin, SizeMixin},
    nodes::PolygonCompNode,
};
use tiny_skia_path::PathBuilder;

pub fn outline_polygon(
    mut commands: Commands,
    query: Query<
        (Entity, &PolygonCompNode, &SizeMixin),
        Or<(Changed<PolygonCompNode>, Changed<SizeMixin>)>,
    >,
) {
    for (entity, polygon, SizeMixin(Size(size))) in query.iter() {
        if polygon.point_count < 3 {
            continue;
        }

        let radius_x = size.x / 2.0;
        let radius_y = size.y / 2.0;

        let mut path_builder = PathBuilder::new();

        for i in 0..polygon.point_count {
            let angle = 2.0 * std::f32::consts::PI / polygon.point_count as f32 * i as f32
                - std::f32::consts::PI / 2.0;
            let x = radius_x * angle.cos() + radius_x;
            let y = radius_y * angle.sin() + radius_y;

            // Move to the first vertex or line to subsequent vertices
            if i == 0 {
                path_builder.move_to(x, y);
            } else {
                path_builder.line_to(x, y);
            }
        }

        // Close the path to complete the polygon shape
        path_builder.close();

        // Insert or update the Path component for the entity
        if let Some(path) = path_builder.finish() {
            commands.entity(entity).insert(PathMixin(path));
        }
    }
}
