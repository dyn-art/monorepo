use bevy_ecs::{query::With, system::Query};
use glam::Vec2;

use crate::core::modules::{
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
    let cursor_position = transform_point_to_view_box(composition, cursor_position);

    selected_nodes_query.for_each_mut(
        |(mut relative_transform_mixin, absolute_transform_mixin, dimension_mixin)| {
            let relative_pivot_point = Vec2::new(
                dimension_mixin.width as f32 / 2.0,
                dimension_mixin.height as f32 / 2.0,
            );
            let absolute_pivot_point =
                apply_transform_to_point(absolute_transform_mixin.0, relative_pivot_point);

            // Determine rotation offset based on corner
            let rotation_offset_in_radians: f32 = match corner {
                _ if corner == (HandleSide::Top as u8 | HandleSide::Left as u8) => {
                    // Top-Left corner
                    // let absolute_corner = apply_transform_to_point(
                    //     absolute_transform_mixin.0,
                    //     Vec2::new(0.0, 0.0),
                    // );

                    (-135.0 as f32).to_radians()
                    // + calculate_rotation_correction(
                    //     dimension_mixin.width as f32,
                    //     dimension_mixin.height as f32,
                    //     absolute_pivot_point,
                    //     absolute_corner,
                    // )
                }
                _ if corner == (HandleSide::Top as u8 | HandleSide::Right as u8) => {
                    // Top-Right corner
                    // let absolute_corner = apply_transform_to_point(
                    //     absolute_transform_mixin.0,
                    //     Vec2::new(dimension_mixin.width as f32, 0.0),
                    // );

                    (-45.0 as f32).to_radians()
                }
                _ if corner == (HandleSide::Bottom as u8 | HandleSide::Right as u8) => {
                    // Bottom-Right corner
                    // let absolute_corner = apply_transform_to_point(
                    //     absolute_transform_mixin.0,
                    //     Vec2::new(
                    //         dimension_mixin.width as f32,
                    //         dimension_mixin.height as f32,
                    //     ),
                    // );

                    (45.0 as f32).to_radians()
                }
                _ if corner == (HandleSide::Bottom as u8 | HandleSide::Left as u8) => {
                    // Bottom-Left corner
                    // let absolute_corner = apply_transform_to_point(
                    //     absolute_transform_mixin.0,
                    //     Vec2::new(0.0, dimension_mixin.height as f32),
                    // );

                    (135.0 as f32).to_radians()
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

// TODO Calculate offset for proper rotation if the node is not perfect square
// fn calculate_rotation_correction(width: f32, height: f32, center: Vec2, p2: Vec2) -> f32 {
//     let radius = width.max(height) / 2.0;
//     let angle_for_square = std::f32::consts::PI / 4.0; // 45 degrees

//     // Calculate P1's coordinates
//     let p1 = Vec2 {
//         x: center.x - radius * angle_for_square.cos(),
//         y: center.y - radius * angle_for_square.sin(),
//     };

//     // Calculate vectors MP1 and MP2
//     let mp1 = Vec2 {
//         x: p1.x - center.x,
//         y: p1.y - center.y,
//     };
//     let mp2 = Vec2 {
//         x: p2.x - center.x,
//         y: p2.y - center.y,
//     };

//     // Calculate the angle between MP1 and MP2
//     let dot_product = mp1.x * mp2.x + mp1.y * mp2.y;
//     let magnitudes =
//         (mp1.x.powi(2) + mp1.y.powi(2)).sqrt() * (mp2.x.powi(2) + mp2.y.powi(2)).sqrt();

//     let angle = (dot_product / magnitudes).acos();

//     info!(
//         "calculate_rotation_correction: \n width: {} \n height: {} \n center: {:?} \n p2: {:?} \n p1: {:?} \n angle: {}",
//         width, height, center, p2, p1, angle
//     );

//     return angle;
// }

fn calculate_rotation(
    initial_angle_in_radians: f32,
    cursor_point: &Vec2,
    pivot_point: &Vec2,
) -> f32 {
    // Calculate the angle from the pivot point to the current cursor position
    let current_angle = (cursor_point.y - pivot_point.y).atan2(cursor_point.x - pivot_point.x);

    // Calculate the raw angle difference
    let angle_diff = current_angle - initial_angle_in_radians;

    return -angle_diff;
}
