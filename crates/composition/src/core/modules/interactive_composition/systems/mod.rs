use bevy_ecs::{
    entity::Entity,
    event::EventReader,
    query::{With, Without},
    system::{Commands, Query, ResMut},
};
use glam::{Mat3, Vec2, Vec3};
use log::info;

use crate::core::modules::{
    interactive_composition::{
        helper::{apply_rotation, set_rotation},
        resources::InteractionMode,
    },
    node::components::{
        mixins::{DimensionMixin, RelativeTransformMixin},
        states::{Locked, Selected},
        types::{Frame, Node, Root},
    },
};

use super::{
    events::{
        CursorDownOnComposition, CursorDownOnEntity, CursorDownOnResizeHandle,
        CursorDownOnRotateHandle, CursorEnteredComposition, CursorExitedComposition,
        CursorMovedOnComposition, CursorUpOnComposition,
    },
    helper::{extract_transform_data, rotate_point},
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
        let CursorMovedOnComposition {
            position: cursor_position,
        } = *event;
        match &mut interactive_composition.interaction_mode {
            InteractionMode::Translating { current, .. } => {
                let offset = cursor_position - *current;

                selected_nodes_query.for_each_mut(|(_, mut relative_transform_mixin, _)| {
                    let translation = Mat3::from_translation(offset);
                    relative_transform_mixin.0 = translation * relative_transform_mixin.0;
                });

                *current = cursor_position;
            }
            InteractionMode::Resizing {
                corner,
                initial_bounds,
                ..
            } => {
                selected_nodes_query.for_each_mut(
                    |(_, mut relative_transform_mixin, mut dimension_mixin)| {
                        let (node_angle, _, _) =
                            extract_transform_data(&relative_transform_mixin.0);
                        let new_bounds =
                            resize_bounds(&initial_bounds, *corner, cursor_position, node_angle);

                        relative_transform_mixin.0.col_mut(2).x = new_bounds.position.x;
                        relative_transform_mixin.0.col_mut(2).y = new_bounds.position.y;
                        dimension_mixin.width = new_bounds.width;
                        dimension_mixin.height = new_bounds.height;
                    },
                );
            }
            InteractionMode::Rotating {
                corner,
                initial_rotation_in_radians: initial_rotation,
                rotation_in_degrees,
            } => {
                selected_nodes_query.for_each_mut(
                    |(_, mut relative_transform_mixin, dimension_mixin)| {
                        let relative_transform = relative_transform_mixin.0;

                        // Calculate absolute (relative to composition) pivot point
                        // TODO: ofc only if there is no nesting then I have to add an absolute_transform to each node
                        let relative_pivot_point = Vec2::new(
                            dimension_mixin.width as f32 / 2.0,
                            dimension_mixin.height as f32 / 2.0,
                        );
                        let transformed_point = relative_transform
                            * Vec3::new(relative_pivot_point.x, relative_pivot_point.y, 1.0);
                        let absolute_pivot_point =
                            Vec2::new(transformed_point.x, transformed_point.y);

                        // Determine rotation offset based on corner
                        let rotation_offset_in_radians: f32 = match corner {
                            _ if *corner == (HandleSide::Top as u8 | HandleSide::Left as u8) => {
                                (-135.0 as f32).to_radians()
                            }
                            _ if *corner == (HandleSide::Top as u8 | HandleSide::Right as u8) => {
                                (135.0 as f32).to_radians()
                            }
                            _ if *corner
                                == (HandleSide::Bottom as u8 | HandleSide::Right as u8) =>
                            {
                                (-135.0 as f32).to_radians()
                            }
                            _ if *corner == (HandleSide::Bottom as u8 | HandleSide::Left as u8) => {
                                (135.0 as f32).to_radians()
                            }
                            _ => 0.0,
                        };

                        // Calculate rotation based on the corner
                        let rotation_angle = calculate_rotation(
                            *initial_rotation,
                            cursor_position,
                            absolute_pivot_point,
                        );
                        let final_rotation_angle =
                            rotation_angle + rotation_offset_in_radians - *initial_rotation;
                        relative_transform_mixin.0 = set_rotation(
                            relative_transform,
                            final_rotation_angle,
                            relative_pivot_point,
                        );
                        *rotation_in_degrees =
                            final_rotation_angle.to_degrees() - rotation_offset_in_radians;
                    },
                );
            }
            _ => {}
        }
    }
}

fn calculate_rotation(initial_angle_in_radians: f32, cursor_point: Vec2, pivot_point: Vec2) -> f32 {
    // Calculate the angle from the center to the current cursor position
    let current_angle = (cursor_point.y - pivot_point.y).atan2(cursor_point.x - pivot_point.x);

    // Calculate the raw angle difference
    let mut angle_diff = current_angle - initial_angle_in_radians;

    // Normalize the angle difference to be within -π to π
    angle_diff = if angle_diff > std::f32::consts::PI {
        angle_diff - 2.0 * std::f32::consts::PI
    } else if angle_diff < -std::f32::consts::PI {
        angle_diff + 2.0 * std::f32::consts::PI
    } else {
        angle_diff
    };

    return -angle_diff;
}

// TODO: Refactor and solve with matrix
pub fn resize_bounds(bounds: &XYWH, corner: u8, cursor_point: Vec2, node_angle: f32) -> XYWH {
    let mut result = bounds.clone();
    let pivot = bounds.position;

    // Calculate the unrotated position of the cursor
    let unrotated_cursor_point = rotate_point(cursor_point, pivot, node_angle);

    // Adjust the bounds based on the unrotated cursor position
    if (corner & HandleSide::Left as u8) == HandleSide::Left as u8 {
        result.position.x = unrotated_cursor_point
            .x
            .min(bounds.position.x + bounds.width as f32);
        result.width =
            (bounds.position.x + bounds.width as f32 - unrotated_cursor_point.x).abs() as u32;
    }
    if (corner & HandleSide::Right as u8) == HandleSide::Right as u8 {
        result.position.x = unrotated_cursor_point.x.min(bounds.position.x);
        result.width = (unrotated_cursor_point.x - bounds.position.x).abs() as u32;
    }
    if (corner & HandleSide::Top as u8) == HandleSide::Top as u8 {
        result.position.y = unrotated_cursor_point
            .y
            .min(bounds.position.y + bounds.height as f32);
        result.height =
            (bounds.position.y + bounds.height as f32 - unrotated_cursor_point.y).abs() as u32;
    }
    if (corner & HandleSide::Bottom as u8) == HandleSide::Bottom as u8 {
        result.position.y = unrotated_cursor_point.y.min(bounds.position.y);
        result.height = (unrotated_cursor_point.y - bounds.position.y).abs() as u32;
    }

    // Rotate the bounds back to the original angle
    result.position = rotate_point(result.position, pivot, -node_angle);

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
            initial_bounds: event.initial_bounds.clone(),
            rotation_in_degrees: event.rotation_in_radians.to_degrees(),
        };
    }
}

pub fn handle_cursor_down_on_rotate_handle(
    mut event_reader: EventReader<CursorDownOnRotateHandle>,
    mut interactive_composition: ResMut<InteractiveCompositionRes>,
) {
    for event in event_reader.read() {
        #[cfg(feature = "trace")]
        info!("handle_cursor_down_on_rotate_handle: {:#?}", event);

        interactive_composition.interaction_mode = InteractionMode::Rotating {
            corner: event.corner,
            initial_rotation_in_radians: event.initial_rotation_in_radians,
            rotation_in_degrees: event.initial_rotation_in_radians.to_degrees(),
        };
    }
}
