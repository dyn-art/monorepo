use bevy_ecs::{entity::Entity, world::World};

/// Can be used to log components in a Bevy system using:
/// `commands.entity(entity).add(log_components_command);`
pub fn log_components_command(entity: Entity, world: &mut World) {
    let debug_infos: Vec<_> = world
        .inspect_entity(entity)
        .into_iter()
        .map(|component_info| component_info.name())
        .collect();
    log::info!("Entity {:?}: {:?}", entity, debug_infos);
}
