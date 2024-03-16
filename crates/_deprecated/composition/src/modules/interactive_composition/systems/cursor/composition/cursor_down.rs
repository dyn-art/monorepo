use bevy_ecs::{event::EventReader, system::ResMut};

use crate::modules::interactive_composition::{
    events::CursorDownOnComposition,
    resources::{InteractionMode, InteractiveCompositionRes, MouseButton},
};

pub fn handle_cursor_down_on_composition(
    mut event_reader: EventReader<CursorDownOnComposition>,
    mut interactive_composition: ResMut<InteractiveCompositionRes>,
) {
    for event in event_reader.read() {
        #[cfg(feature = "tracing")]
        log::info!("handle_cursor_down_on_composition: {:#?}", event);

        if event.button == MouseButton::Middle {
            interactive_composition.interaction_mode = InteractionMode::Dragging {
                current: event.position,
            };
        } else {
            interactive_composition.interaction_mode = InteractionMode::Pressing {
                origin: event.position,
                button: event.button,
            };
        }
    }
}
