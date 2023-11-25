use bevy_ecs::{
    entity::Entity,
    event::EventReader,
    query::{With, Without},
    system::{Commands, Query},
};
use log::info;

use crate::core::modules::node::components::{
    states::{Locked, Selected},
    types::{Frame, Node, Root},
};

use super::events::{
    CursorDownOnEntity, CursorEnteredComposition, CursorExitedComposition, CursorMovedOnComposition,
};

// Logs:
// INFO: Start: handle_cursor_down_on_entity_event
// INFO: handle_cursor_down_on_entity_event: 2v0 -> Paint
// INFO: handle_cursor_down_on_entity_event: 1v0 -> Rectangle
// INFO: handle_cursor_down_on_entity_event: 0v0 -> Frame
// INFO: End: handle_cursor_down_on_entity_event
//
// Frame (0v0) -> Rectangle (1v0) -> Paint (2v0)

pub fn handle_cursor_down_on_entity_event(
    mut event_reader: EventReader<CursorDownOnEntity>,
    mut commands: Commands,
    selected_nodes_query: Query<Entity, With<Selected>>,
    frame_query: Query<
        Entity,
        (
            With<Frame>,
            Without<Locked>,
            Without<Root>,
            Without<Selected>,
        ),
    >,
    node_query: Query<
        Entity,
        (
            With<Node>,
            Without<Locked>,
            Without<Frame>,
            Without<Selected>,
        ),
    >,
) {
    let raycast_entities: Vec<Entity> = event_reader.read().map(|event| event.entity).collect();
    if raycast_entities.is_empty() {
        return;
    }

    // Find the next best node to select
    let selected_entity = select_next_node(&raycast_entities, &frame_query, &node_query);

    // Select new entity if it's not already selected
    if let Some(entity) = selected_entity {
        commands.entity(entity).insert(Selected);
        #[cfg(trace)]
        info!("Selected Entity: {:#?}", entity);
    }

    // Unselect previously selected nodes that are no longer selected
    selected_nodes_query.for_each(|entity| {
        if selected_entity.map_or(true, |selected| selected != entity) {
            commands.entity(entity).remove::<Selected>();
            #[cfg(trace)]
            info!("Unselected Entity: {:#?}", entity);
        }
    });
}

fn select_next_node(
    raycast_entities: &Vec<Entity>,
    frame_query: &Query<
        Entity,
        (
            With<Frame>,
            Without<Locked>,
            Without<Root>,
            Without<Selected>,
        ),
    >,
    node_query: &Query<
        Entity,
        (
            With<Node>,
            Without<Locked>,
            Without<Frame>,
            Without<Selected>,
        ),
    >,
) -> Option<Entity> {
    // First, attempt to find a non-Frame, non-Locked, non-Selected Node
    raycast_entities
        .iter()
        .rev()
        .find_map(|&entity| {
            if node_query.contains(entity) {
                Some(entity)
            } else {
                None
            }
        })
        // If no such Node is found, try to find a Frame that is not a Root,
        // not Selected and not Locked
        .or_else(|| {
            raycast_entities.iter().rev().find_map(|&entity| {
                if frame_query.contains(entity) {
                    Some(entity)
                } else {
                    None
                }
            })
        })
}

pub fn handle_cursor_moved_on_composition(mut event_reader: EventReader<CursorMovedOnComposition>) {
    for event in event_reader.read() {
        let CursorMovedOnComposition { position } = event;
        // info!("handle_cursor_moved_on_composition: {:#?}", position);
    }
}

pub fn handle_cursor_entered_composition(mut event_reader: EventReader<CursorEnteredComposition>) {
    for event in event_reader.read() {
        info!("handle_cursor_entered_composition");
    }
}

pub fn handle_cursor_exited_composition(mut event_reader: EventReader<CursorExitedComposition>) {
    for event in event_reader.read() {
        info!("handle_cursor_exited_composition");
    }
}
