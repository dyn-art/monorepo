use bevy_ecs::{event::EventReader, system::ResMut};

use crate::core::modules::interactive_composition::{
    events::CursorExitedComposition,
    resources::{InteractionMode, InteractiveCompositionRes},
};

pub fn handle_cursor_exited_composition(
    mut event_reader: EventReader<CursorExitedComposition>,
    mut interactive_composition: ResMut<InteractiveCompositionRes>,
) {
    for event in event_reader.read() {
        #[cfg(feature = "tracing")]
        log::info!("handle_cursor_exited_composition");

        interactive_composition.interaction_mode = InteractionMode::None;
    }
}
