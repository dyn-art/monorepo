use bevy_ecs::{event::EventReader, system::ResMut};
use log::info;

use crate::core::modules::interactive_composition::{
    events::CursorDownOnResizeHandle,
    resources::{InteractionMode, InteractiveCompositionRes},
};

pub fn handle_cursor_down_on_resize_handle(
    mut event_reader: EventReader<CursorDownOnResizeHandle>,
    mut interactive_composition: ResMut<InteractiveCompositionRes>,
) {
    for event in event_reader.read() {
        #[cfg(feature = "tracing")]
        info!("handle_cursor_down_on_resize_handle: {:#?}", event);

        interactive_composition.interaction_mode = InteractionMode::Resizing {
            corner: event.corner,
            initial_bounds: event.initial_bounds.clone(),
            rotation_in_degrees: event.rotation_in_radians.to_degrees(),
        };
    }
}
