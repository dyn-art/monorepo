use bevy_ecs::{
    entity::Entity,
    query::{Changed, Or},
    system::{Commands, Query},
};
use dyn_comp_common::mixins::{PathMixin, StrokeMixin, StrokePathMixin};
use tiny_skia_path::PathStroker;

pub fn stroke_path(
    mut commands: Commands,
    query: Query<
        (Entity, &StrokeMixin, &PathMixin),
        Or<(Changed<StrokeMixin>, Changed<PathMixin>)>,
    >,
) {
    for (entity, StrokeMixin(stroke), PathMixin(path)) in query.iter() {
        let mut stroker = PathStroker::new();
        if let Some(stroke_path) = stroker.stroke(path, stroke, 1.0) {
            commands.entity(entity).insert(StrokePathMixin(stroke_path));
        }
    }
}
