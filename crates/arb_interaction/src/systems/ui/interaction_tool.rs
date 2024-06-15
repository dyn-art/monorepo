use crate::{
    components::Selected,
    events::InteractionToolChangedInputEvent,
    resources::arb_interaction::{ArbInteractionRes, InteractionTool},
};
use bevy_ecs::{
    entity::Entity,
    event::EventReader,
    query::With,
    system::{Commands, Query, ResMut},
};
use dyn_arb_bundles::components::nodes::ArbNode;

pub fn interaction_tool_changed_input_system(
    mut commands: Commands,
    mut event_reader: EventReader<InteractionToolChangedInputEvent>,
    mut arb_interaction_res: ResMut<ArbInteractionRes>,
    selected_node_query: Query<Entity, (With<ArbNode>, With<Selected>)>,
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

        arb_interaction_res.interaction_tool = event.tool;
    }
}
