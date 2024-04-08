use crate::{
    events::CursorDownOnCompInputEvent,
    input::{
        button_input::ButtonInput,
        mouse::{MouseButton, MouseButtonValue},
    },
    resources::comp_interaction::{CompInteractionRes, InteractionMode, InteractionTool, XYWH},
};
use bevy_ecs::{change_detection::DetectChangesMut, event::EventReader, system::ResMut};
use dyn_utils::properties::size::Size;

pub fn cursor_down_on_comp_input_system(
    mut event_reader: EventReader<CursorDownOnCompInputEvent>,
    mut mouse_button_input_res: ResMut<ButtonInput<MouseButton, MouseButtonValue>>,
) {
    mouse_button_input_res.bypass_change_detection().clear();
    for event in event_reader.read() {
        log::info!("[cursor_down_on_comp_input_system] {:?}", event.button);
        mouse_button_input_res.press(
            event.button,
            MouseButtonValue {
                position: event.position,
            },
        );
    }
}

pub fn cursor_down_on_comp_system(
    mut comp_interaction_res: ResMut<CompInteractionRes>,
    mouse_button_input_res: ResMut<ButtonInput<MouseButton, MouseButtonValue>>,
) {
    for (mouse_button, mouse_button_value) in mouse_button_input_res.get_just_pressed() {
        match mouse_button {
            MouseButton::Left => match comp_interaction_res.interaction_tool {
                InteractionTool::Shape { variant } => {
                    comp_interaction_res.interaction_mode = InteractionMode::Inserting {
                        initial_bounds: XYWH {
                            position: mouse_button_value.position,
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
                    current: mouse_button_value.position,
                };
                return;
            }
            _ => {}
        }

        comp_interaction_res.interaction_mode = InteractionMode::Pressing {
            origin: mouse_button_value.position,
            button: *mouse_button,
        };
    }
}
