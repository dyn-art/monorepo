use bevy_ecs::component::Component;

/// Marks an entity as the root or top-level entity.
#[derive(Component, Debug, Default, Copy, Clone)]
pub struct Root;

/// Marks an entity to be removed at the end of the update cycle.
#[derive(Component, Debug, Default, Copy, Clone)]
pub struct Removed;

/// Marks an entity's `Transform` as stale, signaling the need for recomputation.
/// Commonly used when a child transform in a `Group Node` changes.
#[derive(Component, Debug, Default, Copy, Clone)]
pub struct StaleTransform;

/// Marks an entity's `Size` as stale, signaling the need for recomputation.
/// This is typically triggered by changes in the entity's dimensions or scaling factors.
#[derive(Component, Debug, Default, Copy, Clone)]
pub struct StaleSize;
