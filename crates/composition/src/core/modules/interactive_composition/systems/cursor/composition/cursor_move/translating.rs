use bevy_ecs::{
    query::With,
    system::{Query, Res},
};
use glam::{Mat3, Vec2};

use crate::core::modules::{
    composition::resources::composition::CompositionRes,
    interactive_composition::events::CursorMovedOnComposition,
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

    let offset = *cursor_position - *current;

    // Calculate scale factors based on the view_box
    let scale_x = composition.view_box.width / composition.width;
    let scale_y = composition.view_box.height / composition.height;

    // Scale the offset
    let scaled_offset = Vec2::new(offset.x * scale_x, offset.y * scale_y);

    selected_nodes_query.for_each_mut(|mut relative_transform_mixin| {
        let translation = Mat3::from_translation(scaled_offset);
        relative_transform_mixin.0 = translation * relative_transform_mixin.0;
    });

    *current = cursor_position.clone();
}
