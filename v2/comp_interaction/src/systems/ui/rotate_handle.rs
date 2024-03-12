use bevy_ecs::{event::EventReader, system::ResMut};

use crate::{
    events::CursorDownOnRotateHandleInputEvent,
    resources::comp_interaction::{CompInteractionRes, InteractionMode},
};

pub fn handle_cursor_down_on_rotate_handle_event(
    mut event_reader: EventReader<CursorDownOnRotateHandleInputEvent>,
    mut comp_interaction_res: ResMut<CompInteractionRes>,
) {
    for event in event_reader.read() {
        #[cfg(feature = "tracing")]
        log::info!("[handle_cursor_down_on_rotate_handle_event] {:?}", event);

        comp_interaction_res.interaction_mode = InteractionMode::Rotating {
            corner: event.corner,
            initial_rotation_in_radians: event.initial_rotation_in_radians,
            rotation_in_degrees: -event.initial_rotation_in_radians.to_degrees(),
        };
    }
}
