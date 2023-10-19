use bevy_app::{App, Plugin};
use bevy_ecs::{
    entity::Entity,
    query::{Changed, With},
    schedule::IntoSystemConfigs,
    system::{Query, ResMut, Resource},
};
use bevy_utils::HashMap;
use serde::Serialize;

use crate::{
    bindgen::js_bindings,
    js_event_queue::{JsEvent, JsEventQueue},
    shapes::{Path, Shape, Transform},
};

use super::render_plugin::{
    extract_param::Extract, ExtractSchedule, RenderApp, RenderSchedule, RenderSet,
};

#[derive(Serialize, Clone, Debug)]
pub enum Change {
    Transform(Transform),
    Path(Path),
}

#[derive(Serialize, Debug, Clone)]
pub struct ChangeSet {
    pub entity: Entity,
    pub changes: Vec<Change>,
}

// =============================================================================
// Ressources
// =============================================================================

#[derive(Resource, Default, Debug)]
pub struct ChangedComponents {
    changes: HashMap<Entity, Vec<Change>>,
}

// =============================================================================
// Systems
// =============================================================================

fn extract_transforms(
    mut changed: ResMut<ChangedComponents>,
    query: Extract<Query<(Entity, &Transform), (With<Shape>, Changed<Transform>)>>,
) {
    query.for_each(|(entity, transform)| {
        let change_set = changed.changes.entry(entity).or_insert(vec![]);
        change_set.push(Change::Transform(transform.clone()));
    });
}

fn extract_paths(
    mut changed: ResMut<ChangedComponents>,
    query: Extract<Query<(Entity, &Path), (With<Shape>, Changed<Path>)>>,
) {
    query.for_each(|(entity, path)| {
        let change_set = changed.changes.entry(entity).or_insert(vec![]);
        change_set.push(Change::Path(path.clone()));
    });
}

fn queue_render_changes(
    mut changed: ResMut<ChangedComponents>,
    mut event_queue: ResMut<JsEventQueue>,
) {
    if (!changed.changes.is_empty()) {
        let changed_sets: Vec<ChangeSet> = changed
            .changes
            .drain()
            .map(|(entity, changes)| ChangeSet {
                entity: entity.clone(),
                changes: changes.clone(),
            })
            .collect();
        event_queue.push_event(JsEvent::RenderUpdate(changed_sets));
    }
}

fn forward_events_to_js(mut event_queue: ResMut<JsEventQueue>) {
    event_queue.forward_events_to_js();
}

fn extract_system_log() {
    js_bindings::log("Inside extract_system");
}

fn render_system_log() {
    js_bindings::log("Inside render_system");
}

// =============================================================================
// Plugin
// =============================================================================

pub struct BindgenRenderPlugin;

impl Plugin for BindgenRenderPlugin {
    fn build(&self, app: &mut App) {
        let render_app = match app.get_sub_app_mut(RenderApp) {
            Ok(render_app) => render_app,
            Err(_) => return,
        };

        // Register resources
        render_app.init_resource::<ChangedComponents>();
        render_app.init_resource::<JsEventQueue>();

        // Register systems
        render_app
            .add_systems(
                ExtractSchedule,
                (extract_system_log, extract_transforms, extract_paths),
            )
            .add_systems(
                RenderSchedule,
                (
                    render_system_log,
                    queue_render_changes.in_set(RenderSet::Queue),
                    forward_events_to_js.in_set(RenderSet::Render),
                ),
            );
    }
}
