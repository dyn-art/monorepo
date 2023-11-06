use bevy_ecs::{
    component::Component,
    entity::Entity,
    query::{Changed, With},
    system::{Commands, Query, ResMut},
};
use dyn_bevy_render_skeleton::extract_param::Extract;
use dyn_composition::core::modules::node::components::types::Node;

use crate::core::modules::output_event::{resources::OutputEventQueue, OutputEvent};

use super::{resources::ChangedComponents, ToRenderChange};

pub fn extract_mixin_generic<T: Component + Clone + ToRenderChange>(
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

pub fn prepare_render_changes(mut commands: Commands, mut changed: ResMut<ChangedComponents>) {
    // TODO:
    // Prepare SVG path based on PathMixin
    // and other stuff that needs to be prepared
    // or not because its SVG specific?
}

pub fn queue_render_changes(
    mut changed: ResMut<ChangedComponents>,
    mut output_event_queue: ResMut<OutputEventQueue>,
) {
    if !changed.changes.is_empty() {
        changed
            .changes
            .drain()
            .into_iter()
            .for_each(|(entity, (node_type, changes))| {
                output_event_queue.push_event(OutputEvent::RenderUpdate {
                    entity,
                    node_type,
                    changes,
                });
            });
    }
}
