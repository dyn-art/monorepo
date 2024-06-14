#![cfg(feature = "output_svg_element_changes")]

use crate::{
    events::{SvgBuilderOutputEvent, SvgElementChangesOutputEvent},
    resources::{
        changed_svg_bundles::{ChangedSvgBundle, ChangedSvgBundlesRes},
        output_event_sender::OutputEventSenderRes,
    },
    svg::svg_element::element_changes::SvgElementChanges,
};
use bevy_ecs::system::ResMut;

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
    mut changed_bundles: Vec<ChangedSvgBundle>,
    output_event_sender_res: &mut OutputEventSenderRes,
) {
    // Sort changed bundles by hierarchy level first, then by child index
    changed_bundles.sort_by(|a, b| {
        a.hierarchy_level
            .cmp(&b.hierarchy_level)
            // Note: Reverse the order because in a SVG, the top-most element appears last in the children "array",
            // contrary to the DtifCanvas convention where the first element (index = 0) is at the top
            .then(b.child_index.cmp(&a.child_index))
    });

    // Send output events
    for changed_bundle in changed_bundles {
        for element_changes in changed_bundle.elements_changes {
            let event = SvgBuilderOutputEvent::SvgElementChanges(SvgElementChangesOutputEvent {
                id: element_changes.id,
                changes: element_changes.changes,
            });
            output_event_sender_res.push_event(event);
        }
    }
}

fn queue_deferred_elements_changes(
    deferred_elements_changes: Vec<SvgElementChanges>,
    output_event_sender_res: &mut OutputEventSenderRes,
) {
    for deferred_elements_change in deferred_elements_changes {
        let event = SvgBuilderOutputEvent::SvgElementChanges(SvgElementChangesOutputEvent {
            id: deferred_elements_change.id,
            changes: deferred_elements_change.changes,
        });
        output_event_sender_res.push_event(event);
    }
}
