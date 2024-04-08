use crate::{events::KeyDownOnCompInputEvent, input::keyboard::KeyCodeButtonInput};
use bevy_ecs::{change_detection::DetectChangesMut, event::EventReader, system::ResMut};

pub fn key_down_input_system(
    mut event_reader: EventReader<KeyDownOnCompInputEvent>,
    mut keyboard_input_res: ResMut<KeyCodeButtonInput>,
) {
    keyboard_input_res.bypass_change_detection().clear();
    for event in event_reader.read() {
        log::info!("[key_down_input_system] {:?}", event.key_code);
        keyboard_input_res.press(event.key_code, ());
    }
}
