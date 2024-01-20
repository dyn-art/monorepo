use bevy_ecs::{event::EventReader, system::ResMut};

use crate::core::modules::interactive_composition::{
    events::CursorDownOnRotateHandle,
    resources::{InteractionMode, InteractiveCompositionRes},
};

pub fn handle_cursor_down_on_rotate_handle(
    mut event_reader: EventReader<CursorDownOnRotateHandle>,
    mut interactive_composition: ResMut<InteractiveCompositionRes>,
) {
    for event in event_reader.read() {
        #[cfg(feature = "tracing")]
        log::info!("handle_cursor_down_on_rotate_handle: {:#?}", event);

        interactive_composition.interaction_mode = InteractionMode::Rotating {
            corner: event.corner,
            initial_rotation_in_radians: event.initial_rotation_in_radians,
            rotation_in_degrees: -event.initial_rotation_in_radians.to_degrees(),
        };
    }
}
