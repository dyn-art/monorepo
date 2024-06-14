use bevy_ecs::component::Component;

/// Marks an entity as the root or top-level entity.
#[derive(Component, Debug, Default, Copy, Clone)]
pub struct Root;

/// Marks an entity to be removed at the end of the update cycle.
#[derive(Component, Debug, Default, Copy, Clone)]
pub struct Removed;

#[derive(Component, Debug, Default, Copy, Clone)]
pub struct StaleStaticLayout;
