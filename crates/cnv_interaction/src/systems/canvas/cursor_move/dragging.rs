use crate::events::CursorMovedOnCnvInputEvent;
use dyn_cnv_bundles::properties::Viewport;
use dyn_cnv_core::resources::canvas::CanvasRes;
use glam::Vec2;

pub fn handle_dragging(
    cnv_res: &mut CanvasRes,
    event: &CursorMovedOnCnvInputEvent,
    current: &mut Vec2,
) {
    let CursorMovedOnCnvInputEvent {
        position: cursor_position,
        ..
    } = event;
    let CanvasRes {
        viewport: Viewport {
            physical_position,
            physical_size,
        },
        size,
        ..
    } = cnv_res;

    let delta = event.position - *current;
    let factor = physical_size.width() / size.width();
    *physical_position -= delta * factor;

    *current = *cursor_position;
}
