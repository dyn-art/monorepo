use std::collections::HashSet;

use bevy_ecs::{
    change_detection::DetectChanges,
    entity::Entity,
    query::With,
    system::{Local, Query, Res, ResMut},
};
use dyn_composition::core::modules::{
    composition::resources::composition::CompositionRes,
    interactive_composition::resources::{HandleSide, InteractionMode, InteractiveCompositionRes},
    node::components::states::Selected,
};

use crate::core::events::{
    output_event::{
        CompositionChange, CompositionChangeEvent, CursorChangeEvent, CursorForFrontend,
        InteractionModeChangeEvent, InteractionModeForFrontend, OutputEvent, SelectionChangeEvent,
    },
    output_event_queue::OutputEventQueueRes,
};

pub fn check_composition_changes(
    mut output_event_queue: ResMut<OutputEventQueueRes>,
    composition: Res<CompositionRes>,
) {
    // TODO: Can be granulated to avoid sending too much data, but for now, that's good enough
    if composition.is_changed() {
        output_event_queue.push_event(OutputEvent::CompositionChange(CompositionChangeEvent {
            change: CompositionChange {
                width: composition.width,
                height: composition.height,
                view_box: composition.view_box,
                root_node: composition.root_node,
            },
        }));
    }
}

pub fn check_selection_changes(
    mut output_event_queue: ResMut<OutputEventQueueRes>,
    mut last_selected: Local<HashSet<Entity>>,
    selected_query: Query<Entity, With<Selected>>,
) {
    let current_selected: HashSet<Entity> = selected_query.iter().collect();

    // Check if the set of selected entities has changed
    if *last_selected != current_selected {
        output_event_queue.push_event(OutputEvent::SelectionChange(SelectionChangeEvent {
            selected: current_selected.clone().into_iter().collect(),
        }));

        // Update the local tracking set
        *last_selected = current_selected;
    }
}

pub fn check_interaction_mode_changes(
    mut output_event_queue: ResMut<OutputEventQueueRes>,
    interactive_composition: Res<InteractiveCompositionRes>,
    mut last_interaction_mode: Local<InteractionModeForFrontend>,
) {
    if interactive_composition.is_changed() {
        // Map InteractionMode to InteractionModeChange.
        // Node: Not passing InteractionMode itself as it contains inrelevant data
        // that would trigger unnecessary re-renders
        let current_interaction_mode = match interactive_composition.interaction_mode {
            InteractionMode::None => InteractionModeForFrontend::None,
            InteractionMode::Pressing { .. } => InteractionModeForFrontend::Pressing,
            InteractionMode::Translating { .. } => InteractionModeForFrontend::Translating,
            InteractionMode::Resizing { .. } => InteractionModeForFrontend::Resizing,
            InteractionMode::Rotating { .. } => InteractionModeForFrontend::Rotating,
            InteractionMode::Dragging { .. } => InteractionModeForFrontend::Dragging,
        };

        // Check whether the interaction mode has changed
        if *last_interaction_mode != current_interaction_mode {
            output_event_queue.push_event(OutputEvent::InteractionModeChange(
                InteractionModeChangeEvent {
                    interaction_mode: current_interaction_mode.clone(),
                },
            ));

            // Update the local tracking of the interaction mode
            *last_interaction_mode = current_interaction_mode;
        }
    }
}

pub fn check_cursor_changes(
    mut output_event_queue: ResMut<OutputEventQueueRes>,
    interactive_composition: Res<InteractiveCompositionRes>,
    mut last_cursor: Local<CursorForFrontend>,
) {
    if interactive_composition.is_changed() {
        let current_cursor = match interactive_composition.interaction_mode {
            InteractionMode::Resizing {
                corner,
                rotation_in_degrees,
                ..
            } => {
                let mut cursor_rotation = 0.0;

                match corner {
                    _ if corner == (HandleSide::Top as u8 | HandleSide::Left as u8) => {
                        cursor_rotation = -135.0;
                    }
                    _ if corner == HandleSide::Top as u8 => {
                        cursor_rotation = 90.0;
                    }
                    _ if corner == (HandleSide::Top as u8 | HandleSide::Right as u8) => {
                        cursor_rotation = 135.0;
                    }
                    _ if corner == HandleSide::Right as u8 => {
                        cursor_rotation = 0.0;
                    }
                    _ if corner == (HandleSide::Bottom as u8 | HandleSide::Right as u8) => {
                        cursor_rotation = -135.0;
                    }
                    _ if corner == HandleSide::Bottom as u8 => {
                        cursor_rotation = 90.0;
                    }
                    _ if corner == (HandleSide::Bottom as u8 | HandleSide::Left as u8) => {
                        cursor_rotation = 135.0;
                    }
                    _ if corner == HandleSide::Left as u8 => {
                        cursor_rotation = 0.0;
                    }
                    _ => {}
                }

                cursor_rotation -= rotation_in_degrees;

                CursorForFrontend::Resize {
                    rotation_in_degrees: cursor_rotation,
                }
            }
            InteractionMode::Rotating {
                corner,
                rotation_in_degrees,
                ..
            } => {
                let mut cursor_rotation = 0.0;

                match corner {
                    _ if corner == (HandleSide::Top as u8 | HandleSide::Left as u8) => {
                        cursor_rotation = 90.0;
                    }
                    _ if corner == (HandleSide::Top as u8 | HandleSide::Right as u8) => {
                        cursor_rotation = 180.0;
                    }
                    _ if corner == (HandleSide::Bottom as u8 | HandleSide::Right as u8) => {
                        cursor_rotation = 270.0;
                    }
                    _ if corner == (HandleSide::Bottom as u8 | HandleSide::Left as u8) => {
                        cursor_rotation = 0.0;
                    }
                    _ => {}
                }

                cursor_rotation -= rotation_in_degrees;

                CursorForFrontend::Rotate {
                    rotation_in_degrees: cursor_rotation,
                }
            }
            _ => CursorForFrontend::Default,
        };

        // Check whether the cursor has changed
        if *last_cursor != current_cursor {
            output_event_queue.push_event(OutputEvent::CursorChange(CursorChangeEvent {
                cursor: current_cursor.clone(),
            }));

            // Update the local tracking of the cursor
            *last_cursor = current_cursor;
        }
    }
}
