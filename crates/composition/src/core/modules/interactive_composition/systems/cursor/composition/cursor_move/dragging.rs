use glam::Vec2;

use crate::core::modules::{
    composition::resources::composition::CompositionRes,
    interactive_composition::events::CursorMovedOnComposition,
};

pub fn handle_dragging(
    composition: &mut CompositionRes,
    event: &CursorMovedOnComposition,
    current: &mut Vec2,
) {
    let CursorMovedOnComposition {
        position: cursor_position,
        ..
    } = event;

    let delta = event.position - *current;
    let factor = composition.view_box.width / composition.width;

    composition.view_box.min_x -= delta.x * factor;
    composition.view_box.min_y -= delta.y * factor;

    *current = *cursor_position;
}
