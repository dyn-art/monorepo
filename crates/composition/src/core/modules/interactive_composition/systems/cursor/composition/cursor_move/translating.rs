use bevy_ecs::{
    query::With,
    system::{Query, Res},
};
use glam::{Mat3, Vec2};

use crate::core::modules::{
    composition::resources::composition::CompositionRes,
    interactive_composition::{events::CursorMovedOnComposition, utils::apply_view_box_offset},
    node::components::{mixins::RelativeTransformMixin, states::Selected},
};

pub fn handle_translating(
    composition: &Res<CompositionRes>,
    selected_nodes_query: &mut Query<&mut RelativeTransformMixin, With<Selected>>,
    event: &CursorMovedOnComposition,
    current: &mut Vec2,
) {
    let CursorMovedOnComposition {
        position: cursor_position,
    } = event;

    let offset = apply_view_box_offset(composition, &(*cursor_position - *current));

    selected_nodes_query.for_each_mut(|mut relative_transform_mixin| {
        let translation = Mat3::from_translation(offset);
        relative_transform_mixin.0 = translation * relative_transform_mixin.0;
    });

    *current = cursor_position.clone();
}
