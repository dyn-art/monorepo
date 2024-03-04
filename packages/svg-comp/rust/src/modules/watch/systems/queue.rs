use crate::{
    events::SvgCompOutputEvent,
    modules::watch::{
        events::{
            CompositionChange, CompositionChangeOutputEvent, WatchedEntityChangesOutputEvent,
        },
        resources::{
            changed_components::ChangedComponentsRes, output_event_sender::OutputEventSenderRes,
        },
    },
};
use bevy_ecs::{
    change_detection::DetectChanges,
    system::{Res, ResMut},
};
use dyn_comp_core::resources::composition::CompositionRes;

pub fn queue_changed_components(
    mut changed_components_res: ResMut<ChangedComponentsRes>,
    output_event_sender_res: ResMut<OutputEventSenderRes>,
) {
    for (entity, changes) in changed_components_res.drain() {
        output_event_sender_res.push_event(SvgCompOutputEvent::WatchedEntityChanges(
            WatchedEntityChangesOutputEvent { entity, changes },
        ))
    }
}

pub fn queue_composition_changes(
    output_event_sender_res: ResMut<OutputEventSenderRes>,
    composition: Res<CompositionRes>,
) {
    // TODO: Can be granulated to avoid sending too much data, but for now, that's good enough
    if composition.is_changed() {
        output_event_sender_res.push_event(SvgCompOutputEvent::CompositionChange(
            CompositionChangeOutputEvent {
                change: CompositionChange {
                    size: composition.size,
                    root_nodes: composition.root_nodes.clone(),
                    viewport: composition.viewport,
                },
            },
        ))
    }
}
