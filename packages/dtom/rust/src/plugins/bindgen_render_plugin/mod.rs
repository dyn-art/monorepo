use bevy_app::{App, Plugin};
use bevy_ecs::{
    entity::Entity,
    query::{Changed, With},
    schedule::IntoSystemConfigs,
    system::{Commands, Query, ResMut, Resource},
};
use bevy_utils::HashMap;
use serde::Serialize;
#[cfg(feature = "cli")]
use specta::Type;

use crate::{
    bindgen::{
        event_queue::to_js_event_queue::{ToJsEvent, ToJsEventQueue},
        js_bindings,
    },
    core::node::{
        mixins::{
            BlendMixin, ChildrenMixin, CompositionMixin, LayoutMixin, PathMixin,
            RectangleCornerMixin,
        },
        types::Node,
    },
};

use super::render_plugin::{
    extract_param::Extract, ExtractSchedule, RenderApp, RenderSchedule, RenderSet,
};

#[cfg_attr(feature = "cli", derive(Type))]
#[derive(Serialize, Clone, Debug)]
pub enum RenderChange {
    RectangleCorner(RectangleCornerMixin),
    Children(ChildrenMixin),
    Layout(LayoutMixin),
    Composition(CompositionMixin),
    Blend(BlendMixin),
    Path(PathMixin),
}

// =============================================================================
// Ressources
// =============================================================================

#[derive(Resource, Default, Debug)]
pub struct ChangedComponents {
    changes: HashMap<Entity, Vec<RenderChange>>,
}

// =============================================================================
// Systems
// =============================================================================

fn extract_rectangle_corner_mixin(
    mut changed: ResMut<ChangedComponents>,
    query: Extract<
        Query<(Entity, &RectangleCornerMixin), (With<Node>, Changed<RectangleCornerMixin>)>,
    >,
) {
    query.for_each(|(entity, rectangle_corner_mixin)| {
        let change_set = changed.changes.entry(entity).or_insert(vec![]);
        change_set.push(RenderChange::RectangleCorner(
            rectangle_corner_mixin.clone(),
        ));
    });
}

fn extract_children_mixin(
    mut changed: ResMut<ChangedComponents>,
    query: Extract<Query<(Entity, &ChildrenMixin), (With<Node>, Changed<ChildrenMixin>)>>,
) {
    query.for_each(|(entity, children_mixin)| {
        let change_set = changed.changes.entry(entity).or_insert(vec![]);
        change_set.push(RenderChange::Children(children_mixin.clone()));
    });
}

fn extract_layout_mixin(
    mut changed: ResMut<ChangedComponents>,
    query: Extract<Query<(Entity, &LayoutMixin), (With<Node>, Changed<LayoutMixin>)>>,
) {
    query.for_each(|(entity, layout_mixin)| {
        let change_set = changed.changes.entry(entity).or_insert(vec![]);
        change_set.push(RenderChange::Layout(layout_mixin.clone()));
    });
}

fn extract_composition_mixin(
    mut changed: ResMut<ChangedComponents>,
    query: Extract<Query<(Entity, &CompositionMixin), (With<Node>, Changed<CompositionMixin>)>>,
) {
    query.for_each(|(entity, composition_mixin)| {
        let change_set = changed.changes.entry(entity).or_insert(vec![]);
        change_set.push(RenderChange::Composition(composition_mixin.clone()));
    });
}

fn extract_blend_mixin(
    mut changed: ResMut<ChangedComponents>,
    query: Extract<Query<(Entity, &BlendMixin), (With<Node>, Changed<BlendMixin>)>>,
) {
    query.for_each(|(entity, blend_mixin)| {
        let change_set = changed.changes.entry(entity).or_insert(vec![]);
        change_set.push(RenderChange::Blend(blend_mixin.clone()));
    });
}

fn extract_path_mixin(
    mut changed: ResMut<ChangedComponents>,
    query: Extract<Query<(Entity, &PathMixin), (With<Node>, Changed<PathMixin>)>>,
) {
    query.for_each(|(entity, path_mixin)| {
        let change_set = changed.changes.entry(entity).or_insert(vec![]);
        change_set.push(RenderChange::Path(path_mixin.clone()));
    });
}

fn prepare_render_changes(mut commands: Commands, mut changed: ResMut<ChangedComponents>) {
    // TODO:
    // Prepare SVG path based on PathMixin
    // and other stuff that needs to be prepared
    // or not because its SVG specific?
}

fn queue_render_changes(
    mut changed: ResMut<ChangedComponents>,
    event_queue: ResMut<ToJsEventQueue>,
) {
    if !changed.changes.is_empty() {
        changed
            .changes
            .drain()
            .into_iter()
            .for_each(|(entity, changes)| {
                event_queue.push_event(ToJsEvent::RenderUpdate {
                    entity: entity.index(),
                    changes: changes.clone(),
                });
            });
    }
}

fn forward_render_changes_to_js(mut event_queue: ResMut<ToJsEventQueue>) {
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
        render_app.init_resource::<ToJsEventQueue>();

        // Register systems
        render_app
            .add_systems(
                ExtractSchedule,
                (
                    extract_system_log,
                    extract_rectangle_corner_mixin,
                    extract_children_mixin,
                    extract_layout_mixin,
                    extract_composition_mixin,
                    extract_blend_mixin,
                    extract_path_mixin,
                ),
            )
            .add_systems(
                RenderSchedule,
                (
                    render_system_log,
                    prepare_render_changes.in_set(RenderSet::Prepare),
                    queue_render_changes.in_set(RenderSet::Queue),
                    forward_render_changes_to_js.in_set(RenderSet::Render),
                ),
            );
    }
}
