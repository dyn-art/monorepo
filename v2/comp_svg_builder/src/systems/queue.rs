use std::collections::HashMap;

use bevy_ecs::{entity::Entity, system::ResMut};

use crate::{
    events::{SvgBuilderOutputEvent, SvgElementChangesEvent},
    resources::{changed_svg_nodes::ChangedSvgNodesRes, output_event_sender::OutputEventSenderRes},
};

pub fn queue_svg_node_changes(
    mut changed_svg_nodes_res: ResMut<ChangedSvgNodesRes>,
    output_event_sender_res: ResMut<OutputEventSenderRes>,
) {
    let mut changes = changed_svg_nodes_res.drain();

    // Preparing a lookup map for parent positions
    let parent_positions: HashMap<Entity, usize> = changes
        .iter()
        .enumerate()
        .map(|(index, node)| (node.entity, index))
        .collect();

    // Sorting changes with consideration for parent-child relationships and indices
    changes.sort_by(|a, b| {
        let pos_a = a
            .parent_entity
            .and_then(|parent| parent_positions.get(&parent))
            .unwrap_or(&usize::MAX);
        let pos_b = b
            .parent_entity
            .and_then(|parent| parent_positions.get(&parent))
            .unwrap_or(&usize::MAX);
        pos_a.cmp(&pos_b).then_with(|| a.index.cmp(&b.index))
    });

    // Iterating through sorted changes to send events
    for changed_svg_node in changes {
        for changes in changed_svg_node.changes {
            let event = SvgBuilderOutputEvent::ElementChanges(SvgElementChangesEvent { changes });
            output_event_sender_res.push_event(event);
        }
    }
}
