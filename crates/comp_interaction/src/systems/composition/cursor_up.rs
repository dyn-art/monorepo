use crate::{
    components::Selected,
    events::CursorUpOnCompInputEvent,
    input::{
        button_input::ButtonInput,
        mouse::{MouseButton, MouseButtonValue},
    },
    resources::comp_interaction::{CompInteractionRes, InteractionMode, InteractionTool},
};
use bevy_ecs::{
    event::EventReader,
    system::{Commands, ResMut},
};

pub fn cursor_up_on_comp_input_system(
    mut event_reader: EventReader<CursorUpOnCompInputEvent>,
    mut mouse_button_input_res: ResMut<ButtonInput<MouseButton, MouseButtonValue>>,
) {
    for event in event_reader.read() {
        log::info!("[cursor_up_on_comp_input_system] {:?}", event.button);
        mouse_button_input_res.release(event.button);
    }
}

pub fn cursor_up_on_comp_system(
    mut commands: Commands,
    mut comp_interaction_res: ResMut<CompInteractionRes>,
    mouse_button_input_res: ResMut<ButtonInput<MouseButton, MouseButtonValue>>,
) {
    if mouse_button_input_res.get_just_released().len() > 0 {
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
