use crate::events::CursorEnteredCompInputEvent;
use bevy_ecs::event::EventReader;

pub fn cursor_entered_comp_input_system(
    mut event_reader: EventReader<CursorEnteredCompInputEvent>,
) {
    for _ in event_reader.read() {
        // Do nothing as of right now
    }
}
