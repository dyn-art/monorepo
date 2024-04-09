use crate::{
    components::Selected,
    events::CursorUpOnCompInputEvent,
    input::mouse::{
        MouseButtonButtonInputRes, MouseButtonOnEntityButtonInputRes,
        MouseButtonOnResizeHandleButtonInputRes, MouseButtonOnRotateHandleButtonInputRes,
    },
    resources::comp_interaction::{CompInteractionRes, InteractionMode, InteractionTool},
};
use bevy_ecs::{
    event::EventReader,
    system::{Commands, Res, ResMut},
};

pub fn cursor_up_on_comp_input_system(
    mut event_reader: EventReader<CursorUpOnCompInputEvent>,
    mut mouse_button_on_comp_input_res: ResMut<MouseButtonButtonInputRes>,
    mut mouse_button_on_resize_handle_input_res: ResMut<MouseButtonOnResizeHandleButtonInputRes>,
    mut mouse_button_on_rotate_handle_input_res: ResMut<MouseButtonOnRotateHandleButtonInputRes>,
    mut mouse_button_on_entity_res: ResMut<MouseButtonOnEntityButtonInputRes>,
) {
    for event in event_reader.read() {
        log::info!("[cursor_up_on_comp_input_system] {:?}", event.button);
        mouse_button_on_comp_input_res.release(event.button);
        mouse_button_on_resize_handle_input_res
            .release_unretained(|key, _| key.button != event.button);
        mouse_button_on_rotate_handle_input_res
            .release_unretained(|key, _| key.button != event.button);
        mouse_button_on_entity_res.release_unretained(|key, _| key.button != event.button);
    }
}

pub fn cursor_up_on_comp_system(
    mut commands: Commands,
    mut comp_interaction_res: ResMut<CompInteractionRes>,
    mouse_button_on_comp_input_res: Res<MouseButtonButtonInputRes>,
    mouse_button_on_resize_handle_input_res: Res<MouseButtonOnResizeHandleButtonInputRes>,
    mouse_button_on_rotate_handle_input_res: Res<MouseButtonOnRotateHandleButtonInputRes>,
) {
    if mouse_button_on_comp_input_res.was_any_just_released() {
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

    if mouse_button_on_resize_handle_input_res.was_any_just_released()
        || mouse_button_on_rotate_handle_input_res.was_any_just_released()
    {
        comp_interaction_res.interaction_mode = InteractionMode::None;
    }
}
