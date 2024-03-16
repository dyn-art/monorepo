use crate::{
    components::Selected, events::CursorMovedOnCompInputEvent, utils::transform_point_to_viewport,
};
use bevy_ecs::{query::With, system::Query};
use bevy_transform::components::Transform;
use dyn_comp_core::resources::composition::CompositionRes;
use glam::Vec2;

pub fn handle_translating(
    comp_res: &CompositionRes,
    selected_nodes_query: &mut Query<&mut Transform, With<Selected>>,
    event: &CursorMovedOnCompInputEvent,
    current: &mut Vec2,
) {
    let CursorMovedOnCompInputEvent {
        position: cursor_position,
        ..
    } = event;
    let offset = transform_point_to_viewport(comp_res, &(*cursor_position - *current), false);

    for mut transform in selected_nodes_query.iter_mut() {
        transform.translation.x += offset.x;
        transform.translation.y += offset.y;
    }

    *current = *cursor_position;
}
