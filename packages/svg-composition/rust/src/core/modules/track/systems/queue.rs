use std::mem::take;

use bevy_ecs::system::ResMut;

use crate::core::{
    events::{
        output_event::{OutputEvent, TrackUpdateEvent},
        output_event_queue::OutputEventQueueRes,
    },
    modules::track::resources::changed_components::ChangedComponentsRes,
};

pub fn queue_tracked_changes(
    mut changed: ResMut<ChangedComponentsRes>,
    mut output_event_queue: ResMut<OutputEventQueueRes>,
) {
    let mut changed_entities = take(&mut changed.changed_entities);
    for (key, value) in changed_entities.drain() {
        output_event_queue.push_event(OutputEvent::TrackUpdate(TrackUpdateEvent {
            id: key,
            updates: value,
        }))
    }
}
