use crate::{
    events::CursorDownOnResizeHandleInputEvent,
    input::mouse::{
        MouseButton, MouseButtonOnResizeHandle, MouseButtonOnResizeHandleButtonInputRes,
        MouseButtonOnResizeHandleValue,
    },
    resources::arb_interaction::{ArbInteractionRes, InteractionMode},
};
use bevy_ecs::{
    change_detection::DetectChangesMut,
    event::EventReader,
    system::{Res, ResMut},
};

pub fn cursor_down_on_resize_handle_input_system(
    mut event_reader: EventReader<CursorDownOnResizeHandleInputEvent>,
    mut mouse_button_input_res: ResMut<MouseButtonOnResizeHandleButtonInputRes>,
) {
    mouse_button_input_res.bypass_change_detection().clear();
    for event in event_reader.read() {
        log::info!(
            "[cursor_down_on_resize_handle_input_system] {:?}",
            event.corner,
        );
        mouse_button_input_res.press(
            MouseButtonOnResizeHandle {
                button: MouseButton::Left,
                corner: event.corner,
            },
            MouseButtonOnResizeHandleValue {
                initial_bounds: event.initial_bounds,
                rotation_rad: event.rotation_rad,
            },
        );
    }
}

pub fn cursor_down_on_resize_handle_system(
    mut arb_interaction_res: ResMut<ArbInteractionRes>,
    mouse_button_input_res: Res<MouseButtonOnResizeHandleButtonInputRes>,
) {
    for (
        MouseButtonOnResizeHandle { corner, .. },
        MouseButtonOnResizeHandleValue {
            initial_bounds,
            rotation_rad,
        },
    ) in mouse_button_input_res.get_just_pressed()
    {
        arb_interaction_res.interaction_mode = InteractionMode::Resizing {
            corner: *corner,
            initial_bounds: *initial_bounds,
            rotation_deg: rotation_rad.to_degrees(),
        };
    }
}
