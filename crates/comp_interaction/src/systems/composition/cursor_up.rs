use crate::{
    components::Selected,
    events::CursorUpOnCompInputEvent,
    input::mouse::{
        MouseButton, MouseButtonButtonInput, MouseButtonOnResizeHandleButtonInput,
        MouseButtonOnRotateHandleButtonInput,
    },
    resources::comp_interaction::{CompInteractionRes, InteractionMode, InteractionTool},
};
use bevy_ecs::{
    event::EventReader,
    system::{Commands, Res, ResMut},
};

pub fn cursor_up_on_comp_input_system(
    mut event_reader: EventReader<CursorUpOnCompInputEvent>,
    mut mouse_button_on_comp_input_res: ResMut<MouseButtonButtonInput>,
    mut mouse_button_on_resize_handle_input_res: ResMut<MouseButtonOnResizeHandleButtonInput>,
    mut mouse_button_on_rotate_handle_input_res: ResMut<MouseButtonOnRotateHandleButtonInput>,
) {
    for event in event_reader.read() {
        log::info!("[cursor_up_on_comp_input_system] {:?}", event.button);
        mouse_button_on_comp_input_res.release(event.button);
        if event.button == MouseButton::Left {
            mouse_button_on_resize_handle_input_res.release_all();
            mouse_button_on_rotate_handle_input_res.release_all();
        }
    }
}

pub fn cursor_up_on_comp_system(
    mut commands: Commands,
    mut comp_interaction_res: ResMut<CompInteractionRes>,
    mouse_button_on_comp_input_res: Res<MouseButtonButtonInput>,
    mouse_button_on_resize_handle_input_res: Res<MouseButtonOnResizeHandleButtonInput>,
    mouse_button_on_rotate_handle_input_res: Res<MouseButtonOnRotateHandleButtonInput>,
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
