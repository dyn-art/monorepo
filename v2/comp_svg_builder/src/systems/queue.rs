#![cfg(feature = "output_svg_element_changes")]

use crate::{
    events::{SvgBuilderOutputEvent, SvgElementChangesOutputEvent},
    resources::{
        changed_svg_bundles::{ChangedSvgBundle, ChangedSvgBundlesRes},
        output_event_sender::OutputEventSenderRes,
    },
};
use bevy_ecs::{entity::Entity, system::ResMut};
use std::collections::{HashMap, HashSet, VecDeque};

pub fn queue_svg_bundle_changes(
    mut changed_svg_bundles_res: ResMut<ChangedSvgBundlesRes>,
    output_event_sender_res: ResMut<OutputEventSenderRes>,
) {
    let changed_bundles = changed_svg_bundles_res.drain_changes();
    let deferred_elements_changes = changed_svg_bundles_res.drain_deferred_changes();
    let changes_length = changed_bundles.len();

    let entities_set: HashSet<Entity> =
        changed_bundles.iter().map(|bundle| bundle.entity).collect();

    // Mapping of parent id to children, still maintaining order
    let mut parent_to_children: HashMap<Option<Entity>, Vec<ChangedSvgBundle>> = HashMap::new();

    // Separate changes into roots (None, that have no parent in the current changes)
    // and children grouped by parent entities
    for changed_bundle in changed_bundles {
        let is_orphan = changed_bundle
            .parent_entity
            .map_or(true, |parent| !entities_set.contains(&parent));

        if is_orphan {
            parent_to_children
                .entry(None)
                .or_default()
                .push(changed_bundle);
        } else {
            parent_to_children
                .entry(changed_bundle.parent_entity)
                .or_default()
                .push(changed_bundle);
        }
    }

    // Sort children within their parent grouping by their index
    for children in parent_to_children.values_mut() {
        children.sort_by(|a, b| b.index.cmp(&a.index));
    }

    // Process root nodes in depth-first order
    let mut sorted_changed_bundles: Vec<ChangedSvgBundle> = Vec::with_capacity(changes_length);
    let mut root_bundles: VecDeque<ChangedSvgBundle> = parent_to_children
        .remove(&None)
        .unwrap_or_else(Vec::new)
        .into_iter()
        .collect();

    while let Some(change) = root_bundles.pop_back() {
        if let Some(children) = parent_to_children.remove(&Some(change.entity)) {
            for child in children.into_iter().rev() {
                root_bundles.push_back(child);
            }
        }

        sorted_changed_bundles.push(change);
    }

    // Send output events for sorted changes
    for changed_bundle in sorted_changed_bundles {
        for element_changes in changed_bundle.elements_changes {
            let event = SvgBuilderOutputEvent::SvgElementChanges(SvgElementChangesOutputEvent {
                changes: element_changes,
            });
            output_event_sender_res.push_event(event);
        }
    }

    // Send output events for deferred changes
    for deferred_elements_change in deferred_elements_changes {
        let event = SvgBuilderOutputEvent::SvgElementChanges(SvgElementChangesOutputEvent {
            changes: deferred_elements_change,
        });
        output_event_sender_res.push_event(event);
    }
}
