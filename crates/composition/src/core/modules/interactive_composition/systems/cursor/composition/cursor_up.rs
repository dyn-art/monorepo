use bevy_ecs::{event::EventReader, system::ResMut};

use crate::core::modules::interactive_composition::{
    events::{CursorUpOnComposition, MouseButton},
    resources::{InteractionMode, InteractiveCompositionRes},
};

pub fn handle_cursor_up_on_composition(
    mut event_reader: EventReader<CursorUpOnComposition>,
    mut interactive_composition: ResMut<InteractiveCompositionRes>,
) {
    for event in event_reader.read() {
        #[cfg(feature = "tracing")]
        log::info!("handle_cursor_up_on_composition: {:#?}", event);

        if event.button == MouseButton::Left {
            interactive_composition.interaction_mode = InteractionMode::None;
        }
    }
}
