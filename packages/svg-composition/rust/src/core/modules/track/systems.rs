use bevy_ecs::system::{Query, Res, ResMut};
use dyn_composition::core::modules::node::components::mixins::{
    DimensionMixin, RelativeTransformMixin,
};

use crate::core::{
    events::{
        output_event::{OutputEvent, TrackUpdateEvent},
        output_event_queue::OutputEventQueue,
    },
    mixin_change::ToMixinChange,
};

use super::resources::{TrackableMixinType, TrackedEntities};

pub fn track_changes(
    tracked_entities: Res<TrackedEntities>,
    mut output_event_queue: ResMut<OutputEventQueue>,
    query_dimension: Query<&DimensionMixin>,
    query_transform: Query<&RelativeTransformMixin>,
) {
    for (entity, component_types) in tracked_entities.entities.iter() {
        for component_type in component_types {
            match component_type {
                TrackableMixinType::Dimension => {
                    if let Ok(dimension) = query_dimension.get(*entity) {
                        output_event_queue.push_event(OutputEvent::TrackUpdate(TrackUpdateEvent {
                            id: entity.clone(),
                            updates: vec![dimension.to_mixin_change()],
                        }));
                    }
                }
                TrackableMixinType::RelativeTransform => {
                    if let Ok(relative_transform) = query_transform.get(*entity) {
                        output_event_queue.push_event(OutputEvent::TrackUpdate(TrackUpdateEvent {
                            id: entity.clone(),
                            updates: vec![relative_transform.to_mixin_change()],
                        }));
                    }
                }
            }
        }
    }
}
