use crate::{
    events::SvgCompOutputEvent,
    modules::watch::{
        events::{
            CompositionChangeOutputEvent, InteractionModeChangeOutputEvent, InteractionModeLabel,
            SelectionChangeOutputEvent, WatchedEntityChangesOutputEvent,
        },
        resources::{
            changed_components::ChangedComponentsRes, output_event_sender::OutputEventSenderRes,
        },
    },
};
use bevy_ecs::{
    change_detection::DetectChanges,
    entity::Entity,
    query::With,
    system::{Local, Query, Res, ResMut},
};
use dyn_comp_core::resources::composition::CompositionRes;
use dyn_comp_interaction::{components::Selected, resources::comp_interaction::CompInteractionRes};
use std::collections::HashSet;

pub fn queue_changed_components(
    mut changed_components_res: ResMut<ChangedComponentsRes>,
    output_event_sender_res: ResMut<OutputEventSenderRes>,
) {
    for (entity, changes) in changed_components_res.drain() {
        output_event_sender_res.push_event(SvgCompOutputEvent::WatchedEntityChange(
            WatchedEntityChangesOutputEvent { entity, changes },
        ))
    }
}

pub fn queue_composition_changes(
    output_event_sender_res: ResMut<OutputEventSenderRes>,
    comp_res: Res<CompositionRes>,
) {
    // TODO: Can be granulated to avoid sending too much data, but for now, that's good enough
    if comp_res.is_changed() {
        output_event_sender_res.push_event(SvgCompOutputEvent::CompositionChange(
            CompositionChangeOutputEvent {
                size: comp_res.size,
                root_nodes: comp_res.root_nodes.clone(),
                viewport: comp_res.viewport,
            },
        ))
    }
}

pub fn queue_selected_entities_changes(
    output_event_sender_res: ResMut<OutputEventSenderRes>,
    mut last_selected: Local<HashSet<Entity>>,
    selected_query: Query<Entity, With<Selected>>,
) {
    let current_selected: HashSet<Entity> = selected_query.iter().collect();

    // Check whether the set of selected entities has changed
    if *last_selected != current_selected {
        output_event_sender_res.push_event(SvgCompOutputEvent::SelectionChange(
            SelectionChangeOutputEvent {
                selected_entities: (&current_selected).into_iter().copied().collect(),
            },
        ));

        *last_selected = current_selected;
    }
}

pub fn queue_interaction_mode_changes(
    output_event_sender_res: ResMut<OutputEventSenderRes>,
    comp_interaction_res: Res<CompInteractionRes>,
    mut last_interaction_mode: Local<InteractionModeLabel>,
) {
    if comp_interaction_res.is_changed() {
        let current_interaction_mode: InteractionModeLabel =
            (&comp_interaction_res.interaction_mode).into();

        // Check whether the interaction mode has changed
        if *last_interaction_mode != current_interaction_mode {
            output_event_sender_res.push_event(SvgCompOutputEvent::InteractionModeChange(
                InteractionModeChangeOutputEvent {
                    interaction_mode: current_interaction_mode,
                },
            ));

            *last_interaction_mode = current_interaction_mode;
        }
    }
}
