use crate::events::CursorMovedOnCompInputEvent;
use dyn_comp_common::common::{Size, Viewport};
use dyn_comp_core::resources::composition::CompositionRes;
use glam::Vec2;

pub fn handle_dragging(
    comp_res: &mut CompositionRes,
    event: &CursorMovedOnCompInputEvent,
    current: &mut Vec2,
) {
    let CursorMovedOnCompInputEvent {
        position: cursor_position,
        ..
    } = event;
    let CompositionRes {
        viewport: Viewport {
            physical_position,
            physical_size,
        },
        size: Size(size),
        ..
    } = comp_res;

    let delta = event.position - *current;
    let factor = physical_size.x / size.x;
    *physical_position -= delta * factor;

    *current = *cursor_position;
}
