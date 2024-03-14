use crate::{
    components::Selected, events::CursorMovedOnCompInputEvent,
    resources::comp_interaction::HandleSide, utils::transform_point_to_viewport,
};
use bevy_ecs::{query::With, system::Query};
use bevy_transform::components::{GlobalTransform, Transform};
use dyn_comp_common::{common::Size, mixins::SizeMixin};
use dyn_comp_core::resources::composition::CompositionRes;
use glam::{Quat, Vec2, Vec3};

pub fn handle_rotating(
    comp_res: &CompositionRes,
    selected_nodes_query: &mut Query<
        (&mut Transform, &GlobalTransform, &SizeMixin),
        With<Selected>,
    >,
    event: &CursorMovedOnCompInputEvent,
    corner: u8,
    initial_rotation_rad: f32,
    rotation_deg: &mut f32,
) {
    let CursorMovedOnCompInputEvent {
        position: cursor_position,
        ..
    } = event;
    let cursor_position = transform_point_to_viewport(comp_res, cursor_position, true);

    for (mut transform, global_transform, SizeMixin(Size(size))) in selected_nodes_query.iter_mut()
    {
        let relative_pivot_point = Vec3::new(size.x / 2.0, size.y / 2.0, 0.0);
        let absolute_pivot_point =
            global_transform.compute_transform().translation * relative_pivot_point;

        // Determine rotation offset based on corner
        let rotation_offset_rad: f32 = match corner {
            _ if corner == (HandleSide::Top as u8 | HandleSide::Left as u8) => {
                f32::atan2(-size.y, size.x)
            }
            _ if corner == (HandleSide::Top as u8 | HandleSide::Right as u8) => {
                f32::atan2(size.y, size.x)
            }
            _ if corner == (HandleSide::Bottom as u8 | HandleSide::Right as u8) => {
                f32::atan2(size.y, size.x)
            }
            _ if corner == (HandleSide::Bottom as u8 | HandleSide::Left as u8) => {
                f32::atan2(size.y, size.x)
            }
            _ => 0.0,
        };

        // Calculate rotation based on the corner
        let rotation_angle_rad = calculate_rotation_rad(
            initial_rotation_rad,
            &cursor_position,
            &absolute_pivot_point,
        );
        let final_rotation_angle_rad =
            rotation_angle_rad + rotation_offset_rad - initial_rotation_rad;
        transform.rotate_around(
            relative_pivot_point,
            Quat::from_rotation_z(final_rotation_angle_rad),
        );
        *rotation_deg = final_rotation_angle_rad.to_degrees();
    }
}

fn calculate_rotation_rad(initial_angle_rad: f32, cursor_point: &Vec2, pivot_point: &Vec3) -> f32 {
    // Calculate the angle from the pivot point to the current cursor position
    let current_angle_rad = f32::atan2(
        cursor_point.y - pivot_point.y,
        cursor_point.x - pivot_point.x,
    );

    // Calculate the raw angle difference
    let angle_diff = current_angle_rad - initial_angle_rad;

    return -angle_diff;
}
