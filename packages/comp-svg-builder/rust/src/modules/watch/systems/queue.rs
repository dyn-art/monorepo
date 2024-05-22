use crate::{
    events::SvgCompOutputEvent,
    modules::watch::{
        events::{
            CompositionChangeOutputEvent, Cursor, CursorChangeOutputEvent,
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
use dyn_comp_core::resources::composition::CompositionRes;
use dyn_comp_interaction::{
    components::Selected,
    resources::comp_interaction::{
        CompInteractionRes, HandleSide, InteractionMode, InteractionTool,
    },
};
use std::collections::HashSet;

pub fn queue_changed_components(
    mut changed_components_res: ResMut<ChangedComponentsRes>,
    output_event_sender_res: ResMut<OutputEventSenderRes>,
) {
    for (entity, changes) in changed_components_res.drain() {
        output_event_sender_res.push_event(SvgCompOutputEvent::WatchedEntityChange(
            WatchedEntityChangesOutputEvent { entity, changes },
        ))
    }
}

pub fn queue_composition_changes(
    output_event_sender_res: ResMut<OutputEventSenderRes>,
    comp_res: Res<CompositionRes>,
) {
    // TODO: Can be granulated to avoid sending too much data, but for now, that's good enough
    if comp_res.is_changed() {
        output_event_sender_res.push_event(SvgCompOutputEvent::CompositionChange(
            CompositionChangeOutputEvent {
                size: comp_res.size,
                viewport: comp_res.viewport,
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
        output_event_sender_res.push_event(SvgCompOutputEvent::SelectionChange(
            SelectionChangeOutputEvent {
                selected_entities: (&current_selected).into_iter().copied().collect(),
            },
        ));

        *last_selected = current_selected;
    }
}

pub fn queue_interaction_mode_changes(
    output_event_sender_res: ResMut<OutputEventSenderRes>,
    comp_interaction_res: Res<CompInteractionRes>,
    mut last_interaction_mode: Local<InteractionModeLabel>,
) {
    if comp_interaction_res.is_changed() {
        let current_interaction_mode: InteractionModeLabel =
            (&comp_interaction_res.interaction_mode).into();

        // Check whether the interaction mode has changed
        if *last_interaction_mode != current_interaction_mode {
            output_event_sender_res.push_event(SvgCompOutputEvent::InteractionModeChange(
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
    comp_interaction_res: Res<CompInteractionRes>,
    mut last_interaction_tool: Local<InteractionTool>,
) {
    if comp_interaction_res.is_changed() {
        let current_interaction_tool = comp_interaction_res.interaction_tool;

        // Check whether the interaction tool has changed
        if *last_interaction_tool != current_interaction_tool {
            output_event_sender_res.push_event(SvgCompOutputEvent::InteractionToolChange(
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
    comp_interaction_res: Res<CompInteractionRes>,
    mut last_cursor: Local<Cursor>,
) {
    if comp_interaction_res.is_changed() {
        let current_cursor = match (
            comp_interaction_res.interaction_mode,
            comp_interaction_res.interaction_tool,
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
            output_event_sender_res.push_event(SvgCompOutputEvent::CursorChange(
                CursorChangeOutputEvent {
                    cursor: current_cursor,
                },
            ));

            // Update the local tracking of the cursor
            *last_cursor = current_cursor;
        }
    }
}
