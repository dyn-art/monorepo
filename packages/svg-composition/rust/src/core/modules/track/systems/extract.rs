use bevy_ecs::{
    component::Component,
    entity::Entity,
    query::Changed,
    system::{Query, Res, ResMut},
};
use dyn_composition::modules::node::components::mixins::{DimensionMixin, RelativeTransformMixin};

use crate::core::modules::track::{
    mixin_change::{MixinType, ToMixinChange},
    resources::{changed_components::ChangedComponentsRes, tracked_entities::TrackedEntitiesRes},
};

pub fn extract_tracked_mixin_changes(
    tracked_entities: Res<TrackedEntitiesRes>,
    mut changed: ResMut<ChangedComponentsRes>,
    query_dimension_mixin: Query<&DimensionMixin, Changed<DimensionMixin>>,
    query_relative_transform: Query<&RelativeTransformMixin, Changed<RelativeTransformMixin>>,
) {
    for (entity, component_types) in tracked_entities.tracked_entities.iter() {
        for component_type in component_types {
            match component_type {
                MixinType::Dimension => {
                    handle_component_change(*entity, &query_dimension_mixin, &mut changed);
                }
                MixinType::RelativeTransform => {
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
