use crate::{
    events::CursorUpOnEntityInputEvent,
    input::{
        button_input::ButtonInput,
        mouse::{MouseButtonOnEntity, MouseButtonValue},
    },
};
use bevy_ecs::{event::EventReader, system::ResMut};

pub fn cursor_up_on_entity_input_system(
    mut event_reader: EventReader<CursorUpOnEntityInputEvent>,
    mut mouse_button_input_res: ResMut<ButtonInput<MouseButtonOnEntity, MouseButtonValue>>,
) {
    for event in event_reader.read() {
        log::info!(
            "[cursor_up_on_entity_input_system] {:?} on {:?}",
            event.button,
            event.entity
        );
        mouse_button_input_res.release(MouseButtonOnEntity {
            entity: event.entity,
            button: event.button,
        });
    }
}
