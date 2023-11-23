use std::mem::take;

use bevy_ecs::system::ResMut;

use crate::core::{
    events::{
        output_event::{OutputEvent, TrackUpdateEvent},
        output_event_queue::OutputEventQueue,
    },
    modules::track::resources::changed_components::ChangedComponents,
};

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
