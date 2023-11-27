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
        mixins::{DimensionMixin, RelativeTransformMixin},
        states::{Locked, Selected},
        types::{Frame, Node, Root},
    },
};

use super::{
    events::{
        CursorDownOnComposition, CursorDownOnEntity, CursorDownOnResizeHandle,
        CursorEnteredComposition, CursorExitedComposition, CursorMovedOnComposition,
        CursorUpOnComposition,
    },
    resources::{HandleSide, InteractiveCompositionRes, XYWH},
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
        .map(|event| (event.entity, event.position))
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

            #[cfg(feature = "trace")]
            info!("Selected Entity {:#?} at {:#?}", entity, pos);
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

pub fn handle_cursor_moved_on_composition(
    mut event_reader: EventReader<CursorMovedOnComposition>,
    mut interactive_composition: ResMut<InteractiveCompositionRes>,
    mut selected_nodes_query: Query<
        (Entity, &mut RelativeTransformMixin, &mut DimensionMixin),
        With<Selected>,
    >,
) {
    for event in event_reader.read() {
        let CursorMovedOnComposition { position } = *event;
        match &mut interactive_composition.interaction_mode {
            InteractionMode::Translating { current, .. } => {
                let offset = position - *current;

                selected_nodes_query.for_each_mut(|(_, mut relative_transform_mixin, _)| {
                    let translation = Mat3::from_translation(offset);
                    relative_transform_mixin.0 = translation * relative_transform_mixin.0;
                });

                *current = position;
            }
            InteractionMode::Resizing {
                corner,
                inital_bounds,
                ..
            } => {
                let new_bounds = resize_bounds(&inital_bounds, *corner, position);

                selected_nodes_query.for_each_mut(
                    |(_, mut relative_transform_mixin, mut dimension_mixin)| {
                        relative_transform_mixin.0.col_mut(2).x = new_bounds.position.x;
                        relative_transform_mixin.0.col_mut(2).y = new_bounds.position.y;
                        dimension_mixin.width = new_bounds.width;
                        dimension_mixin.height = new_bounds.height
                    },
                );
            }
            _ => {}
        }
    }
}

pub fn resize_bounds(bounds: &XYWH, corner: u8, point: Vec2) -> XYWH {
    let mut result = bounds.clone();

    if (corner & HandleSide::Left as u8) == HandleSide::Left as u8 {
        result.position.x = point.x.min(bounds.position.x + bounds.width as f32);
        result.width = (bounds.position.x + bounds.width as f32 - point.x).abs() as u32;
    }

    if (corner & HandleSide::Right as u8) == HandleSide::Right as u8 {
        result.position.x = point.x.min(bounds.position.x);
        result.width = (point.x - bounds.position.x).abs() as u32;
    }

    if (corner & HandleSide::Top as u8) == HandleSide::Top as u8 {
        result.position.y = point.y.min(bounds.position.y + bounds.height as f32);
        result.height = (bounds.position.y + bounds.height as f32 - point.y).abs() as u32;
    }

    if (corner & HandleSide::Bottom as u8) == HandleSide::Bottom as u8 {
        result.position.y = point.y.min(bounds.position.y);
        result.height = (point.y - bounds.position.y).abs() as u32;
    }

    return result;
}

pub fn handle_cursor_down_on_composition(
    mut event_reader: EventReader<CursorDownOnComposition>,
    mut interactive_composition: ResMut<InteractiveCompositionRes>,
) {
    for event in event_reader.read() {
        #[cfg(feature = "trace")]
        info!("handle_cursor_down_on_composition: {:#?}", event);

        interactive_composition.interaction_mode = InteractionMode::Pressing {
            origin: event.position,
        };
    }
}

pub fn handle_cursor_up_on_composition(
    mut event_reader: EventReader<CursorUpOnComposition>,
    mut interactive_composition: ResMut<InteractiveCompositionRes>,
) {
    for event in event_reader.read() {
        #[cfg(feature = "trace")]
        info!("handle_cursor_up_on_composition: {:#?}", event);

        interactive_composition.interaction_mode = InteractionMode::None;
    }
}

pub fn handle_cursor_entered_composition(mut event_reader: EventReader<CursorEnteredComposition>) {
    for event in event_reader.read() {
        #[cfg(feature = "trace")]
        info!("handle_cursor_entered_composition");
    }
}

pub fn handle_cursor_exited_composition(
    mut event_reader: EventReader<CursorExitedComposition>,
    mut interactive_composition: ResMut<InteractiveCompositionRes>,
) {
    for event in event_reader.read() {
        #[cfg(feature = "trace")]
        info!("handle_cursor_exited_composition");

        interactive_composition.interaction_mode = InteractionMode::None;
    }
}

pub fn handle_cursor_down_on_resize_handle(
    mut event_reader: EventReader<CursorDownOnResizeHandle>,
    mut interactive_composition: ResMut<InteractiveCompositionRes>,
) {
    for event in event_reader.read() {
        #[cfg(feature = "trace")]
        info!("handle_cursor_down_on_resize_handle: {:#?}", event);

        interactive_composition.interaction_mode = InteractionMode::Resizing {
            corner: event.corner,
            inital_bounds: event.inital_bounds.clone(),
            rotation: 0.0, // TODO: Get and update rotation of selected elements
        };
    }
}
