use bevy_ecs::component::Component;
use serde::Serialize;
#[cfg(feature = "cli")]
use specta::Type;

#[cfg_attr(feature = "cli", derive(Type))]
#[derive(Component, Serialize, Clone, Default, Debug)]
pub struct Node;

#[cfg_attr(feature = "cli", derive(Type))]
#[derive(Component, Serialize, Clone, Debug)]
pub struct Shape;

#[cfg_attr(feature = "cli", derive(Type))]
#[derive(Component, Serialize, Clone, Debug)]
pub struct Frame {
    clip_content: bool,
}

impl Default for Frame {
    fn default() -> Self {
        Self {
            clip_content: false,
        }
    }
}

#[cfg_attr(feature = "cli", derive(Type))]
#[derive(Component, Serialize, Clone, Default, Debug)]
pub struct Group;

#[cfg_attr(feature = "cli", derive(Type))]
#[derive(Component, Serialize, Clone, Default, Debug)]
pub struct Rectangle;
