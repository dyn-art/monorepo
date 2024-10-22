use crate::{
    events::SvgArbOutputEvent,
    modules::watch::{
        events::{
            ArtboardChangeOutputEvent, Cursor, CursorChangeOutputEvent,
            InteractionModeChangeOutputEvent, InteractionModeLabel,
            InteractionToolChangeOutputEvent, SelectionChangeOutputEvent,
            WatchedEntityChangesOutputEvent,
        },
        resources::{
            changed_components::ChangedComponentsRes, output_event_sender::OutputEventSenderRes,
        },
    },
};
use bevy_ecs::{
    change_detection::DetectChanges,
    entity::Entity,
    query::With,
    system::{Local, Query, Res, ResMut},
};
use dyn_arb_core::resources::artboard::ArtboardRes;
use dyn_arb_interaction::{
    components::Selected,
    resources::arb_interaction::{ArbInteractionRes, HandleSide, InteractionMode, InteractionTool},
};
use std::collections::HashSet;

pub fn queue_changed_components(
    mut changed_components_res: ResMut<ChangedComponentsRes>,
    output_event_sender_res: ResMut<OutputEventSenderRes>,
) {
    for (entity, changes) in changed_components_res.drain() {
        output_event_sender_res.push_event(SvgArbOutputEvent::WatchedEntityChange(
            WatchedEntityChangesOutputEvent { entity, changes },
        ))
    }
}

pub fn queue_artboard_changes(
    output_event_sender_res: ResMut<OutputEventSenderRes>,
    arb_res: Res<ArtboardRes>,
) {
    // TODO: Can be granulated to avoid sending too much data, but for now, that's good enough
    if arb_res.is_changed() {
        output_event_sender_res.push_event(SvgArbOutputEvent::ArtboardChange(
            ArtboardChangeOutputEvent {
                size: arb_res.size,
                viewport: arb_res.viewport,
            },
        ))
    }
}

pub fn queue_selected_entities_changes(
    output_event_sender_res: ResMut<OutputEventSenderRes>,
    mut last_selected: Local<HashSet<Entity>>,
    selected_query: Query<Entity, With<Selected>>,
) {
    let current_selected: HashSet<Entity> = selected_query.iter().collect();

    // Check whether the set of selected entities has changed
    if *last_selected != current_selected {
        output_event_sender_res.push_event(SvgArbOutputEvent::SelectionChange(
            SelectionChangeOutputEvent {
                selected_entities: (&current_selected).into_iter().copied().collect(),
            },
        ));

        *last_selected = current_selected;
    }
}

pub fn queue_interaction_mode_changes(
    output_event_sender_res: ResMut<OutputEventSenderRes>,
    arb_interaction_res: Res<ArbInteractionRes>,
    mut last_interaction_mode: Local<InteractionModeLabel>,
) {
    if arb_interaction_res.is_changed() {
        let current_interaction_mode: InteractionModeLabel =
            (&arb_interaction_res.interaction_mode).into();

        // Check whether the interaction mode has changed
        if *last_interaction_mode != current_interaction_mode {
            output_event_sender_res.push_event(SvgArbOutputEvent::InteractionModeChange(
                InteractionModeChangeOutputEvent {
                    interaction_mode: current_interaction_mode,
                },
            ));

            *last_interaction_mode = current_interaction_mode;
        }
    }
}

pub fn queue_interaction_tool_changes(
    output_event_sender_res: ResMut<OutputEventSenderRes>,
    arb_interaction_res: Res<ArbInteractionRes>,
    mut last_interaction_tool: Local<InteractionTool>,
) {
    if arb_interaction_res.is_changed() {
        let current_interaction_tool = arb_interaction_res.interaction_tool;

        // Check whether the interaction tool has changed
        if *last_interaction_tool != current_interaction_tool {
            output_event_sender_res.push_event(SvgArbOutputEvent::InteractionToolChange(
                InteractionToolChangeOutputEvent {
                    interaction_tool: current_interaction_tool,
                },
            ));

            *last_interaction_tool = current_interaction_tool;
        }
    }
}

pub fn queue_cursor_changes(
    output_event_sender_res: ResMut<OutputEventSenderRes>,
    arb_interaction_res: Res<ArbInteractionRes>,
    mut last_cursor: Local<Cursor>,
) {
    if arb_interaction_res.is_changed() {
        let current_cursor = match (
            arb_interaction_res.interaction_mode,
            arb_interaction_res.interaction_tool,
        ) {
            (
                InteractionMode::Resizing {
                    corner,
                    rotation_deg,
                    ..
                },
                _,
            ) => {
                let mut cursor_rotation = 0.0;

                match corner {
                    _ if corner == (HandleSide::Top as u8 | HandleSide::Left as u8) => {
                        cursor_rotation = 135.0;
                    }
                    _ if corner == HandleSide::Top as u8 => {
                        cursor_rotation = 0.0;
                    }
                    _ if corner == (HandleSide::Top as u8 | HandleSide::Right as u8) => {
                        cursor_rotation = -135.0;
                    }
                    _ if corner == HandleSide::Right as u8 => {
                        cursor_rotation = 90.0;
                    }
                    _ if corner == (HandleSide::Bottom as u8 | HandleSide::Right as u8) => {
                        cursor_rotation = 135.0;
                    }
                    _ if corner == HandleSide::Bottom as u8 => {
                        cursor_rotation = 0.0;
                    }
                    _ if corner == (HandleSide::Bottom as u8 | HandleSide::Left as u8) => {
                        cursor_rotation = -135.0;
                    }
                    _ if corner == HandleSide::Left as u8 => {
                        cursor_rotation = 90.0;
                    }
                    _ => {}
                }

                cursor_rotation += rotation_deg;

                Cursor::Resize {
                    rotation_deg: cursor_rotation,
                }
            }
            (
                InteractionMode::Rotating {
                    corner,
                    rotation_deg,
                    ..
                },
                _,
            ) => {
                let mut cursor_rotation = 0.0;

                match corner {
                    _ if corner == (HandleSide::Top as u8 | HandleSide::Left as u8) => {
                        cursor_rotation = 0.0;
                    }
                    _ if corner == (HandleSide::Top as u8 | HandleSide::Right as u8) => {
                        cursor_rotation = 90.0;
                    }
                    _ if corner == (HandleSide::Bottom as u8 | HandleSide::Right as u8) => {
                        cursor_rotation = 180.0;
                    }
                    _ if corner == (HandleSide::Bottom as u8 | HandleSide::Left as u8) => {
                        cursor_rotation = -90.0;
                    }
                    _ => {}
                }

                cursor_rotation += rotation_deg;

                Cursor::Rotate {
                    rotation_deg: cursor_rotation,
                }
            }
            (InteractionMode::Dragging { .. }, _) => Cursor::Grabbing,
            (_, InteractionTool::Shape { .. }) => Cursor::Crosshair,
            (_, InteractionTool::Select) => Cursor::Default,
        };

        // Check whether the cursor has changed
        if *last_cursor != current_cursor {
            output_event_sender_res.push_event(SvgArbOutputEvent::CursorChange(
                CursorChangeOutputEvent {
                    cursor: current_cursor,
                },
            ));

            // Update the local tracking of the cursor
            *last_cursor = current_cursor;
        }
    }
}
