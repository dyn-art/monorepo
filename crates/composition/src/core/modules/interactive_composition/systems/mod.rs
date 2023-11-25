use bevy_ecs::{
    entity::Entity,
    event::EventReader,
    query::With,
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
    mut selected_nodes_query: Query<Entity, With<Selected>>,
    frame_query: Query<Entity, With<Frame>>,
    locked_query: Query<Entity, With<Locked>>,
    node_query: Query<Entity, With<Node>>,
    root_node_query: Query<Entity, With<Root>>,
) {
    let raycast_entities: Vec<Entity> = event_reader.read().map(|event| event.entity).collect();
    let mut selected_node_entities: Vec<Entity> = selected_nodes_query.iter_mut().collect();

    if raycast_entities.len() == 0 {
        return;
    }

    // Iterate through raycast entities and determine the next selection
    if let Some(next_entity) = select_next_node(
        &raycast_entities,
        &frame_query,
        &locked_query,
        &node_query,
        &root_node_query,
    ) {
        if !selected_node_entities.contains(&next_entity) {
            commands.entity(next_entity).insert(Selected);
            selected_node_entities.push(next_entity);
            info!("Selected Entity: {:#?}", next_entity);
        }
    }

    // Unselect previously selected nodes if they are not in the raycast entities
    for entity in selected_nodes_query.iter_mut() {
        if !raycast_entities.contains(&entity) {
            commands.entity(entity).remove::<Selected>();
            info!("Unselected Entity: {:#?}", entity);
        }
    }
}

fn select_next_node(
    raycast_entities: &[Entity],
    frame_query: &Query<Entity, With<Frame>>,
    locked_query: &Query<Entity, With<Locked>>,
    node_query: &Query<Entity, With<Node>>,
    root_node_query: &Query<Entity, With<Root>>,
) -> Option<Entity> {
    for &entity in raycast_entities.iter().rev() {
        if node_query.contains(entity)
            && !frame_query.contains(entity)
            && !locked_query.contains(entity)
        {
            return Some(entity);
        }
    }

    // If only Frames are left or all nodes are locked, select the deepest Frame
    return raycast_entities
        .iter()
        .rev()
        .find(|&&entity| frame_query.contains(entity) && !root_node_query.contains(entity))
        .copied();
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
