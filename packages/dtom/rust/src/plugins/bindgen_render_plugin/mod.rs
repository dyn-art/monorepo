pub mod bundles;
pub mod render_event_queue;
pub mod shapes;

use bevy_app::{App, Plugin};
use bevy_ecs::{
    entity::Entity,
    query::With,
    system::{Query, ResMut, Resource},
};
use bevy_utils::HashMap;
use serde::Serialize;

use self::{
    render_event_queue::{RenderEvent, RenderEventQueue},
    shapes::{Path, Shape, Transform},
};

use super::render_plugin::{ExtractSchedule, RenderApp, RenderSchedule};

#[derive(Serialize, Clone, Debug)]
pub enum Change {
    Transform(Transform),
    Path(Path),
}

#[derive(Serialize, Debug)]
pub struct ChangeSet {
    pub entity: Entity,
    pub changes: Vec<Change>,
}

// =============================================================================
// Ressources
// =============================================================================

#[derive(Resource, Default, Debug)]
pub struct ChangedRessource {
    changes: HashMap<Entity, Vec<Change>>,
}

// =============================================================================
// Systems
// =============================================================================

fn extract_transforms(
    mut changed: ResMut<ChangedRessource>,
    query: Query<(Entity, &Transform), With<Shape>>,
) {
    query.for_each(|(entity, transform)| {
        let change_set = changed.changes.entry(entity).or_insert(vec![]);
        change_set.push(Change::Transform(transform.clone()));
    });
}

fn extract_paths(
    mut changed: ResMut<ChangedRessource>,
    query: Query<(Entity, &Path), With<Shape>>,
) {
    query.for_each(|(entity, path)| {
        let change_set = changed.changes.entry(entity).or_insert(vec![]);
        change_set.push(Change::Path(path.clone()));
    });
}

fn send_to_frontend(
    mut changed: ResMut<ChangedRessource>,
    mut event_queue: ResMut<RenderEventQueue>,
) {
    let change_sets: Vec<ChangeSet> = changed
        .changes
        .iter()
        .map(|(entity, changes)| ChangeSet {
            entity: entity.clone(),
            changes: changes.clone(),
        })
        .collect();
    let json_str = serde_json::to_string(&change_sets).expect("Failed to serialize");

    event_queue.push_event(RenderEvent::Update(json_str));

    changed.changes.clear();
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

        // Init Resources
        render_app.init_resource::<ChangedRessource>();
        render_app.init_resource::<RenderEventQueue>();

        render_app
            .add_systems(ExtractSchedule, extract_transforms)
            .add_systems(ExtractSchedule, extract_paths)
            .add_systems(RenderSchedule, send_to_frontend);
    }
}
