use crate::{
    components::Selected,
    events::InteractionToolChangedInputEvent,
    resources::cnv_interaction::{CnvInteractionRes, InteractionTool},
};
use bevy_ecs::{
    entity::Entity,
    event::EventReader,
    query::With,
    system::{Commands, Query, ResMut},
};
use dyn_cnv_bundles::components::nodes::CnvNode;

pub fn interaction_tool_changed_input_system(
    mut commands: Commands,
    mut event_reader: EventReader<InteractionToolChangedInputEvent>,
    mut cnv_interaction_res: ResMut<CnvInteractionRes>,
    selected_node_query: Query<Entity, (With<CnvNode>, With<Selected>)>,
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

        cnv_interaction_res.interaction_tool = event.tool;
    }
}
