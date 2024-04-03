use crate::{
    components::Selected, events::CursorMovedOnCompInputEvent, utils::transform_point_to_viewport,
};
use bevy_ecs::{query::With, system::Query};
use bevy_hierarchy::Parent;
use bevy_transform::components::{GlobalTransform, Transform};
use dyn_comp_bundles::utils::{get_parent_global_transfrom, global_to_local_vector3};
use dyn_comp_core::resources::composition::CompositionRes;
use glam::Vec2;

pub fn handle_translating(
    comp_res: &CompositionRes,
    selected_nodes_query: &mut Query<(&mut Transform, Option<&Parent>), With<Selected>>,
    global_transform_query: &Query<&GlobalTransform>,
    event: &CursorMovedOnCompInputEvent,
    current: &mut Vec2,
) {
    let CursorMovedOnCompInputEvent {
        position: cursor_position,
        ..
    } = event;
    let offset = transform_point_to_viewport(comp_res, &(*cursor_position - *current), false);

    for (mut transform, maybe_parent) in selected_nodes_query.iter_mut() {
        let local_offset = global_to_local_vector3(
            offset.extend(0.0),
            get_parent_global_transfrom(maybe_parent, global_transform_query),
        )
        .truncate();

        transform.translation.x += local_offset.x;
        transform.translation.y += local_offset.y;
    }

    *current = *cursor_position;
}
