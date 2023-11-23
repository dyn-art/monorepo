use std::mem::take;

use bevy_ecs::{
    component::Component,
    entity::Entity,
    query::Changed,
    system::{Query, Res, ResMut},
};

use crate::core::{
    events::{
        output_event::{OutputEvent, TrackUpdateEvent},
        output_event_queue::OutputEventQueue,
    },
    mixin_change::ToMixinChange,
};

use super::resources::{
    changed_components::ChangedComponents, trackable_entities::TrackedEntities,
};

pub fn track_changes<C: Component + ToMixinChange>(
    mut changed: ResMut<ChangedComponents>,
    tracked_entities: Res<TrackedEntities>,
    query: Query<(Entity, &C), Changed<C>>,
) {
    for (entity, component) in query.iter() {
        if tracked_entities.entities.contains_key(&entity) {
            let changed_component = changed
                .changed_entities
                .entry(entity)
                .or_insert_with(Vec::new);
            changed_component.push(component.to_mixin_change());
        }
    }
}

pub fn queue_tracked_changes(
    mut changed: ResMut<ChangedComponents>,
    mut output_event_queue: ResMut<OutputEventQueue>,
) {
    let mut changed_entities = take(&mut changed.changed_entities);
    for (key, value) in changed_entities.drain() {
        output_event_queue.push_event(OutputEvent::TrackUpdate(TrackUpdateEvent {
            id: key,
            updates: value,
        }))
    }
}
