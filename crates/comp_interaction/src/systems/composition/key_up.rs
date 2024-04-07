use crate::events::KeyUpOnCompInputEvent;
use bevy_ecs::{change_detection::DetectChangesMut, event::EventReader, system::ResMut};
use bevy_input::{keyboard::KeyCode, ButtonInput};

pub fn handle_key_up_event(
    mut event_reader: EventReader<KeyUpOnCompInputEvent>,
    mut keyboard_input_res: ResMut<ButtonInput<KeyCode>>,
) {
    keyboard_input_res.bypass_change_detection().clear();
    for event in event_reader.read() {
        log::info!("[handle_key_up_event] {:?}", event.key_code);
        keyboard_input_res.release(event.key_code);
    }
}
