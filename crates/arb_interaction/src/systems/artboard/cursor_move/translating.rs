use crate::{
    components::Selected, events::CursorMovedOnArbInputEvent, utils::transform_point_to_viewport,
};
use bevy_ecs::{query::With, system::Query};
use bevy_hierarchy::Parent;
use bevy_transform::components::{GlobalTransform, Transform};
use dyn_arb_bundles::utils::{get_parent_global_transfrom, global_to_local_vector3};
use dyn_arb_core::resources::artboard::ArtboardRes;
use glam::Vec2;

pub fn handle_translating(
    arb_res: &ArtboardRes,
    selected_nodes_query: &mut Query<(&mut Transform, Option<&Parent>), With<Selected>>,
    global_transform_query: &Query<&GlobalTransform>,
    event: &CursorMovedOnArbInputEvent,
    current: &mut Vec2,
) {
    let CursorMovedOnArbInputEvent {
        position: cursor_position,
        ..
    } = event;
    let offset = transform_point_to_viewport(arb_res, &(*cursor_position - *current), false);

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
