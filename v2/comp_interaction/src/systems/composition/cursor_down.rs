use crate::{
    events::CursorDownOnCompInputEvent,
    resources::comp_interaction::{CompInteractionRes, InteractionMode, MouseButton},
};
use bevy_ecs::{event::EventReader, system::ResMut};

pub fn handle_cursor_down_on_comp_event(
    mut event_reader: EventReader<CursorDownOnCompInputEvent>,
    mut comp_interaction_res: ResMut<CompInteractionRes>,
) {
    for event in event_reader.read() {
        #[cfg(feature = "tracing")]
        log::info!("[handle_cursor_down_on_comp_event] {:?}", event);

        if event.button == MouseButton::Middle {
            comp_interaction_res.interaction_mode = InteractionMode::Dragging {
                current: event.position,
            };
        } else {
            comp_interaction_res.interaction_mode = InteractionMode::Pressing {
                origin: event.position,
                button: event.button,
            };
        }
    }
}
