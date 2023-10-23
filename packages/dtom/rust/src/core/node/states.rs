use bevy_ecs::component::Component;
use serde::Serialize;
#[cfg(feature = "cli")]
use specta::Type;

#[cfg_attr(feature = "cli", derive(Type))]
#[derive(Component, Serialize, Clone, Default, Debug)]
pub struct Selected;
