use std::collections::HashSet;

use bevy_ecs::{
    change_detection::DetectChanges,
    component::Component,
    entity::Entity,
    query::{Changed, With},
    system::{Local, Query, Res, ResMut},
};
use dyn_composition::core::modules::{
    interactive_composition::resources::{InteractionMode, InteractiveCompositionRes},
    node::components::{
        mixins::{DimensionMixin, RelativeTransformMixin},
        states::Selected,
    },
};

use crate::core::{
    events::{
        output_event::{
            InteractionModeChangeEvent, OutputEvent, RawInteractionMode, SelectionChangeEvent,
        },
        output_event_queue::OutputEventQueueRes,
    },
    mixin_change::ToMixinChange,
    modules::track::resources::{
        changed_components::ChangedComponentsRes,
        trackable_entities::{TrackableMixinType, TrackedEntitiesRes},
    },
};

pub fn extract_tracked_mixin_changes(
    tracked_entities: Res<TrackedEntitiesRes>,
    mut changed: ResMut<ChangedComponentsRes>,
    query_dimension: Query<&DimensionMixin, Changed<DimensionMixin>>,
    query_relative_transform: Query<&RelativeTransformMixin, Changed<RelativeTransformMixin>>,
) {
    for (entity, component_types) in tracked_entities.entities.iter() {
        for component_type in component_types {
            match component_type {
                TrackableMixinType::Dimension => {
                    handle_component_change(*entity, &query_dimension, &mut changed);
                }
                TrackableMixinType::RelativeTransform => {
                    handle_component_change(*entity, &query_relative_transform, &mut changed);
                }
            }
        }
    }
}

fn handle_component_change<T: Component + ToMixinChange>(
    entity: Entity,
    query: &Query<&T, Changed<T>>,
    changed_components: &mut ChangedComponentsRes,
) {
    if let Ok(component) = query.get(entity) {
        let changed_component = changed_components
            .changed_entities
            .entry(entity)
            .or_insert_with(Vec::new);
        changed_component.push(component.to_mixin_change());
    }
}

pub fn check_selected_changes(
    mut output_event_queue: ResMut<OutputEventQueueRes>,
    mut last_selected: Local<HashSet<Entity>>,
    query: Query<Entity, With<Selected>>,
) {
    let current_selected: HashSet<Entity> = query.iter().collect();

    // Check if the set of selected entities has changed
    if *last_selected != current_selected {
        output_event_queue.push_event(OutputEvent::SelectionChange(SelectionChangeEvent {
            selected: current_selected.clone().into_iter().collect(),
        }));

        // Update the local tracking set
        *last_selected = current_selected;
    }
}

pub fn check_interactive_composition_changes(
    mut output_event_queue: ResMut<OutputEventQueueRes>,
    mut last_raw_interaction_mode: Local<RawInteractionMode>,
    interactive_composition: Res<InteractiveCompositionRes>,
) {
    if interactive_composition.is_changed() {
        let current_raw_interaction_mode = match interactive_composition.interaction_mode {
            InteractionMode::None => RawInteractionMode::None,
            InteractionMode::Pressing { .. } => RawInteractionMode::Pressing,
            InteractionMode::Translating { .. } => RawInteractionMode::Translating,
        };

        // Check whether the interaction mode has changed
        if *last_raw_interaction_mode != current_raw_interaction_mode {
            output_event_queue.push_event(OutputEvent::InteractionModeChange(
                InteractionModeChangeEvent {
                    interaction_mode: current_raw_interaction_mode.clone(),
                },
            ));

            // Update the local tracking of the interaction mode
            *last_raw_interaction_mode = current_raw_interaction_mode;
        }
    }
}
