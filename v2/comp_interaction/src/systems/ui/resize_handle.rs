use crate::{
    events::CursorDownOnResizeHandleInputEvent,
    resources::comp_interaction::{CompInteractionRes, InteractionMode},
};
use bevy_ecs::{event::EventReader, system::ResMut};

pub fn handle_cursor_down_on_resize_handle_event(
    mut event_reader: EventReader<CursorDownOnResizeHandleInputEvent>,
    mut comp_interaction_res: ResMut<CompInteractionRes>,
) {
    for event in event_reader.read() {
        #[cfg(feature = "tracing")]
        log::info!("[handle_cursor_down_on_resize_handle_event] {:?}", event);

        comp_interaction_res.interaction_mode = InteractionMode::Resizing {
            corner: event.corner,
            initial_bounds: event.initial_bounds.clone(),
            rotation_in_degrees: event.rotation_in_radians.to_degrees(),
        };
    }
}
