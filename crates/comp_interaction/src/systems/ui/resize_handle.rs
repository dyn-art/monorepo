use crate::{
    events::CursorDownOnResizeHandleInputEvent,
    input::{
        button_input::ButtonInput,
        mouse::{MouseButton, MouseButtonOnResizeHandle},
    },
    resources::comp_interaction::{CompInteractionRes, InteractionMode},
};
use bevy_ecs::{event::EventReader, system::ResMut};

pub fn cursor_down_on_resize_handle_input_system(
    mut event_reader: EventReader<CursorDownOnResizeHandleInputEvent>,
    mut comp_interaction_res: ResMut<CompInteractionRes>,
    mut mouse_button_input_res: ResMut<ButtonInput<MouseButtonOnResizeHandle, ()>>,
) {
    for event in event_reader.read() {
        mouse_button_input_res.press(MouseButtonOnResizeHandle(MouseButton::Left), ());
        comp_interaction_res.interaction_mode = InteractionMode::Resizing {
            corner: event.corner,
            initial_bounds: event.initial_bounds,
            rotation_deg: event.rotation_rad.to_degrees(),
        };
    }
}
