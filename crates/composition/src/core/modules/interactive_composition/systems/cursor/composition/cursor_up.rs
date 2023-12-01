use bevy_ecs::{event::EventReader, system::ResMut};
use log::info;

use crate::core::modules::interactive_composition::{
    events::CursorUpOnComposition,
    resources::{InteractionMode, InteractiveCompositionRes},
};

pub fn handle_cursor_up_on_composition(
    mut event_reader: EventReader<CursorUpOnComposition>,
    mut interactive_composition: ResMut<InteractiveCompositionRes>,
) {
    for event in event_reader.read() {
        #[cfg(feature = "trace")]
        info!("handle_cursor_up_on_composition: {:#?}", event);

        interactive_composition.interaction_mode = InteractionMode::None;
    }
}
