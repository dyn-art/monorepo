#![cfg(feature = "output_svg_element_changes")]

use crate::{
    events::{SvgBuilderOutputEvent, SvgElementChangesOutputEvent},
    resources::{
        changed_svg_bundles::{ChangedSvgBundle, ChangedSvgBundlesRes},
        output_event_sender::OutputEventSenderRes,
    },
    svg::svg_element::element_changes::SvgElementChanges,
};
use bevy_ecs::{entity::Entity, system::ResMut};
use std::collections::{HashMap, HashSet, VecDeque};

pub fn queue_svg_bundle_changes(
    mut changed_svg_bundles_res: ResMut<ChangedSvgBundlesRes>,
    mut output_event_sender_res: ResMut<OutputEventSenderRes>,
) {
    let changed_bundles = changed_svg_bundles_res.drain_changes();
    queue_changed_bundles(changed_bundles, &mut output_event_sender_res);

    let deferred_elements_changes = changed_svg_bundles_res.drain_deferred_changes();
    queue_deferred_elements_changes(deferred_elements_changes, &mut output_event_sender_res);
}

fn queue_changed_bundles(
    changed_bundles: Vec<ChangedSvgBundle>,
    output_event_sender_res: &mut OutputEventSenderRes,
) {
    let changes_length = changed_bundles.len();

    // Map children to their parents
    let mut parent_to_children_map = create_parent_to_children_map(changed_bundles);

    // Sort children within their parent grouping by their index
    for children in parent_to_children_map.values_mut() {
        children.sort_by(|a, b| a.index.cmp(&b.index));
    }

    // Process root bundles in depth-first order
    let sorted_changed_bundles = process_root_bundles(&mut parent_to_children_map, changes_length);

    // Send output events
    for changed_bundle in sorted_changed_bundles {
        for element_changes in changed_bundle.elements_changes {
            let event = SvgBuilderOutputEvent::SvgElementChanges(SvgElementChangesOutputEvent {
                changes: element_changes,
            });
            output_event_sender_res.push_event(event);
        }
    }
}

fn create_parent_to_children_map(
    changed_bundles: Vec<ChangedSvgBundle>,
) -> HashMap<Option<Entity>, Vec<ChangedSvgBundle>> {
    let changed_entities_set: HashSet<Entity> =
        changed_bundles.iter().map(|bundle| bundle.entity).collect();
    let mut parent_to_children_map: HashMap<Option<Entity>, Vec<ChangedSvgBundle>> = HashMap::new();

    // Separate changes into roots (None, that have no parent in the current changes)
    // and children grouped by parent entities
    for changed_bundle in changed_bundles {
        let is_orphan = changed_bundle
            .parent_entity
            .map_or(true, |parent| !changed_entities_set.contains(&parent));

        if is_orphan {
            parent_to_children_map
                .entry(None)
                .or_default()
                .push(changed_bundle);
        } else {
            parent_to_children_map
                .entry(changed_bundle.parent_entity)
                .or_default()
                .push(changed_bundle);
        }
    }

    return parent_to_children_map;
}

fn process_root_bundles(
    parent_to_children: &mut HashMap<Option<Entity>, Vec<ChangedSvgBundle>>,
    capacity: usize,
) -> Vec<ChangedSvgBundle> {
    let mut sorted_changed_bundles: Vec<ChangedSvgBundle> = Vec::with_capacity(capacity);
    let mut root_bundles: VecDeque<ChangedSvgBundle> = parent_to_children
        .remove(&None)
        .unwrap_or_else(Vec::new)
        .into_iter()
        .collect();

    // Process root bundles in depth-first order
    while let Some(change) = root_bundles.pop_front() {
        if let Some(children) = parent_to_children.remove(&Some(change.entity)) {
            for child in children {
                root_bundles.push_back(child);
            }
        }
        sorted_changed_bundles.push(change);
    }

    return sorted_changed_bundles;
}

fn queue_deferred_elements_changes(
    deferred_elements_changes: Vec<SvgElementChanges>,
    output_event_sender_res: &mut OutputEventSenderRes,
) {
    // Send output events
    for deferred_elements_change in deferred_elements_changes {
        let event = SvgBuilderOutputEvent::SvgElementChanges(SvgElementChangesOutputEvent {
            changes: deferred_elements_change,
        });
        output_event_sender_res.push_event(event);
    }
}
