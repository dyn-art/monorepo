use crate::{
    events::{SvgBuilderOutputEvent, SvgElementChangesOutputEvent},
    resources::{
        changed_svg_nodes::{ChangedSvgNode, ChangedSvgNodesRes},
        output_event_sender::OutputEventSenderRes,
    },
};
use bevy_ecs::{entity::Entity, system::ResMut};
use std::collections::{HashMap, VecDeque};

pub fn queue_svg_node_changes(
    mut changed_svg_nodes_res: ResMut<ChangedSvgNodesRes>,
    output_event_sender_res: ResMut<OutputEventSenderRes>,
) {
    let changes = changed_svg_nodes_res.drain();
    let changes_length = changes.len();

    // Mapping of parent id to children, still maintaining order
    let mut parent_to_children: HashMap<Option<Entity>, Vec<ChangedSvgNode>> = HashMap::new();

    // Separate changes into roots (None) and children grouped by parent
    for change in changes {
        match change.parent_entity {
            Some(parent) => {
                parent_to_children
                    .entry(Some(parent))
                    .or_default()
                    .push(change);
            }
            None => {
                parent_to_children.entry(None).or_default().push(change);
            }
        }
    }

    // Sort children within their parent grouping by their index
    for children in parent_to_children.values_mut() {
        children.sort_unstable_by_key(|change| change.index);
    }

    // Process root nodes in depth-first order, taking ownership of the data
    let mut sorted_changes: Vec<ChangedSvgNode> = Vec::with_capacity(changes_length);
    let mut stack: VecDeque<ChangedSvgNode> = parent_to_children
        .remove(&None)
        .unwrap_or_else(Vec::new)
        .into_iter()
        .collect();

    while let Some(change) = stack.pop_back() {
        if let Some(children) = parent_to_children.remove(&Some(change.entity)) {
            for child in children.into_iter().rev() {
                stack.push_back(child);
            }
        }

        sorted_changes.push(change);
    }

    log::info!("[queue_svg_node_changes] After sort: {:#?}", sorted_changes);

    // Process sorted changes to send events
    for changed_svg_node in sorted_changes {
        for changes in changed_svg_node.changes {
            let event =
                SvgBuilderOutputEvent::ElementChanges(SvgElementChangesOutputEvent { changes });
            output_event_sender_res.push_event(event);
        }
    }
}
