use bevy_ecs::{
    entity::Entity,
    event::EventReader,
    query::With,
    system::{Query, ResMut},
};
use glam::{Mat3, Vec2, Vec3};
use log::info;

use crate::core::modules::{
    interactive_composition::{
        events::CursorMovedOnComposition,
        helper::{extract_transform_data, rotate_point, set_rotation},
        resources::{HandleSide, InteractionMode, InteractiveCompositionRes, XYWH},
    },
    node::components::{
        mixins::{AbsoluteTransformMixin, DimensionMixin, RelativeTransformMixin},
        states::Selected,
    },
};

pub fn handle_cursor_moved_on_composition(
    mut event_reader: EventReader<CursorMovedOnComposition>,
    mut interactive_composition: ResMut<InteractiveCompositionRes>,
    mut selected_nodes_query: Query<
        (
            Entity,
            &mut RelativeTransformMixin,
            &AbsoluteTransformMixin,
            &mut DimensionMixin,
        ),
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

                selected_nodes_query.for_each_mut(|(_, mut relative_transform_mixin, ..)| {
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
                    |(_, mut relative_transform_mixin, _, mut dimension_mixin)| {
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
                    |(
                        _,
                        mut relative_transform_mixin,
                        absolute_transform_mixin,
                        dimension_mixin,
                    )| {
                        // Calculate absolute (relative to composition) pivot point
                        let relative_pivot_point = Vec2::new(
                            dimension_mixin.width as f32 / 2.0,
                            dimension_mixin.height as f32 / 2.0,
                        );
                        let transformed_point = absolute_transform_mixin.0
                            * Vec3::new(relative_pivot_point.x, relative_pivot_point.y, 1.0);
                        let absolute_pivot_point =
                            Vec2::new(transformed_point.x, transformed_point.y);

                        // Determine rotation offset based on corner
                        let rotation_offset_in_radians: f32 = match corner {
                            _ if *corner == (HandleSide::Top as u8 | HandleSide::Left as u8) => {
                                (-135.0 as f32).to_radians()
                            }
                            _ if *corner == (HandleSide::Top as u8 | HandleSide::Right as u8) => {
                                (-45.0 as f32).to_radians()
                            }
                            _ if *corner
                                == (HandleSide::Bottom as u8 | HandleSide::Right as u8) =>
                            {
                                (45.0 as f32).to_radians()
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
                            relative_transform_mixin.0,
                            final_rotation_angle,
                            relative_pivot_point,
                        );
                        *rotation_in_degrees = final_rotation_angle.to_degrees();
                    },
                );
            }
            _ => {}
        }
    }
}

fn calculate_rotation(initial_angle_in_radians: f32, cursor_point: Vec2, pivot_point: Vec2) -> f32 {
    // Calculate the angle from the pivot point to the current cursor position
    let current_angle = (cursor_point.y - pivot_point.y).atan2(cursor_point.x - pivot_point.x);

    // Calculate the raw angle difference
    let angle_diff = current_angle - initial_angle_in_radians;

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
