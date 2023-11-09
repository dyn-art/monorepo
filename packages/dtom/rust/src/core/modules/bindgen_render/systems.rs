use bevy_ecs::{
    component::Component,
    entity::Entity,
    query::{Changed, With},
    system::{Query, ResMut},
};
use bevy_hierarchy::Parent;
use dyn_bevy_render_skeleton::extract_param::Extract;
use dyn_composition::core::modules::node::components::types::Node;
use log::info;

use crate::core::events::output_event::{OutputEvent, OutputEventQueue, RenderUpdateEvent};

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

pub fn queue_render_changes(
    mut changed: ResMut<ChangedComponents>,
    mut output_event_queue: ResMut<OutputEventQueue>,
    parent_query: Query<&Parent>,
) {
    if !changed.changes.is_empty() {
        changed
            .changes
            .drain()
            .into_iter()
            .for_each(|(entity, (node_type, changes))| {
                // Attempt to get the parent entity ID
                let mut parent_id: Option<Entity> = None;
                if let Ok(parent) = parent_query.get(entity) {
                    parent_id = Some(parent.get());
                }

                output_event_queue.push_event(OutputEvent::RenderUpdate(RenderUpdateEvent {
                    parent: parent_id,
                    entity,
                    node_type,
                    changes,
                }));
            });
    }
}
