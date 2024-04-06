use crate::{
    components::Selected,
    events::CursorUpOnCompInputEvent,
    resources::comp_interaction::{CompInteractionRes, InteractionMode, InteractionTool},
};
use bevy_ecs::{
    event::EventReader,
    system::{Commands, ResMut},
};

pub fn handle_cursor_up_on_comp_event(
    mut commands: Commands,
    mut event_reader: EventReader<CursorUpOnCompInputEvent>,
    mut comp_interaction_res: ResMut<CompInteractionRes>,
) {
    if event_reader.read().len() > 0 {
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
