use crate::{
    components::Selected,
    events::InteractionToolChangedInputEvent,
    resources::comp_interaction::{CompInteractionRes, InteractionTool},
};
use bevy_ecs::{
    entity::Entity,
    event::EventReader,
    query::With,
    system::{Commands, Query, ResMut},
};
use dyn_comp_bundles::components::nodes::CompNode;

pub fn interaction_tool_changed_input_system(
    mut commands: Commands,
    mut event_reader: EventReader<InteractionToolChangedInputEvent>,
    mut comp_interaction_res: ResMut<CompInteractionRes>,
    selected_node_query: Query<Entity, (With<CompNode>, With<Selected>)>,
) {
    if let Some(event) = event_reader.read().last() {
        match &event.tool {
            InteractionTool::Shape { .. } => {
                for entity in selected_node_query.iter() {
                    commands.entity(entity).remove::<Selected>();
                }
            }
            _ => {}
        }

        comp_interaction_res.interaction_tool = event.tool;
    }
}
