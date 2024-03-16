use bevy_ecs::{query::With, system::Query};
use glam::Vec2;

use crate::modules::{
    composition::resources::composition::CompositionRes,
    interactive_composition::{
        events::CursorMovedOnComposition, resources::HandleSide, utils::transform_point_to_view_box,
    },
    node::{
        components::{
            mixins::{AbsoluteTransformMixin, DimensionMixin, RelativeTransformMixin},
            states::Selected,
        },
        utils::transform::{apply_transform_to_point, set_rotation},
    },
};

pub fn handle_rotating(
    composition: &CompositionRes,
    selected_nodes_query: &mut Query<
        (
            &mut RelativeTransformMixin,
            &AbsoluteTransformMixin,
            &mut DimensionMixin,
        ),
        With<Selected>,
    >,
    event: &CursorMovedOnComposition,
    corner: u8,
    initial_rotation: f32,
    rotation_in_degrees: &mut f32,
) {
    let CursorMovedOnComposition {
        position: cursor_position,
        ..
    } = event;
    let cursor_position = transform_point_to_view_box(composition, cursor_position, true);

    selected_nodes_query.for_each_mut(
        |(mut relative_transform_mixin, absolute_transform_mixin, dimension_mixin)| {
            let width: f32 = dimension_mixin.width;
            let height: f32 = dimension_mixin.height;
            let relative_pivot_point = Vec2::new(width / 2.0, height / 2.0);
            let absolute_pivot_point =
                apply_transform_to_point(absolute_transform_mixin.0, relative_pivot_point);

            // Determine rotation offset based on corner
            let rotation_offset_in_radians: f32 = match corner {
                _ if corner == (HandleSide::Top as u8 | HandleSide::Left as u8) => {
                    f32::atan2(-height, -width)
                }
                _ if corner == (HandleSide::Top as u8 | HandleSide::Right as u8) => {
                    f32::atan2(-height, width)
                }
                _ if corner == (HandleSide::Bottom as u8 | HandleSide::Right as u8) => {
                    f32::atan2(height, width)
                }
                _ if corner == (HandleSide::Bottom as u8 | HandleSide::Left as u8) => {
                    f32::atan2(height, -width)
                }
                _ => 0.0,
            };

            // Calculate rotation based on the corner
            let rotation_angle =
                calculate_rotation(initial_rotation, &cursor_position, &absolute_pivot_point);
            let final_rotation_angle =
                rotation_angle + rotation_offset_in_radians - initial_rotation;
            relative_transform_mixin.0 = set_rotation(
                relative_transform_mixin.0,
                final_rotation_angle,
                relative_pivot_point,
            );
            *rotation_in_degrees = final_rotation_angle.to_degrees();
        },
    );
}

fn calculate_rotation(
    initial_angle_in_radians: f32,
    cursor_point: &Vec2,
    pivot_point: &Vec2,
) -> f32 {
    // Calculate the angle from the pivot point to the current cursor position
    let current_angle = f32::atan2(
        cursor_point.y - pivot_point.y,
        cursor_point.x - pivot_point.x,
    );

    // Calculate the raw angle difference
    let angle_diff = current_angle - initial_angle_in_radians;

    return -angle_diff;
}
