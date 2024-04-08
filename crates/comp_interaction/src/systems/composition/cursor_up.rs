use crate::{
    components::Selected,
    events::CursorUpOnCompInputEvent,
    input::{
        button_input::ButtonInput,
        mouse::{
            MouseButton, MouseButtonOnResizeHandle, MouseButtonOnRotateHandle, MouseButtonValue,
        },
    },
    resources::comp_interaction::{CompInteractionRes, InteractionMode, InteractionTool},
};
use bevy_ecs::{
    event::EventReader,
    system::{Commands, Res, ResMut},
};

pub fn cursor_up_on_comp_input_system(
    mut event_reader: EventReader<CursorUpOnCompInputEvent>,
    mut mouse_button_on_comp_input_res: ResMut<ButtonInput<MouseButton, MouseButtonValue>>,
    mut mouse_button_on_resize_handle_input_res: ResMut<ButtonInput<MouseButtonOnResizeHandle, ()>>,
    mut mouse_button_on_rotate_handle_input_res: ResMut<ButtonInput<MouseButtonOnRotateHandle, ()>>,
) {
    for event in event_reader.read() {
        log::info!("[cursor_up_on_comp_input_system] {:?}", event.button);
        mouse_button_on_comp_input_res.release(event.button);
        mouse_button_on_resize_handle_input_res.release(MouseButtonOnResizeHandle(event.button));
        mouse_button_on_rotate_handle_input_res.release(MouseButtonOnRotateHandle(event.button));
    }
}

pub fn cursor_up_on_comp_system(
    mut commands: Commands,
    mut comp_interaction_res: ResMut<CompInteractionRes>,
    mouse_button_on_comp_input_res: Res<ButtonInput<MouseButton, MouseButtonValue>>,
    mouse_button_on_resize_handle_input_res: Res<ButtonInput<MouseButtonOnResizeHandle, ()>>,
    mouse_button_on_rotate_handle_input_res: Res<ButtonInput<MouseButtonOnRotateHandle, ()>>,
) {
    if mouse_button_on_comp_input_res.get_just_released().len() > 0
        || mouse_button_on_resize_handle_input_res
            .get_just_released()
            .len()
            > 0
        || mouse_button_on_rotate_handle_input_res
            .get_just_released()
            .len()
            > 0
    {
        match comp_interaction_res.interaction_mode {
            InteractionMode::Inserting {
                entity: maybe_entity,
                ..
            } => {
                if let Some(entity) = maybe_entity {
                    commands.entity(entity).insert(Selected {
                        timestamp: web_time::Instant::now(),
                    });
                    comp_interaction_res.interaction_tool = InteractionTool::Select;
                }
            }
            _ => {}
        };

        comp_interaction_res.interaction_mode = InteractionMode::None;
    }
}
