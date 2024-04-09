use crate::{events::KeyUpOnCompInputEvent, input::keyboard::KeyCodeButtonInput};
use bevy_ecs::{event::EventReader, system::ResMut};

pub fn key_up_input_system(
    mut event_reader: EventReader<KeyUpOnCompInputEvent>,
    mut keyboard_input_res: ResMut<KeyCodeButtonInput>,
) {
    for event in event_reader.read() {
        log::info!("[key_up_input_system] {:?}", event.key_code);
        keyboard_input_res.release(event.key_code);
    }
}
