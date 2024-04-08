use super::button_input::ButtonInput;
use crate::resources::comp_interaction::XYWH;
/**
 * -----------------------------------------------------------------------------
 * This file includes code derived from the project bevyengine/bevy by @bevyengine.
 * Project Repository: https://github.com/bevyengine/bevy/blob/main/crates/bevy_input/src/mouse.rs
 *
 * Date of Import: 08 April 2024
 * -----------------------------------------------------------------------------
 * The code included in this file is licensed under the MIT License,
 * as per the original project by @bevyengine.
 * For the license text, see: https://github.com/bevyengine/bevy/blob/main/LICENSE-MIT
 * -----------------------------------------------------------------------------
 */
use bevy_ecs::entity::Entity;
use glam::Vec2;

/// A button on a mouse device.
///
/// ## Usage
///
/// It is used as the generic `T` value of an [`ButtonInput`] to create a `bevy`
/// resource.
///
/// ## Updating
///
/// The resource is updated inside of the [`mouse_button_input_system`].
#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[cfg_attr(feature = "serde_support", derive(serde::Deserialize, specta::Type))]
pub enum MouseButton {
    /// The left mouse button.
    Left,
    /// The right mouse button.
    Right,
    /// The middle mouse button.
    Middle,
    /// The back mouse button.
    Back,
    /// The forward mouse button.
    Forward,
    /// Another mouse button with the associated number.
    Other(u16),
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde_support", derive(serde::Deserialize, specta::Type))]
pub struct MouseButtonValue {
    pub position: Vec2,
}

pub type MouseButtonButtonInput = ButtonInput<MouseButton, MouseButtonValue>;

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[cfg_attr(feature = "serde_support", derive(serde::Deserialize, specta::Type))]
pub struct MouseButtonOnEntity {
    pub button: MouseButton,
    pub entity: Entity,
}

pub type MouseButtonOnEntityButtonInput = ButtonInput<MouseButtonOnEntity, MouseButtonValue>;

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[cfg_attr(feature = "serde_support", derive(serde::Deserialize, specta::Type))]
pub struct MouseButtonOnResizeHandle {
    pub button: MouseButton,
    pub corner: u8,
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde_support", derive(serde::Deserialize, specta::Type))]
pub struct MouseButtonOnResizeHandleValue {
    pub initial_bounds: XYWH,
    pub rotation_rad: f32,
}

pub type MouseButtonOnResizeHandleButtonInput =
    ButtonInput<MouseButtonOnResizeHandle, MouseButtonOnResizeHandleValue>;

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[cfg_attr(feature = "serde_support", derive(serde::Deserialize, specta::Type))]
pub struct MouseButtonOnRotateHandle {
    pub button: MouseButton,
    pub corner: u8,
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde_support", derive(serde::Deserialize, specta::Type))]
pub struct MouseButtonOnRotateHandleValue {
    pub initial_rotation_rad: f32,
}

pub type MouseButtonOnRotateHandleButtonInput =
    ButtonInput<MouseButtonOnRotateHandle, MouseButtonOnRotateHandleValue>;
