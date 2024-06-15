use crate::{
    components::Selected, events::CursorMovedOnArbInputEvent,
    resources::arb_interaction::HandleSide, utils::transform_point_to_viewport,
};
use bevy_ecs::{query::With, system::Query};
use bevy_transform::components::{GlobalTransform, Transform};
use dyn_arb_bundles::{components::mixins::SizeMixin, utils::transform_to_z_rotation_rad};
use dyn_arb_core::resources::canvas::ArtboardRes;
use dyn_utils::math::matrix::rotate_around_point;
use glam::{Vec2, Vec3};

pub fn handle_rotating(
    arb_res: &ArtboardRes,
    selected_nodes_query: &mut Query<
        (&mut Transform, &GlobalTransform, &SizeMixin),
        With<Selected>,
    >,
    event: &CursorMovedOnArbInputEvent,
    corner: u8,
    initial_rotation_rad: f32,
    rotation_deg: &mut f32,
) {
    let CursorMovedOnArbInputEvent {
        position: cursor_position,
        ..
    } = event;
    let cursor_position = transform_point_to_viewport(arb_res, cursor_position, true);

    for (mut transform, global_transform, SizeMixin(size)) in selected_nodes_query.iter_mut() {
        let global_transform = global_transform.compute_transform();
        let (width, height) = size.to_tuple();
        let pivot_point = Vec3::new(width / 2.0, height / 2.0, 0.0);
        let global_pivot_point = global_transform.transform_point(pivot_point);

        // Determine rotation offset based on corner
        let rotation_offset_corner_rad: f32 = match corner {
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

        // Determine rotation offset based on global rotation (necessary for nested node)
        let rotation_offset_nested_rad = transform_to_z_rotation_rad(&global_transform)
            - transform_to_z_rotation_rad(&transform);

        let rotation_offset_rad = rotation_offset_corner_rad + rotation_offset_nested_rad;

        // Calculate rotation based on the corner
        let rotation_angle_rad =
            calculate_rotation_rad(initial_rotation_rad, &cursor_position, &global_pivot_point);
        let final_rotation_angle_rad =
            (-rotation_angle_rad + rotation_offset_rad - initial_rotation_rad) * -1.0;

        // Apply rotation to transform
        let reset_rotation_transform_mat4 = rotate_around_point(
            transform.compute_matrix(),
            -transform_to_z_rotation_rad(&transform),
            pivot_point,
        );
        let rotation_transform_mat4 = rotate_around_point(
            reset_rotation_transform_mat4,
            final_rotation_angle_rad,
            pivot_point,
        );
        *transform = Transform::from_matrix(rotation_transform_mat4);

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
    return current_angle_rad - initial_angle_rad;
}
