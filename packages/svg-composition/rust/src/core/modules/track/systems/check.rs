use std::collections::HashSet;

use bevy_ecs::{
    change_detection::DetectChanges,
    entity::Entity,
    query::With,
    system::{Local, Query, Res, ResMut},
};
use dyn_composition::core::modules::{
    interactive_composition::resources::{InteractionMode, InteractiveCompositionRes},
    node::components::states::Selected,
};

use crate::core::events::{
    output_event::{
        InteractionModeChangeEvent, OutputEvent, RawInteractionMode, SelectionChangeEvent,
    },
    output_event_queue::OutputEventQueueRes,
};

pub fn check_selection_changes(
    mut output_event_queue: ResMut<OutputEventQueueRes>,
    mut last_selected: Local<HashSet<Entity>>,
    query: Query<Entity, With<Selected>>,
) {
    let current_selected: HashSet<Entity> = query.iter().collect();

    // Check if the set of selected entities has changed
    if *last_selected != current_selected {
        output_event_queue.push_event(OutputEvent::SelectionChange(SelectionChangeEvent {
            selected: current_selected.clone().into_iter().collect(),
        }));

        // Update the local tracking set
        *last_selected = current_selected;
    }
}

pub fn check_interactive_composition_changes(
    mut output_event_queue: ResMut<OutputEventQueueRes>,
    mut last_raw_interaction_mode: Local<RawInteractionMode>,
    interactive_composition: Res<InteractiveCompositionRes>,
) {
    if interactive_composition.is_changed() {
        let current_raw_interaction_mode = match interactive_composition.interaction_mode {
            InteractionMode::None => RawInteractionMode::None,
            InteractionMode::Pressing { .. } => RawInteractionMode::Pressing,
            InteractionMode::Translating { .. } => RawInteractionMode::Translating,
            InteractionMode::Resizing { .. } => RawInteractionMode::Resizing,
        };

        // Check whether the interaction mode has changed
        if *last_raw_interaction_mode != current_raw_interaction_mode {
            output_event_queue.push_event(OutputEvent::InteractionModeChange(
                InteractionModeChangeEvent {
                    interaction_mode: current_raw_interaction_mode.clone(),
                },
            ));

            // Update the local tracking of the interaction mode
            *last_raw_interaction_mode = current_raw_interaction_mode;
        }
    }
}
