use crate::events::CursorEnteredCompInputEvent;
use bevy_ecs::event::EventReader;

pub fn handle_cursor_entered_comp_event(
    mut event_reader: EventReader<CursorEnteredCompInputEvent>,
) {
    for event in event_reader.read() {
        #[cfg(feature = "tracing")]
        log::info!("[handle_cursor_entered_comp_event] {:?}", event);
    }
}
