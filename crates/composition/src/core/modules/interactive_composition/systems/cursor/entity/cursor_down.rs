use bevy_ecs::{
    entity::Entity,
    event::EventReader,
    query::{With, Without},
    system::{Commands, Query, ResMut},
};
use glam::Vec2;

use crate::core::modules::{
    interactive_composition::{
        events::{CursorDownOnEntity, MouseButton},
        resources::{InteractionMode, InteractiveCompositionRes},
    },
    node::components::{
        states::{Locked, Selected},
        types::{Frame, Node, Root},
    },
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
    mut interactive_composition: ResMut<InteractiveCompositionRes>,
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
    node_query_with_selected: Query<Entity, (With<Node>, Without<Locked>, Without<Root>)>,
) {
    let raycast_entities: Vec<(Entity, Vec2)> = event_reader
        .read()
        .filter_map(|event| {
            if event.button == MouseButton::Left {
                Some((event.entity, event.position))
            } else {
                None
            }
        })
        .collect();
    if raycast_entities.is_empty() {
        return;
    }

    // Find the next best node to select
    let selected_entity = select_next_node(
        &raycast_entities,
        &frame_query,
        &node_query,
        &node_query_with_selected,
    );

    // Select new entity if it's not already selected
    if let Some((entity, pos, is_new)) = selected_entity {
        // Mark node as selected
        if is_new {
            commands.entity(entity).insert(Selected);

            #[cfg(feature = "tracing")]
            log::info!("Selected Entity {:#?} at {:#?}", entity, pos);
        }

        interactive_composition.interaction_mode = InteractionMode::Translating {
            origin: pos,
            current: pos,
        };
    }

    // Unselect previously selected nodes that are no longer selected
    selected_nodes_query.for_each(|entity| {
        if selected_entity.map_or(true, |(selected, _, _)| selected != entity) {
            commands.entity(entity).remove::<Selected>();
            #[cfg(feature = "tracing")]
            log::info!("Unselected Entity: {:#?}", entity);
        }
    });
}

fn select_next_node(
    raycast_entities: &Vec<(Entity, Vec2)>,
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
    node_query_with_selected: &Query<Entity, (With<Node>, Without<Locked>, Without<Root>)>,
) -> Option<(Entity, Vec2, bool)> {
    // First, attempt to find a non-Frame, non-locked, non-selected node
    raycast_entities
        .iter()
        .rev()
        .find_map(|&(entity, pos)| {
            if node_query.contains(entity) {
                Some((entity, pos, true))
            } else {
                None
            }
        })
        // If no such node is found, try to find a frame that is not a root,
        // not selected and not locked
        .or_else(|| {
            raycast_entities.iter().rev().find_map(|&(entity, pos)| {
                if frame_query.contains(entity) {
                    Some((entity, pos, true))
                } else {
                    None
                }
            })
        })
        // If still no new node found, select already selected node
        .or_else(|| {
            raycast_entities.iter().rev().find_map(|&(entity, pos)| {
                if node_query_with_selected.contains(entity) {
                    Some((entity, pos, false))
                } else {
                    None
                }
            })
        })
}
