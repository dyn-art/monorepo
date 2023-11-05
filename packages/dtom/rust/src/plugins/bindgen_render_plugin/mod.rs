use bevy_app::{App, Plugin};
use bevy_ecs::{
    component::Component,
    entity::Entity,
    query::{Changed, With},
    schedule::IntoSystemConfigs,
    system::{Commands, Query, ResMut, Resource},
};
use bevy_utils::HashMap;
use dyn_bevy_render_skeleton::{
    extract_param::Extract, ExtractSchedule, Render, RenderApp, RenderSet,
};
use serde::Serialize;
use specta::Type;

use crate::{
    bindgen::event_queue::to_js_event_queue::{
        forward_events_to_js, BaseToJsEvent, ToJsEventQueue,
    },
    core::node::{
        mixins::{
            BlendMixin, ChildrenMixin, CompositionMixin, LayoutMixin, ParentMixin, PathMixin,
            RectangleCornerMixin,
        },
        types::{Node, NodeType},
    },
};

#[derive(Debug, Serialize, Clone, Type)]
pub enum BindgenRenderToJsEvent {
    RenderUpdate {
        entity: Entity,
        node_type: NodeType,
        changes: Vec<RenderChange>,
    },
}

impl BaseToJsEvent for BindgenRenderToJsEvent {}

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
    mut event_queue: ResMut<ToJsEventQueue<BindgenRenderToJsEvent>>,
) {
    if !changed.changes.is_empty() {
        changed
            .changes
            .drain()
            .into_iter()
            .for_each(|(entity, (node_type, changes))| {
                event_queue.push_event(BindgenRenderToJsEvent::RenderUpdate {
                    entity,
                    node_type,
                    changes,
                });
            });
    }
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
        render_app.init_resource::<ToJsEventQueue<BindgenRenderToJsEvent>>();

        // Register systems
        render_app
            .add_systems(
                ExtractSchedule,
                (
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
                Render,
                (
                    prepare_render_changes.in_set(RenderSet::Prepare),
                    queue_render_changes.in_set(RenderSet::Queue),
                    forward_events_to_js::<BindgenRenderToJsEvent>.in_set(RenderSet::Render),
                ),
            );
    }
}
