use crate::events::CursorMovedOnArbInputEvent;
use dyn_arb_bundles::properties::Viewport;
use dyn_arb_core::resources::canvas::ArtboardRes;
use glam::Vec2;

pub fn handle_dragging(
    arb_res: &mut ArtboardRes,
    event: &CursorMovedOnArbInputEvent,
    current: &mut Vec2,
) {
    let CursorMovedOnArbInputEvent {
        position: cursor_position,
        ..
    } = event;
    let ArtboardRes {
        viewport: Viewport {
            physical_position,
            physical_size,
        },
        size,
        ..
    } = arb_res;

    let delta = event.position - *current;
    let factor = physical_size.width() / size.width();
    *physical_position -= delta * factor;

    *current = *cursor_position;
}
