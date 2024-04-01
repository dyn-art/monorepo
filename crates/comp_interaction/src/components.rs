use bevy_ecs::component::Component;

#[derive(Component, Default, Debug, Clone, Copy)]
pub struct Selected;

#[derive(Component, Debug, Clone, Copy)]
pub struct Preselected {
    pub timestamp: web_time::Instant,
}

#[derive(Component, Default, Debug, Clone, Copy)]
pub struct Locked;
