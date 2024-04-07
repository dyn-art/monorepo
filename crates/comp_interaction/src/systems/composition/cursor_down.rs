use crate::{
    events::CursorDownOnCompInputEvent,
    resources::comp_interaction::{CompInteractionRes, InteractionMode, InteractionTool, XYWH},
};
use bevy_ecs::{event::EventReader, system::ResMut};
use bevy_input::mouse::MouseButton;
use dyn_utils::properties::size::Size;

pub fn handle_cursor_down_on_comp_event(
    mut event_reader: EventReader<CursorDownOnCompInputEvent>,
    mut comp_interaction_res: ResMut<CompInteractionRes>,
) {
    for event in event_reader.read() {
        match event.button {
            MouseButton::Left => match comp_interaction_res.interaction_tool {
                InteractionTool::Shape { variant } => {
                    comp_interaction_res.interaction_mode = InteractionMode::Inserting {
                        initial_bounds: XYWH {
                            position: event.position,
                            size: Size::zero(),
                        },
                        shape_variant: variant,
                        entity: None,
                    };
                    return;
                }
                _ => {}
            },
            MouseButton::Middle => {
                comp_interaction_res.interaction_mode = InteractionMode::Dragging {
                    current: event.position,
                };
                return;
            }
            _ => {}
        }

        comp_interaction_res.interaction_mode = InteractionMode::Pressing {
            origin: event.position,
            button: event.button,
        };
    }
}
