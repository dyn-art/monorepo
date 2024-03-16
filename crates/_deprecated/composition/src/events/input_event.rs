//! Events received by the Composition

// Specified in specific plugins

use bevy_ecs::world::World;

pub trait InputEvent {
    fn send_to_ecs(self, world: &mut World);
}
