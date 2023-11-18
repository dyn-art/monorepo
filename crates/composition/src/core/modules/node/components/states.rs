use bevy_ecs::component::Component;
use serde::Serialize;
use specta::Type;

// TODO: Define here or in interactive composition?

#[derive(Component, Serialize, Clone, Default, Debug, Type)]
pub struct Selected;
