use crate::{
    events::CursorUpOnCompInputEvent,
    resources::comp_interaction::{CompInteractionRes, InteractionMode},
};
use bevy_ecs::{event::EventReader, system::ResMut};

pub fn handle_cursor_up_on_comp_event(
    mut event_reader: EventReader<CursorUpOnCompInputEvent>,
    mut comp_interaction_res: ResMut<CompInteractionRes>,
) {
    if event_reader.read().len() > 0 {
        comp_interaction_res.interaction_mode = InteractionMode::None;
    }
}
