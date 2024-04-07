pub mod components;
pub mod events;
pub mod resources;
mod systems;
mod utils;

use bevy_app::{App, Plugin, PreUpdate};
use bevy_ecs::schedule::{IntoSystemConfigs, IntoSystemSetConfigs, SystemSet};
use bevy_input::{keyboard::KeyCode, mouse::MouseButton, ButtonInput};
use events::{
    CursorDownOnCompInputEvent, CursorDownOnEntityInputEvent, CursorDownOnResizeHandleInputEvent,
    CursorDownOnRotateHandleInputEvent, CursorEnteredCompInputEvent, CursorExitedCompInputEvent,
    CursorMovedOnCompInputEvent, CursorUpOnCompInputEvent, InteractionToolChangedInputEvent,
    KeyDownOnCompInputEvent, KeyUpOnCompInputEvent, WheelActionOnCompInputEvent,
};
use resources::comp_interaction::CompInteractionRes;
use systems::{
    composition::{
        cursor_down::handle_cursor_down_on_comp_event,
        cursor_entered::handle_cursor_entered_comp_event,
        cursor_exited::handle_cursor_exited_comp_event,
        cursor_move::handle_cursor_moved_on_comp_event, cursor_up::handle_cursor_up_on_comp_event,
        key_down::handle_key_down_event, key_up::handle_key_up_event,
        wheel::handle_wheel_action_on_comp_event,
    },
    entity::cursor_down::handle_cursor_down_on_entity_event,
    ui::{
        interaction_tool::handle_interaction_tool_change_event,
        resize_handle::handle_cursor_down_on_resize_handle_event,
        rotate_handle::handle_cursor_down_on_rotate_handle_event,
    },
};

pub struct CompInteractionPlugin;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum CompInteractionSystemSet {
    /// Sets initial conditions for interaction, preparing the system for user input processing.
    First,

    /// Processes user-initiated actions, readying the system for direct manipulation.
    Activation,

    /// Handles direct manipulation such as dragging or resizing, ensuring responsive feedback.
    Manipulation,

    /// Manages continuous actions (e.g. cursor movement, scrolling), providing immediate feedback.
    Continuous,

    /// Addresses multi-step interactions, preparing for their conclusion.
    Intermediate,

    /// Marks the completion of interaction processing, readying the system for new input.
    Last,
}

impl Plugin for CompInteractionPlugin {
    fn build(&self, app: &mut App) {
        // Register events
        app.add_event::<KeyDownOnCompInputEvent>();
        app.add_event::<KeyUpOnCompInputEvent>();
        app.add_event::<CursorEnteredCompInputEvent>();
        app.add_event::<CursorExitedCompInputEvent>();
        app.add_event::<CursorMovedOnCompInputEvent>();
        app.add_event::<CursorDownOnCompInputEvent>();
        app.add_event::<CursorUpOnCompInputEvent>();
        app.add_event::<WheelActionOnCompInputEvent>();
        app.add_event::<CursorDownOnEntityInputEvent>();
        app.add_event::<CursorDownOnResizeHandleInputEvent>();
        app.add_event::<CursorDownOnRotateHandleInputEvent>();
        app.add_event::<InteractionToolChangedInputEvent>();

        // Register resources
        app.init_resource::<CompInteractionRes>();
        app.init_resource::<ButtonInput<KeyCode>>();
        app.init_resource::<ButtonInput<MouseButton>>();

        // Configure system sets
        app.configure_sets(
            PreUpdate,
            (
                CompInteractionSystemSet::First,
                CompInteractionSystemSet::Activation,
                CompInteractionSystemSet::Manipulation,
                CompInteractionSystemSet::Continuous,
                CompInteractionSystemSet::Intermediate,
                CompInteractionSystemSet::Last,
            )
                .chain(),
        );

        // Register systems
        app.add_systems(
            PreUpdate,
            (
                handle_cursor_entered_comp_event.in_set(CompInteractionSystemSet::First),
                handle_key_down_event.in_set(CompInteractionSystemSet::First),
                handle_key_up_event.in_set(CompInteractionSystemSet::First),
                handle_interaction_tool_change_event.in_set(CompInteractionSystemSet::First),
                handle_cursor_down_on_comp_event.in_set(CompInteractionSystemSet::Activation),
                handle_cursor_down_on_entity_event
                    .in_set(CompInteractionSystemSet::Activation)
                    .after(handle_cursor_down_on_comp_event),
                handle_cursor_down_on_resize_handle_event
                    .in_set(CompInteractionSystemSet::Manipulation),
                handle_cursor_down_on_rotate_handle_event
                    .in_set(CompInteractionSystemSet::Manipulation),
                handle_cursor_moved_on_comp_event.in_set(CompInteractionSystemSet::Continuous),
                handle_wheel_action_on_comp_event.in_set(CompInteractionSystemSet::Continuous),
                handle_cursor_up_on_comp_event.in_set(CompInteractionSystemSet::Continuous),
                handle_cursor_exited_comp_event.in_set(CompInteractionSystemSet::Last),
            ),
        );
    }
}
