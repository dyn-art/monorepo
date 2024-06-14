use crate::{
    events::CursorDownOnRotateHandleInputEvent,
    input::mouse::{
        MouseButton, MouseButtonOnRotateHandle, MouseButtonOnRotateHandleButtonInputRes,
        MouseButtonOnRotateHandleValue,
    },
    resources::arb_interaction::{ArbInteractionRes, InteractionMode},
};
use bevy_ecs::{
    change_detection::DetectChangesMut,
    event::EventReader,
    system::{Res, ResMut},
};

pub fn cursor_down_on_rotate_handle_input_system(
    mut event_reader: EventReader<CursorDownOnRotateHandleInputEvent>,
    mut mouse_button_input_res: ResMut<MouseButtonOnRotateHandleButtonInputRes>,
) {
    mouse_button_input_res.bypass_change_detection().clear();
    for event in event_reader.read() {
        log::info!(
            "[cursor_down_on_rotate_handle_input_system] {:?}",
            event.corner,
        );
        mouse_button_input_res.press(
            MouseButtonOnRotateHandle {
                button: MouseButton::Left,
                corner: event.corner,
            },
            MouseButtonOnRotateHandleValue {
                initial_rotation_rad: event.initial_rotation_rad,
            },
        );
    }
}

pub fn cursor_down_on_rotate_handle_system(
    mut arb_interaction_res: ResMut<ArbInteractionRes>,
    mouse_button_input_res: Res<MouseButtonOnRotateHandleButtonInputRes>,
) {
    for (
        MouseButtonOnRotateHandle { corner, .. },
        MouseButtonOnRotateHandleValue {
            initial_rotation_rad,
        },
    ) in mouse_button_input_res.get_just_pressed()
    {
        arb_interaction_res.interaction_mode = InteractionMode::Rotating {
            corner: *corner,
            initial_rotation_rad: *initial_rotation_rad,
            rotation_deg: initial_rotation_rad.to_degrees(),
        };
    }
}
