use crate::events::CursorEnteredCompInputEvent;
use bevy_ecs::event::EventReader;

pub fn handle_cursor_entered_comp_event(
    mut event_reader: EventReader<CursorEnteredCompInputEvent>,
) {
    for _ in event_reader.read() {
        // do nothing as of right now
    }
}
