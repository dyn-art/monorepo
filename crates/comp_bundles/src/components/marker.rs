use bevy_ecs::component::Component;

/// Marker component that flags entities to be removed at the end of the update cycle.
#[derive(Component, Debug, Default, Copy, Clone)]
pub struct Removed;
