use crate::{
    events::CursorExitedCompInputEvent,
    resources::comp_interaction::{CompInteractionRes, InteractionMode},
};
use bevy_ecs::{event::EventReader, system::ResMut};

pub fn handle_cursor_exited_comp_event(
    mut event_reader: EventReader<CursorExitedCompInputEvent>,
    mut comp_interaction_res: ResMut<CompInteractionRes>,
) {
    for _ in event_reader.read() {
        comp_interaction_res.interaction_mode = InteractionMode::None;
    }
}
