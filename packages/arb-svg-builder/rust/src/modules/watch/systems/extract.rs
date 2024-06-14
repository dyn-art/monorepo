use crate::modules::watch::{
    component_change::ToComponentChange,
    resources::{
        changed_components::ChangedComponentsRes,
        watched_entities::{WatchableComponentVariant, WatchedEntitiesRes},
    },
};
use bevy_ecs::{
    query::Changed,
    system::{Query, Res, ResMut},
};
use bevy_transform::components::{GlobalTransform, Transform};
use dyn_arb_bundles::components::mixins::SizeMixin;

pub fn extract_changed_components(
    watched_entities_res: Res<WatchedEntitiesRes>,
    mut changed_components_res: ResMut<ChangedComponentsRes>,
    query_size_mixin: Query<&SizeMixin, Changed<SizeMixin>>,
    query_transform: Query<&Transform, Changed<Transform>>,
    query_global_transform: Query<&GlobalTransform, Changed<GlobalTransform>>,
) {
    for (entity, arbopnent_variants) in watched_entities_res.get_watched_entities().iter() {
        for component_variant in arbopnent_variants {
            match component_variant {
                WatchableComponentVariant::Size => {
                    if let Ok(component) = query_size_mixin.get(*entity) {
                        changed_components_res
                            .push_change(*entity, component.to_component_change());
                    }
                }
                WatchableComponentVariant::Transform => {
                    if let Ok(component) = query_transform.get(*entity) {
                        changed_components_res
                            .push_change(*entity, component.to_component_change());
                    }
                }
                WatchableComponentVariant::GlobalTransform => {
                    if let Ok(component) = query_global_transform.get(*entity) {
                        changed_components_res
                            .push_change(*entity, component.to_component_change());
                    }
                }
            }
        }
    }
}
