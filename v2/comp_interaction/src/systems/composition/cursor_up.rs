use crate::{
    events::CursorUpOnCompInputEvent,
    resources::comp_interaction::{CompInteractionRes, InteractionMode},
};
use bevy_ecs::{event::EventReader, system::ResMut};

pub fn handle_cursor_up_on_comp_event(
    mut event_reader: EventReader<CursorUpOnCompInputEvent>,
    mut comp_interaction_res: ResMut<CompInteractionRes>,
) {
    for event in event_reader.read() {
        #[cfg(feature = "tracing")]
        log::info!("[handle_cursor_up_on_comp_event] {:?}", event);

        comp_interaction_res.interaction_mode = InteractionMode::None;
    }
}
