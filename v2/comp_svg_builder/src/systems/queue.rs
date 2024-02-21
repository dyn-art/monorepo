use std::collections::HashMap;

use bevy_ecs::{entity::Entity, system::ResMut};

use crate::{
    events::{SVGBuilderOutputEvent, SVGElementChangesEvent},
    resources::{
        changed_svg_nodes::{ChangedSVGNode, ChangedSVGNodesRes},
        output_event_sender::OutputEventSenderRes,
    },
};

pub fn queue_svg_node_changes(
    mut changed_svg_nodes_res: ResMut<ChangedSVGNodesRes>,
    output_event_sender_res: ResMut<OutputEventSenderRes>,
) {
    let mut changes: Vec<ChangedSVGNode> = changed_svg_nodes_res.changes.drain(..).collect();

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
        for change in changed_svg_node.changes {
            let event = SVGBuilderOutputEvent::ElementChanges(SVGElementChangesEvent(change));
            let _ = output_event_sender_res.sender.send(event);
        }
    }
}
