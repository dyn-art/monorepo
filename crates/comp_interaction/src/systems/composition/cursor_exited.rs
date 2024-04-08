use crate::{
    events::CursorExitedCompInputEvent,
    input::{
        keyboard::KeyCodeButtonInput,
        mouse::{MouseButtonButtonInput, MouseButtonOnEntityButtonInput},
    },
    resources::comp_interaction::{CompInteractionRes, InteractionMode},
};
use bevy_ecs::{event::EventReader, system::ResMut};

pub fn cursor_exited_comp_input_system(
    mut event_reader: EventReader<CursorExitedCompInputEvent>,
    mut comp_interaction_res: ResMut<CompInteractionRes>,
    mut keyboard_input_res: ResMut<KeyCodeButtonInput>,
    mut mouse_button_on_comp_input_res: ResMut<MouseButtonButtonInput>,
    mut mouse_button_on_entity_input_res: ResMut<MouseButtonOnEntityButtonInput>,
) {
    if event_reader.read().len() > 0 {
        log::info!("[cursor_exited_comp_input_system]");
        comp_interaction_res.interaction_mode = InteractionMode::None;
        keyboard_input_res.reset_all();
        mouse_button_on_comp_input_res.release_all();
        mouse_button_on_entity_input_res.release_all();
    }
}
