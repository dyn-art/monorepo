use crate::{
    components::Selected, events::CursorMovedOnCompInputEvent,
    resources::comp_interaction::HandleSide, utils::transform_point_to_viewport,
};
use bevy_ecs::{query::With, system::Query};
use bevy_transform::components::{GlobalTransform, Transform};
use dyn_comp_common::mixins::SizeMixin;
use dyn_comp_core::resources::composition::CompositionRes;
use glam::{EulerRot, Mat4, Quat, Vec2, Vec3};

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

    for (mut transform, global_transform, SizeMixin(size)) in selected_nodes_query.iter_mut() {
        let computed_global_transform = global_transform.compute_transform();
        let pivot_point = Vec3::new(size.width() / 2.0, size.height() / 2.0, 0.0);
        let global_pivot_point = computed_global_transform.transform_point(pivot_point);

        // Determine rotation offset based on corner
        let rotation_offset_rad: f32 = match corner {
            _ if corner == (HandleSide::Top as u8 | HandleSide::Left as u8) => {
                f32::atan2(-size.height(), -size.width())
            }
            _ if corner == (HandleSide::Top as u8 | HandleSide::Right as u8) => {
                f32::atan2(-size.height(), size.width())
            }
            _ if corner == (HandleSide::Bottom as u8 | HandleSide::Right as u8) => {
                f32::atan2(size.height(), size.width())
            }
            _ if corner == (HandleSide::Bottom as u8 | HandleSide::Left as u8) => {
                f32::atan2(size.height(), -size.width())
            }
            _ => 0.0,
        };

        // Calculate rotation based on the corner
        let rotation_angle_rad =
            calculate_rotation_rad(initial_rotation_rad, &cursor_position, &global_pivot_point);
        let final_rotation_angle_rad =
            -rotation_angle_rad + rotation_offset_rad - initial_rotation_rad;

        // Apply rotation
        let reset_rotation_transform_mat4 = rotate_around_point(
            transform.compute_matrix(),
            -transform.rotation.to_euler(EulerRot::XYZ).2,
            pivot_point,
        );
        let rotation_transform_mat4 = rotate_around_point(
            reset_rotation_transform_mat4,
            -final_rotation_angle_rad,
            pivot_point,
        );
        *transform = Transform::from_matrix(rotation_transform_mat4);

        *rotation_deg = -final_rotation_angle_rad.to_degrees();
    }
}

// https://math.stackexchange.com/questions/2093314/rotation-matrix-of-rotation-around-a-point-other-than-the-origin
pub fn rotate_around_point(transform: Mat4, angle_rad: f32, pivot_point: Vec3) -> Mat4 {
    let translate_to_pivot = Mat4::from_translation(pivot_point);
    let translate_to_origin = Mat4::from_translation(-pivot_point);
    let rotation = Mat4::from_quat(Quat::from_rotation_z(angle_rad));
    return transform * (translate_to_pivot * rotation * translate_to_origin);
}

fn calculate_rotation_rad(initial_angle_rad: f32, cursor_point: &Vec2, pivot_point: &Vec3) -> f32 {
    // Calculate the angle from the pivot point to the current cursor position
    let current_angle_rad = f32::atan2(
        cursor_point.y - pivot_point.y,
        cursor_point.x - pivot_point.x,
    );

    // Calculate the raw angle difference
    return current_angle_rad - initial_angle_rad;
}
