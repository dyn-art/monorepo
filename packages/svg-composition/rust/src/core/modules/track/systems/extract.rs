use bevy_ecs::{
    component::Component,
    entity::Entity,
    query::Changed,
    system::{Query, Res, ResMut},
};
use dyn_composition::core::modules::node::components::mixins::{
    DimensionMixin, RelativeTransformMixin,
};
use dyn_svg_render::mixin_change::ToMixinChange;

use crate::core::modules::track::resources::{
    changed_components::ChangedComponentsRes,
    tracked_entities::{TrackableMixinType, TrackedEntitiesRes},
};

pub fn extract_tracked_mixin_changes(
    tracked_entities: Res<TrackedEntitiesRes>,
    mut changed: ResMut<ChangedComponentsRes>,
    query_dimension: Query<&DimensionMixin, Changed<DimensionMixin>>,
    query_relative_transform: Query<&RelativeTransformMixin, Changed<RelativeTransformMixin>>,
) {
    for (entity, component_types) in tracked_entities.tracked_entities.iter() {
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
