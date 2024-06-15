use crate::{
    events::CursorDownOnArbInputEvent,
    input::mouse::{MouseButton, MouseButtonButtonInputRes, MouseButtonValue},
    resources::arb_interaction::{ArbInteractionRes, InteractionMode, InteractionTool},
};
use bevy_ecs::{
    change_detection::DetectChangesMut,
    event::EventReader,
    system::{Res, ResMut},
};

pub fn cursor_down_on_arb_input_system(
    mut event_reader: EventReader<CursorDownOnArbInputEvent>,
    mut mouse_button_input_res: ResMut<MouseButtonButtonInputRes>,
) {
    mouse_button_input_res.bypass_change_detection().clear();
    for event in event_reader.read() {
        log::info!("[cursor_down_on_arb_input_system] {:?}", event.button);
        mouse_button_input_res.press(
            event.button,
            MouseButtonValue {
                position: event.position,
            },
        );
    }
}

pub fn cursor_down_on_arb_system(
    mut arb_interaction_res: ResMut<ArbInteractionRes>,
    mouse_button_input_res: Res<MouseButtonButtonInputRes>,
) {
    for (mouse_button, mouse_button_value) in mouse_button_input_res.get_just_pressed() {
        match mouse_button {
            MouseButton::Left => match arb_interaction_res.interaction_tool {
                InteractionTool::Shape { variant } => {
                    arb_interaction_res.interaction_mode = InteractionMode::Inserting {
                        origin: mouse_button_value.position,
                        shape_variant: variant,
                        entity: None,
                    };
                    return;
                }
                _ => {}
            },
            MouseButton::Middle => {
                arb_interaction_res.interaction_mode = InteractionMode::Dragging {
                    current: mouse_button_value.position,
                };
                return;
            }
            _ => {}
        }

        arb_interaction_res.interaction_mode = InteractionMode::Pressing {
            origin: mouse_button_value.position,
            button: *mouse_button,
        };
    }
}
