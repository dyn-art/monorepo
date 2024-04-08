/**
 * -----------------------------------------------------------------------------
 * This file includes code derived from the project bevyengine/bevy by @bevyengine.
 * Project Repository: https://github.com/bevyengine/bevy/blob/main/crates/bevy_input/src/button_input.rs
 *
 * Date of Import: 08 April 2024
 * -----------------------------------------------------------------------------
 * The code included in this file is licensed under the MIT License,
 * as per the original project by @bevyengine.
 * For the license text, see: https://github.com/bevyengine/bevy/blob/main/LICENSE-MIT
 * -----------------------------------------------------------------------------
 */
use bevy_ecs::system::Resource;
use std::{collections::BTreeMap, hash::Hash};

/// A "press-able" input of type `T`.
///
/// ## Usage
///
/// This type can be used as a resource to keep the current state of an input, by reacting to
/// events from the input. For a given input value:
///
/// * [`ButtonInput::pressed`] will return `true` between a press and a release event.
/// * [`ButtonInput::just_pressed`] will return `true` for one frame after a press event.
/// * [`ButtonInput::just_released`] will return `true` for one frame after a release event.
///
/// ## Multiple systems
///
/// In case multiple systems are checking for [`ButtonInput::just_pressed`] or [`ButtonInput::just_released`]
/// but only one should react, for example in the case of triggering
/// [`State`] change, you should consider clearing the input state, either by:
///
/// * Using [`ButtonInput::clear_just_pressed`] or [`ButtonInput::clear_just_released`] instead.
/// * Calling [`ButtonInput::clear`] or [`ButtonInput::reset`] immediately after the state change.
///
/// ## Performance
///
/// For all operations, the following conventions are used:
/// - **n** is the number of stored inputs.
/// - **m** is the number of input arguments passed to the method.
/// - **\***-suffix denotes an amortized cost.
/// - **~**-suffix denotes an expected cost.
///
/// See Rust's [std::collections doc on performance](https://doc.rust-lang.org/std/collections/index.html#performance) for more details on the conventions used here.
///
/// | **[`ButtonInput`] operations**          | **Computational complexity** |
/// |-----------------------------------|------------------------------------|
/// | [`ButtonInput::any_just_pressed`]       | *O*(m*n)                     |
/// | [`ButtonInput::any_just_released`]      | *O*(m*n)                     |
/// | [`ButtonInput::any_pressed`]            | *O*(m*n)                     |
/// | [`ButtonInput::get_just_pressed`]       | *O*(n)                       |
/// | [`ButtonInput::get_just_released`]      | *O*(n)                       |
/// | [`ButtonInput::get_pressed`]            | *O*(n)                       |
/// | [`ButtonInput::just_pressed`]           | *O*(1)~                      |
/// | [`ButtonInput::just_released`]          | *O*(1)~                      |
/// | [`ButtonInput::pressed`]                | *O*(1)~                      |
/// | [`ButtonInput::press`]                  | *O*(1)~*                     |
/// | [`ButtonInput::release`]                | *O*(1)~*                     |
/// | [`ButtonInput::release_all`]            | *O*(n)~*                     |
/// | [`ButtonInput::clear_just_pressed`]     | *O*(1)~                      |
/// | [`ButtonInput::clear_just_released`]    | *O*(1)~                      |
/// | [`ButtonInput::reset_all`]              | *O*(n)                       |
/// | [`ButtonInput::clear`]                  | *O*(n)                       |
///
/// ## Window focus
///
/// `ButtonInput<KeyCode>` is tied to window focus. For example, if the user holds a button
/// while the window loses focus, [`ButtonInput::just_released`] will be triggered. Similarly if the window
/// regains focus, [`ButtonInput::just_pressed`] will be triggered. Currently this happens even if the
/// focus switches from one Bevy window to another (for example because a new window was just spawned).
///
/// `ButtonInput<GamepadButton>` is independent of window focus.
///
/// ## Note
///
/// When adding this resource for a new input type, you should:
///
/// * Call the [`ButtonInput::press`] method for each press event.
/// * Call the [`ButtonInput::release`] method for each release event.
/// * Call the [`ButtonInput::clear`] method at each frame start, before processing events.
///
/// Note: Calling `clear` from a [`ResMut`] will trigger change detection.
/// It may be preferable to use [`DetectChangesMut::bypass_change_detection`]
/// to avoid causing the resource to always be marked as changed.
///
///[`ResMut`]: bevy_ecs::system::ResMut
///[`DetectChangesMut::bypass_change_detection`]: bevy_ecs::change_detection::DetectChangesMut::bypass_change_detection
#[derive(Resource, Debug, Clone)]
pub struct ButtonInput<
    T: Copy + Eq + Hash + Ord + Send + Sync + 'static,
    U: Copy + Send + Sync + 'static,
> {
    /// A collection of every button that is currently being pressed.
    pressed: BTreeMap<T, U>,
    /// A collection of every button that has just been pressed.
    just_pressed: BTreeMap<T, U>,
    /// A collection of every button that has just been released.
    just_released: BTreeMap<T, U>,
}

impl<T: Copy + Eq + Hash + Ord + Send + Sync + 'static, U: Copy + Send + Sync + 'static> Default
    for ButtonInput<T, U>
{
    fn default() -> Self {
        Self {
            pressed: Default::default(),
            just_pressed: Default::default(),
            just_released: Default::default(),
        }
    }
}

impl<T, U> ButtonInput<T, U>
where
    T: Copy + Eq + Hash + Ord + Send + Sync + 'static,
    U: Copy + Send + Sync + 'static,
{
    /// Registers a press for the given `input`.
    pub fn press(&mut self, input: T, value: U) {
        // Returns `None` if the `input` wasn't pressed.
        if self.pressed.insert(input, value).is_none() {
            self.just_pressed.insert(input, value);
        }
    }

    /// Returns `true` if the `input` has been pressed.
    pub fn pressed(&self, input: T) -> bool {
        self.pressed.contains_key(&input)
    }

    /// Returns `true` if any item in `inputs` has been pressed.
    pub fn any_pressed(&self, inputs: impl IntoIterator<Item = T>) -> bool {
        inputs.into_iter().any(|it| self.pressed(it))
    }

    /// Returns `true` if all items in `inputs` have been pressed.
    pub fn all_pressed(&self, inputs: impl IntoIterator<Item = T>) -> bool {
        inputs.into_iter().all(|it| self.pressed(it))
    }

    /// Registers a release for the given `input`.
    pub fn release(&mut self, input: T) {
        // Returns `true` if the `input` was pressed.
        if let Some((_input, value)) = self.pressed.remove_entry(&input) {
            self.just_released.insert(_input, value);
        }
    }

    /// Registers a release for all currently pressed inputs.
    pub fn release_all(&mut self) {
        // Move all items from pressed into just_released
        self.just_released.extend(std::mem::take(&mut self.pressed));
    }

    /// Returns `true` if the `input` has been pressed during the current frame.
    ///
    /// Note: This function does not imply information regarding the current state of [`ButtonInput::pressed`] or [`ButtonInput::just_released`].
    pub fn just_pressed(&self, input: T) -> bool {
        self.just_pressed.contains_key(&input)
    }

    /// Returns `true` if any item in `inputs` has been pressed during the current frame.
    pub fn any_just_pressed(&self, inputs: impl IntoIterator<Item = T>) -> bool {
        inputs.into_iter().any(|it| self.just_pressed(it))
    }

    /// Clears the `just_pressed` state of the `input` and returns `true` if the `input` has just been pressed.
    ///
    /// Future calls to [`ButtonInput::just_pressed`] for the given input will return false until a new press event occurs.
    pub fn clear_just_pressed(&mut self, input: T) -> bool {
        self.just_pressed.remove_entry(&input).is_some()
    }

    /// Returns `true` if the `input` has been released during the current frame.
    ///
    /// Note: This function does not imply information regarding the current state of [`ButtonInput::pressed`] or [`ButtonInput::just_pressed`].
    pub fn just_released(&self, input: T) -> bool {
        self.just_released.contains_key(&input)
    }

    /// Returns `true` if any item in `inputs` has just been released.
    pub fn any_just_released(&self, inputs: impl IntoIterator<Item = T>) -> bool {
        inputs.into_iter().any(|it| self.just_released(it))
    }

    /// Clears the `just_released` state of the `input` and returns `true` if the `input` has just been released.
    ///
    /// Future calls to [`ButtonInput::just_released`] for the given input will return false until a new release event occurs.
    pub fn clear_just_released(&mut self, input: T) -> bool {
        self.just_released.remove_entry(&input).is_some()
    }

    /// Clears the `pressed`, `just_pressed` and `just_released` data of the `input`.
    pub fn reset(&mut self, input: T) {
        self.pressed.remove(&input);
        self.just_pressed.remove(&input);
        self.just_released.remove(&input);
    }

    /// Clears the `pressed`, `just_pressed`, and `just_released` data for every input.
    ///
    /// See also [`ButtonInput::clear`] for simulating elapsed time steps.
    pub fn reset_all(&mut self) {
        self.pressed.clear();
        self.just_pressed.clear();
        self.just_released.clear();
    }

    /// Clears the `just pressed` and `just released` data for every input.
    ///
    /// See also [`ButtonInput::reset_all`] for a full reset.
    pub fn clear(&mut self) {
        self.just_pressed.clear();
        self.just_released.clear();
    }

    /// An iterator visiting every pressed input in arbitrary order.
    pub fn get_pressed(&self) -> impl ExactSizeIterator<Item = (&T, &U)> {
        self.pressed.iter()
    }

    /// An iterator visiting every just pressed input in arbitrary order.
    ///
    /// Note: Returned elements do not imply information regarding the current state of [`ButtonInput::pressed`] or [`ButtonInput::just_released`].
    pub fn get_just_pressed(&self) -> impl ExactSizeIterator<Item = (&T, &U)> {
        self.just_pressed.iter()
    }

    /// An iterator visiting every just released input in arbitrary order.
    ///
    /// Note: Returned elements do not imply information regarding the current state of [`ButtonInput::pressed`] or [`ButtonInput::just_pressed`].
    pub fn get_just_released(&self) -> impl ExactSizeIterator<Item = (&T, &U)> {
        self.just_released.iter()
    }
}
