use crate::{
    events::CursorDownOnRotateHandleInputEvent,
    input::{
        button_input::ButtonInput,
        mouse::{MouseButton, MouseButtonOnRotateHandle},
    },
    resources::comp_interaction::{CompInteractionRes, InteractionMode},
};
use bevy_ecs::{event::EventReader, system::ResMut};

pub fn cursor_down_on_rotate_handle_input_system(
    mut event_reader: EventReader<CursorDownOnRotateHandleInputEvent>,
    mut comp_interaction_res: ResMut<CompInteractionRes>,
    mut mouse_button_input_res: ResMut<ButtonInput<MouseButtonOnRotateHandle, ()>>,
) {
    for event in event_reader.read() {
        mouse_button_input_res.press(MouseButtonOnRotateHandle(MouseButton::Left), ());
        comp_interaction_res.interaction_mode = InteractionMode::Rotating {
            corner: event.corner,
            initial_rotation_rad: event.initial_rotation_rad,
            rotation_deg: -event.initial_rotation_rad.to_degrees(),
        };
    }
}
