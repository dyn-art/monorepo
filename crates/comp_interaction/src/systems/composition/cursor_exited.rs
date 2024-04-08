use crate::{
    events::CursorExitedCompInputEvent,
    input::{
        button_input::ButtonInput,
        keyboard::KeyCode,
        mouse::{MouseButton, MouseButtonOnEntity, MouseButtonValue},
    },
    resources::comp_interaction::{CompInteractionRes, InteractionMode},
};
use bevy_ecs::{event::EventReader, system::ResMut};

pub fn cursor_exited_comp_input_system(
    mut event_reader: EventReader<CursorExitedCompInputEvent>,
    mut comp_interaction_res: ResMut<CompInteractionRes>,
    mut keyboard_input_res: ResMut<ButtonInput<KeyCode, ()>>,
    mut mouse_button_on_comp_input_res: ResMut<ButtonInput<MouseButton, MouseButtonValue>>,
    mut mouse_button_on_entity_input_res: ResMut<
        ButtonInput<MouseButtonOnEntity, MouseButtonValue>,
    >,
) {
    if event_reader.read().next().is_some() {
        comp_interaction_res.interaction_mode = InteractionMode::None;
        keyboard_input_res.reset_all();
        mouse_button_on_comp_input_res.release_all();
        mouse_button_on_entity_input_res.release_all();
    }
}
