use bevy_ecs::{query::With, system::Query};
use glam::Vec2;

use crate::core::modules::{
    composition::resources::composition::CompositionRes,
    interactive_composition::{
        events::CursorMovedOnComposition,
        resources::{HandleSide, XYWH},
        utils::transform_point_to_view_box,
    },
    node::{
        components::{
            mixins::{DimensionMixin, RelativeTransformMixin},
            states::Selected,
        },
        utils::transform::{extract_transform_data, rotate_point},
    },
};

pub fn handle_resizing(
    composition: &CompositionRes,
    selected_nodes_query: &mut Query<
        (&mut RelativeTransformMixin, &mut DimensionMixin),
        With<Selected>,
    >,
    event: &CursorMovedOnComposition,
    corner: u8,
    initial_bounds: &mut XYWH,
) {
    let CursorMovedOnComposition {
        position: cursor_position,
        ..
    } = event;
    let cursor_position = transform_point_to_view_box(composition, cursor_position);

    selected_nodes_query.for_each_mut(|(mut relative_transform_mixin, mut dimension_mixin)| {
        let (node_angle, _, _) = extract_transform_data(&relative_transform_mixin.0);
        let new_bounds = resize_bounds(&initial_bounds, corner, &cursor_position, node_angle);

        relative_transform_mixin.0.col_mut(2).x = new_bounds.position.x;
        relative_transform_mixin.0.col_mut(2).y = new_bounds.position.y;
        dimension_mixin.width = new_bounds.width;
        dimension_mixin.height = new_bounds.height;
    });
}

// TODO: Refactor and solve with matrix
pub fn resize_bounds(bounds: &XYWH, corner: u8, cursor_point: &Vec2, node_angle: f32) -> XYWH {
    let mut result = bounds.clone();
    let pivot = bounds.position;

    // Calculate the unrotated position of the cursor
    let unrotated_cursor_point = rotate_point(cursor_point, &pivot, node_angle);

    // Adjust the bounds based on the unrotated cursor position
    if (corner & HandleSide::Left as u8) == HandleSide::Left as u8 {
        result.position.x = unrotated_cursor_point
            .x
            .min(bounds.position.x + bounds.width);
        result.width = (bounds.position.x + bounds.width - unrotated_cursor_point.x).abs();
    }
    if (corner & HandleSide::Right as u8) == HandleSide::Right as u8 {
        result.position.x = unrotated_cursor_point.x.min(bounds.position.x);
        result.width = (unrotated_cursor_point.x - bounds.position.x).abs();
    }
    if (corner & HandleSide::Top as u8) == HandleSide::Top as u8 {
        result.position.y = unrotated_cursor_point
            .y
            .min(bounds.position.y + bounds.height as f32);
        result.height = (bounds.position.y + bounds.height - unrotated_cursor_point.y).abs();
    }
    if (corner & HandleSide::Bottom as u8) == HandleSide::Bottom as u8 {
        result.position.y = unrotated_cursor_point.y.min(bounds.position.y);
        result.height = (unrotated_cursor_point.y - bounds.position.y).abs();
    }

    // Rotate the bounds back to the original angle
    result.position = rotate_point(&result.position, &pivot, -node_angle);

    return result;
}
