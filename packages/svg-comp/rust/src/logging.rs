#[cfg(feature = "tracing")]
pub mod tracing {
    use bevy_ecs::{entity::Entity, world::World};

    pub fn log_entity_components(world: &World, entity: Entity) {
        let component_names = world
            .inspect_entity(entity)
            .iter()
            .map(|info| info.name())
            .collect::<Vec<_>>();

        if component_names.is_empty() {
            log::info!("Entity ({:?}) has no components.", entity);
        } else {
            log::info!(
                "Entity ({:?}) has the following components:\n - {}",
                entity,
                component_names.join("\n - ")
            );
        }
    }
}
