use crate::events::KeyDownOnCompInputEvent;
use bevy_ecs::{change_detection::DetectChangesMut, event::EventReader, system::ResMut};
use bevy_input::{keyboard::KeyCode, ButtonInput};

pub fn handle_key_down_event(
    mut event_reader: EventReader<KeyDownOnCompInputEvent>,
    mut keyboard_input_res: ResMut<ButtonInput<KeyCode>>,
) {
    keyboard_input_res.bypass_change_detection().clear();
    for event in event_reader.read() {
        log::info!("[handle_key_down_event] {:?}", event.key_code);
        keyboard_input_res.press(event.key_code);
    }
}
