use crate::{
    events::CursorDownOnCnvInputEvent,
    input::mouse::{MouseButton, MouseButtonButtonInputRes, MouseButtonValue},
    resources::cnv_interaction::{CnvInteractionRes, InteractionMode, InteractionTool},
};
use bevy_ecs::{
    change_detection::DetectChangesMut,
    event::EventReader,
    system::{Res, ResMut},
};

pub fn cursor_down_on_cnv_input_system(
    mut event_reader: EventReader<CursorDownOnCnvInputEvent>,
    mut mouse_button_input_res: ResMut<MouseButtonButtonInputRes>,
) {
    mouse_button_input_res.bypass_change_detection().clear();
    for event in event_reader.read() {
        log::info!("[cursor_down_on_cnv_input_system] {:?}", event.button);
        mouse_button_input_res.press(
            event.button,
            MouseButtonValue {
                position: event.position,
            },
        );
    }
}

pub fn cursor_down_on_cnv_system(
    mut cnv_interaction_res: ResMut<CnvInteractionRes>,
    mouse_button_input_res: Res<MouseButtonButtonInputRes>,
) {
    for (mouse_button, mouse_button_value) in mouse_button_input_res.get_just_pressed() {
        match mouse_button {
            MouseButton::Left => match cnv_interaction_res.interaction_tool {
                InteractionTool::Shape { variant } => {
                    cnv_interaction_res.interaction_mode = InteractionMode::Inserting {
                        origin: mouse_button_value.position,
                        shape_variant: variant,
                        entity: None,
                    };
                    return;
                }
                _ => {}
            },
            MouseButton::Middle => {
                cnv_interaction_res.interaction_mode = InteractionMode::Dragging {
                    current: mouse_button_value.position,
                };
                return;
            }
            _ => {}
        }

        cnv_interaction_res.interaction_mode = InteractionMode::Pressing {
            origin: mouse_button_value.position,
            button: *mouse_button,
        };
    }
}
