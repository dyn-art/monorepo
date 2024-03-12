use bevy_ecs::component::Component;

#[derive(Component, Default, Debug, Clone, Copy)]
pub struct Selected;

#[derive(Component, Default, Debug, Clone, Copy)]
pub struct Locked;
