use crate::{
    components::Selected,
    events::CursorMovedOnCompInputEvent,
    resources::comp_interaction::{HandleSide, XYWH},
    utils::{rotate_point, transform_point_to_viewport},
};
use bevy_ecs::{query::With, system::Query};
use bevy_transform::components::Transform;
use dyn_comp_common::{common::Size, mixins::SizeMixin};
use dyn_comp_core::resources::composition::CompositionRes;
use glam::Vec2;

pub fn handle_resizing(
    comp_res: &CompositionRes,
    selected_nodes_query: &mut Query<(&mut Transform, &mut SizeMixin), With<Selected>>,
    event: &CursorMovedOnCompInputEvent,
    corner: u8,
    initial_bounds: &mut XYWH,
) {
    let CursorMovedOnCompInputEvent {
        position: cursor_position,
        ..
    } = event;
    let cursor_position = transform_point_to_viewport(comp_res, cursor_position, true);

    for (mut transform, mut size_mixin) in selected_nodes_query.iter_mut() {
        let SizeMixin(Size(size)) = size_mixin.as_mut();
        let new_bounds = resize_bounds(
            &initial_bounds,
            corner,
            &cursor_position,
            transform.rotation.z,
        );

        transform.translation.x = new_bounds.position.x;
        transform.translation.y = new_bounds.position.y;
        *size = Vec2::new(new_bounds.width, new_bounds.height);
    }
}

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
