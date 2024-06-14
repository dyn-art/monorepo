use crate::events::CursorEnteredArbInputEvent;
use bevy_ecs::event::EventReader;

pub fn cursor_entered_arb_input_system(
    mut event_reader: EventReader<CursorEnteredArbInputEvent>,
) {
    if event_reader.read().len() > 0 {
        log::info!("[cursor_entered_arb_input_system]");
    }
}
