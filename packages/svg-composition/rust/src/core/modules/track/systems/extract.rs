use std::collections::HashSet;

use bevy_ecs::{
    component::Component,
    entity::Entity,
    query::{Changed, With},
    system::{Local, Query, Res, ResMut},
};
use dyn_composition::core::modules::node::components::{
    mixins::{DimensionMixin, RelativeTransformMixin},
    states::Selected,
};

use crate::core::{
    events::{
        output_event::{OutputEvent, SelectionChangeEvent},
        output_event_queue::OutputEventQueue,
    },
    mixin_change::ToMixinChange,
    modules::track::resources::{
        changed_components::ChangedComponents,
        trackable_entities::{TrackableMixinType, TrackedEntities},
    },
};

pub fn extract_tracked_mixin_changes(
    tracked_entities: Res<TrackedEntities>,
    mut changed: ResMut<ChangedComponents>,
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
    changed_components: &mut ChangedComponents,
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
    query: Query<Entity, With<Selected>>,
    mut last_selected: Local<HashSet<Entity>>,
    mut output_event_queue: ResMut<OutputEventQueue>,
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
