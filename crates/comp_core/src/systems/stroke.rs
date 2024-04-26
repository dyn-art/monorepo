use std::collections::HashSet;

use bevy_ecs::{
    entity::Entity,
    query::Changed,
    system::{Commands, Query},
};
use dyn_comp_bundles::components::{
    mixins::{PathMixin, StrokePathMixin, StyleChildrenMixin, StyleParentMixin, WindingRule},
    styles::StrokeCompStyle,
};
use tiny_skia_path::PathStroker;

pub fn stroke_path_system(
    mut commands: Commands,
    stroke_query: Query<
        (Entity, &StrokeCompStyle, Option<&StyleParentMixin>),
        Changed<StrokeCompStyle>,
    >,
    path_query: Query<(&PathMixin, Option<&StyleChildrenMixin>), Changed<PathMixin>>,
    stroke_style_query: Query<&StrokeCompStyle>,
) {
    let mut processed_entities: HashSet<Entity> = HashSet::new();

    // Handle stroke style changes
    for (entity, stroke_style, maybe_style_parent_mixin) in stroke_query.iter() {
        if processed_entities.insert(entity) {
            if let Some(StyleParentMixin(parent_entity)) = maybe_style_parent_mixin {
                if let Ok((PathMixin { path, .. }, _)) = path_query.get(*parent_entity) {
                    stroke_path(&mut commands, entity, path, stroke_style);
                }
            }
        }
    }

    // Handle path changes
    for (PathMixin { path, .. }, maybe_style_children_mixin) in path_query.iter() {
        if let Some(StyleChildrenMixin(style_entities)) = maybe_style_children_mixin {
            for style_entity in style_entities.iter() {
                if processed_entities.insert(*style_entity) {
                    if let Ok(stroke_style) = stroke_style_query.get(*style_entity) {
                        stroke_path(&mut commands, *style_entity, path, stroke_style);
                    }
                }
            }
        }
    }
}

fn stroke_path(
    commands: &mut Commands,
    entity: Entity,
    path: &tiny_skia_path::Path,
    stroke_style: &StrokeCompStyle,
) {
    let mut stroker = PathStroker::new();
    if let Some(stroke_path) = stroker.stroke(path, &stroke_style.stroke, 1.0) {
        commands.entity(entity).insert(StrokePathMixin {
            path: stroke_path,
            winding_rule: WindingRule::Nonzero,
        });
    }
}
