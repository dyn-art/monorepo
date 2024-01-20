use bevy_ecs::{event::EventReader, system::ResMut};

use crate::core::modules::interactive_composition::{
    events::{CursorDownOnComposition, MouseButton},
    resources::{InteractionMode, InteractiveCompositionRes},
};

pub fn handle_cursor_down_on_composition(
    mut event_reader: EventReader<CursorDownOnComposition>,
    mut interactive_composition: ResMut<InteractiveCompositionRes>,
) {
    for event in event_reader.read() {
        #[cfg(feature = "tracing")]
        log::info!("handle_cursor_down_on_composition: {:#?}", event);

        if event.button == MouseButton::Left {
            interactive_composition.interaction_mode = InteractionMode::Pressing {
                origin: event.position,
            };
        }
    }
}
