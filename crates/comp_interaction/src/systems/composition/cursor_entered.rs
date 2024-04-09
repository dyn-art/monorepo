use crate::events::CursorEnteredCompInputEvent;
use bevy_ecs::event::EventReader;

pub fn cursor_entered_comp_input_system(
    mut event_reader: EventReader<CursorEnteredCompInputEvent>,
) {
    if event_reader.read().len() > 0 {
        log::info!("[cursor_entered_comp_input_system]");
    }
}
