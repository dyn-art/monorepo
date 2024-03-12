use crate::events::CursorMovedOnCompInputEvent;
use dyn_comp_common::common::Viewport;
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
        ..
    } = comp_res;

    let delta = event.position - *current;
    let factor = physical_size.x / physical_size.y;
    *physical_position -= delta * factor;

    *current = *cursor_position;
}
