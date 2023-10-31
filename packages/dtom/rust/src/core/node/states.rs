use bevy_ecs::component::Component;
use serde::Serialize;
use specta::Type;

#[derive(Component, Serialize, Clone, Default, Debug, Type)]
pub struct Selected;
