use bevy_app::{App, Plugin};
use bevy_ecs::{
    component::Component,
    entity::Entity,
    query::{Changed, With},
    schedule::IntoSystemConfigs,
    system::{Commands, Query, ResMut, Resource},
};
use bevy_utils::HashMap;
use serde::Serialize;
use specta::Type;

use crate::{
    bindgen::{
        event_queue::to_js_event_queue::{ToJsEvent, ToJsEventQueue},
        js_bindings,
    },
    core::node::{
        mixins::{
            BlendMixin, ChildrenMixin, CompositionMixin, LayoutMixin, ParentMixin, PathMixin,
            RectangleCornerMixin,
        },
        types::{Node, NodeType},
    },
};

use super::render_plugin::{
    extract_param::Extract, ExtractSchedule, RenderApp, RenderSchedule, RenderSet,
};

#[derive(Serialize, Clone, Debug, Type)]
pub enum RenderChange {
    RectangleCorner(RectangleCornerMixin),
    Children(ChildrenMixin),
    Layout(LayoutMixin),
    Composition(CompositionMixin),
    Blend(BlendMixin),
    Path(PathMixin),
    ParentMixin(ParentMixin),
}

pub trait ToRenderChange {
    fn to_render_change(&self) -> RenderChange;
}

impl ToRenderChange for ChildrenMixin {
    fn to_render_change(&self) -> RenderChange {
        RenderChange::Children(self.clone())
    }
}

impl ToRenderChange for LayoutMixin {
    fn to_render_change(&self) -> RenderChange {
        RenderChange::Layout(self.clone())
    }
}

impl ToRenderChange for CompositionMixin {
    fn to_render_change(&self) -> RenderChange {
        RenderChange::Composition(self.clone())
    }
}

impl ToRenderChange for BlendMixin {
    fn to_render_change(&self) -> RenderChange {
        RenderChange::Blend(self.clone())
    }
}

impl ToRenderChange for PathMixin {
    fn to_render_change(&self) -> RenderChange {
        RenderChange::Path(self.clone())
    }
}

impl ToRenderChange for RectangleCornerMixin {
    fn to_render_change(&self) -> RenderChange {
        RenderChange::RectangleCorner(self.clone())
    }
}

impl ToRenderChange for ParentMixin {
    fn to_render_change(&self) -> RenderChange {
        RenderChange::ParentMixin(self.clone())
    }
}

// =============================================================================
// Ressources
// =============================================================================

#[derive(Resource, Default, Debug)]
pub struct ChangedComponents {
    changes: HashMap<Entity, (NodeType, Vec<RenderChange>)>,
}

// =============================================================================
// Systems
// =============================================================================

fn extract_mixin_generic<T: Component + Clone + ToRenderChange>(
    mut changed: ResMut<ChangedComponents>,
    query: Extract<Query<(Entity, &Node, &T), (With<Node>, Changed<T>)>>,
) {
    query.for_each(|(entity, node, mixin)| {
        let (_, change_set) = changed
            .changes
            .entry(entity)
            .or_insert((node.node_type.clone(), vec![]));
        change_set.push(mixin.to_render_change());
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
            .for_each(|(entity, (node_type, changes))| {
                event_queue.push_event(ToJsEvent::RenderUpdate {
                    entity: entity.index(),
                    node_type,
                    changes,
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
                    extract_mixin_generic::<RectangleCornerMixin>,
                    extract_mixin_generic::<ChildrenMixin>,
                    extract_mixin_generic::<LayoutMixin>,
                    extract_mixin_generic::<CompositionMixin>,
                    extract_mixin_generic::<BlendMixin>,
                    extract_mixin_generic::<PathMixin>,
                    extract_mixin_generic::<ParentMixin>,
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
