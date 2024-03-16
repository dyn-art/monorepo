use bevy_ecs::{query::With, system::Query};
use glam::{Mat3, Vec2};

use crate::modules::{
    composition::resources::composition::CompositionRes,
    interactive_composition::{
        events::CursorMovedOnComposition, utils::transform_point_to_view_box,
    },
    node::components::{mixins::RelativeTransformMixin, states::Selected},
};

pub fn handle_translating(
    composition: &CompositionRes,
    selected_nodes_query: &mut Query<&mut RelativeTransformMixin, With<Selected>>,
    event: &CursorMovedOnComposition,
    current: &mut Vec2,
) {
    let CursorMovedOnComposition {
        position: cursor_position,
        ..
    } = event;

    let offset = transform_point_to_view_box(composition, &(*cursor_position - *current), false);

    selected_nodes_query.for_each_mut(|mut relative_transform_mixin| {
        let translation = Mat3::from_translation(offset);
        relative_transform_mixin.0 = translation * relative_transform_mixin.0;
    });

    *current = *cursor_position;
}
