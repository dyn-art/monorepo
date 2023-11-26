use bevy_ecs::{
    entity::Entity,
    event::EventReader,
    query::{With, Without},
    system::{Commands, Query, ResMut},
};
use glam::{Mat3, Vec2};
use log::info;

use crate::core::modules::{
    interactive_composition::resources::InteractionMode,
    node::components::{
        mixins::RelativeTransformMixin,
        states::{Locked, Selected},
        types::{Frame, Node, Root},
    },
};

use super::{
    events::{
        CursorDownOnComposition, CursorDownOnEntity, CursorEnteredComposition,
        CursorExitedComposition, CursorMovedOnComposition, CursorUpOnComposition,
    },
    resources::InteractiveCompositionRes,
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
) {
    let raycast_entities: Vec<(Entity, Vec2)> = event_reader
        .read()
        .map(|event| (event.entity, event.position))
        .collect();
    if raycast_entities.is_empty() {
        return;
    }

    // Find the next best node to select
    let selected_entity = select_next_node(&raycast_entities, &frame_query, &node_query);

    // Select new entity if it's not already selected
    if let Some((entity, pos)) = selected_entity {
        commands.entity(entity).insert(Selected);

        interactive_composition.interaction_mode = InteractionMode::Translating {
            origin: pos,
            current: pos,
        };

        #[cfg(feature = "trace")]
        info!("Selected Entity {:#?} at {:#?}", entity, pos);
    }

    // Unselect previously selected nodes that are no longer selected
    selected_nodes_query.for_each(|entity| {
        if selected_entity.map_or(true, |(selected, _)| selected != entity) {
            commands.entity(entity).remove::<Selected>();
            #[cfg(feature = "trace")]
            info!("Unselected Entity: {:#?}", entity);
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
) -> Option<(Entity, Vec2)> {
    // First, attempt to find a non-Frame, non-Locked, non-Selected Node
    raycast_entities
        .iter()
        .rev()
        .find_map(|&(entity, pos)| {
            if node_query.contains(entity) {
                Some((entity, pos))
            } else {
                None
            }
        })
        // If no such Node is found, try to find a Frame that is not a Root,
        // not Selected and not Locked
        .or_else(|| {
            raycast_entities.iter().rev().find_map(|&(entity, pos)| {
                if frame_query.contains(entity) {
                    Some((entity, pos))
                } else {
                    None
                }
            })
        })
}

pub fn handle_cursor_moved_on_composition(
    mut event_reader: EventReader<CursorMovedOnComposition>,
    mut interactive_composition: ResMut<InteractiveCompositionRes>,
    mut selected_nodes_query: Query<(Entity, &mut RelativeTransformMixin), With<Selected>>,
) {
    for event in event_reader.read() {
        let CursorMovedOnComposition { position } = *event;
        match interactive_composition.interaction_mode {
            InteractionMode::Translating {
                ref mut current, ..
            } => {
                let offset = position - *current;

                selected_nodes_query.for_each_mut(|(_, mut relative_transform_mixin)| {
                    let translation = Mat3::from_translation(offset);
                    relative_transform_mixin.0 = relative_transform_mixin.0 * translation;
                });

                *current = position;
            }
            _ => {}
        }
    }
}

pub fn handle_cursor_down_on_composition(
    mut event_reader: EventReader<CursorDownOnComposition>,
    mut interactive_composition: ResMut<InteractiveCompositionRes>,
) {
    for event in event_reader.read() {
        #[cfg(feature = "trace")]
        info!("handle_cursor_down_on_composition: {:#?}", event.position);

        interactive_composition.interaction_mode = InteractionMode::Pressing {
            origin: event.position,
        };
    }
}

pub fn handle_cursor_up_on_composition(
    event_reader: EventReader<CursorUpOnComposition>,
    mut interactive_composition: ResMut<InteractiveCompositionRes>,
) {
    if event_reader.len() > 0 {
        #[cfg(feature = "trace")]
        info!("handle_cursor_up_on_composition");

        interactive_composition.interaction_mode = InteractionMode::None;
    }
}

pub fn handle_cursor_entered_composition(event_reader: EventReader<CursorEnteredComposition>) {
    if event_reader.len() > 0 {
        #[cfg(feature = "trace")]
        info!("handle_cursor_entered_composition");
    }
}

pub fn handle_cursor_exited_composition(
    event_reader: EventReader<CursorExitedComposition>,
    mut interactive_composition: ResMut<InteractiveCompositionRes>,
) {
    if event_reader.len() > 0 {
        #[cfg(feature = "trace")]
        info!("handle_cursor_exited_composition");

        interactive_composition.interaction_mode = InteractionMode::None;
    }
}
