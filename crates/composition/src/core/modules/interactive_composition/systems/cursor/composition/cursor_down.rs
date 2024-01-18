use bevy_ecs::{event::EventReader, system::ResMut};
use log::info;

use crate::core::modules::interactive_composition::{
    events::CursorDownOnComposition,
    resources::{InteractionMode, InteractiveCompositionRes},
};

pub fn handle_cursor_down_on_composition(
    mut event_reader: EventReader<CursorDownOnComposition>,
    mut interactive_composition: ResMut<InteractiveCompositionRes>,
) {
    for event in event_reader.read() {
        #[cfg(feature = "tracing")]
        info!("handle_cursor_down_on_composition: {:#?}", event);

        interactive_composition.interaction_mode = InteractionMode::Pressing {
            origin: event.position,
        };
    }
}
