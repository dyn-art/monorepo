use bevy_ecs::{
    entity::Entity,
    query::{Changed, Or},
    system::{Commands, Query},
};
use dyn_comp_common::{
    mixins::{PathMixin, StrokePathMixin, StyleChildrenMixin, StyleParentMixin},
    styles::StrokeCompStyle,
};
use tiny_skia_path::PathStroker;

pub fn stroke_path_on_stroke_change(
    mut commands: Commands,
    query: Query<
        (Entity, &StrokeCompStyle, &StyleParentMixin),
        Or<(Changed<StrokeCompStyle>, Changed<StyleParentMixin>)>,
    >,
    path_query: Query<&PathMixin>,
) {
    for (entity, stroke_style, StyleParentMixin(parent_entity)) in query.iter() {
        if let Ok(PathMixin(path)) = path_query.get(*parent_entity) {
            let mut stroker = PathStroker::new();
            if let Some(stroke_path) = stroker.stroke(path, &stroke_style.stroke, 1.0) {
                commands.entity(entity).insert(StrokePathMixin(stroke_path));
            }
        }
    }
}

pub fn stroke_path_on_path_change(
    mut commands: Commands,
    query: Query<(&StyleChildrenMixin, &PathMixin), Changed<PathMixin>>,
    stroke_style_query: Query<&StrokeCompStyle>,
) {
    for (StyleChildrenMixin(style_entities), PathMixin(path)) in query.iter() {
        for style_entity in style_entities.iter() {
            if let Ok(stroke_style) = stroke_style_query.get(*style_entity) {
                let mut stroker = PathStroker::new();
                if let Some(stroke_path) = stroker.stroke(path, &stroke_style.stroke, 1.0) {
                    commands
                        .entity(*style_entity)
                        .insert(StrokePathMixin(stroke_path));
                }
            }
        }
    }
}
