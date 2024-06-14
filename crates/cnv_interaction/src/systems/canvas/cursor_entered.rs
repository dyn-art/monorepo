use crate::events::CursorEnteredCnvInputEvent;
use bevy_ecs::event::EventReader;

pub fn cursor_entered_cnv_input_system(
    mut event_reader: EventReader<CursorEnteredCnvInputEvent>,
) {
    if event_reader.read().len() > 0 {
        log::info!("[cursor_entered_cnv_input_system]");
    }
}
