use bevy_ecs::{
    entity::Entity,
    query::{Changed, Or},
    system::{Commands, Query},
};
use dyn_comp_common::{
    common::Size,
    mixins::{PathMixin, SizeMixin},
    nodes::EllipseCompNode,
};
use tiny_skia_path::{PathBuilder, Rect};

pub fn outline_ellipse(
    mut commands: Commands,
    query: Query<
        (Entity, &EllipseCompNode, &SizeMixin),
        Or<(Changed<EllipseCompNode>, Changed<SizeMixin>)>,
    >,
) {
    for (entity, ellipse, SizeMixin(Size(size))) in query.iter() {
        let mut path_builder = PathBuilder::new();

        // Handle a full ellipse or circle
        if ellipse.arc_data.starting_angle == 0.0
            && ellipse.arc_data.ending_angle == 2.0 * std::f32::consts::PI
        {
            path_builder.push_oval(Rect::from_xywh(0.0, 0.0, size.x, size.y).unwrap());
        }
        // Handle a ellipse or circle with arc
        else {
            // TODO
        }

        // Insert or update the PathMixin component for the entity
        if let Some(path) = path_builder.finish() {
            commands.entity(entity).insert(PathMixin(path));
        }
    }
}
