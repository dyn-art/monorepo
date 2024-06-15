use crate::resources::referencer::ReferencerRes;
use bevy_ecs::{
    entity::Entity,
    query::With,
    system::{Commands, Query, ResMut},
};
use bevy_hierarchy::DespawnRecursiveExt;
use dyn_arb_bundles::components::marker::Removed;

pub fn despawn_removed_entities_system(
    mut commands: Commands,
    mut referencer_res: ResMut<ReferencerRes>,
    query: Query<Entity, With<Removed>>,
) {
    for entity in query.iter() {
        referencer_res.remove_by_entity(&entity);
        commands.entity(entity).despawn_recursive();
    }
}
