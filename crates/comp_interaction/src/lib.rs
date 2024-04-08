pub mod components;
pub mod events;
pub mod input;
pub mod resources;
mod systems;
mod utils;

use bevy_app::{App, Plugin, PreUpdate};
use bevy_ecs::schedule::{IntoSystemConfigs, IntoSystemSetConfigs, SystemSet};
use events::{
    CursorDownOnCompInputEvent, CursorDownOnEntityInputEvent, CursorDownOnResizeHandleInputEvent,
    CursorDownOnRotateHandleInputEvent, CursorEnteredCompInputEvent, CursorExitedCompInputEvent,
    CursorMovedOnCompInputEvent, CursorUpOnCompInputEvent, CursorUpOnEntityInputEvent,
    InteractionToolChangedInputEvent, KeyDownOnCompInputEvent, KeyUpOnCompInputEvent,
    MouseWheeledOnCompInputEvent,
};
use input::{
    button_input::ButtonInput,
    keyboard::KeyCode,
    mouse::{MouseButton, MouseButtonOnEntity, MouseButtonValue},
};
use resources::comp_interaction::CompInteractionRes;
use systems::{
    composition::{
        cursor_down::{cursor_down_on_comp_input_system, cursor_down_on_comp_system},
        cursor_entered::cursor_entered_comp_input_system,
        cursor_exited::cursor_exited_comp_input_system,
        cursor_move::cursor_moved_on_comp_input_system,
        cursor_up::{cursor_up_on_comp_input_system, cursor_up_on_comp_system},
        key_down::key_down_input_system,
        key_up::key_up_input_system,
        wheel::mouse_wheeled_on_comp_input_system,
    },
    entity::{
        cursor_down::{cursor_down_on_entity_input_system, cursor_down_on_entity_system},
        cursor_up::cursor_up_on_entity_input_system,
    },
    ui::{
        interaction_tool::interaction_tool_changed_input_system,
        resize_handle::cursor_down_on_resize_handle_input_system,
        rotate_handle::cursor_down_on_rotate_handle_input_system,
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
        app.add_event::<MouseWheeledOnCompInputEvent>();
        app.add_event::<CursorDownOnEntityInputEvent>();
        app.add_event::<CursorUpOnEntityInputEvent>();
        app.add_event::<CursorDownOnResizeHandleInputEvent>();
        app.add_event::<CursorDownOnRotateHandleInputEvent>();
        app.add_event::<InteractionToolChangedInputEvent>();

        // Register resources
        app.init_resource::<CompInteractionRes>();
        app.init_resource::<ButtonInput<KeyCode, ()>>();
        app.init_resource::<ButtonInput<MouseButton, MouseButtonValue>>();
        app.init_resource::<ButtonInput<MouseButtonOnEntity, MouseButtonValue>>();

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
                cursor_entered_comp_input_system.in_set(CompInteractionSystemSet::First),
                key_down_input_system.in_set(CompInteractionSystemSet::First),
                key_up_input_system
                    .in_set(CompInteractionSystemSet::First)
                    .after(key_down_input_system),
                cursor_down_on_comp_input_system.in_set(CompInteractionSystemSet::First),
                cursor_up_on_comp_input_system
                    .in_set(CompInteractionSystemSet::First)
                    .after(cursor_down_on_comp_input_system),
                cursor_down_on_entity_input_system.in_set(CompInteractionSystemSet::First),
                cursor_up_on_entity_input_system
                    .in_set(CompInteractionSystemSet::First)
                    .after(cursor_down_on_entity_input_system),
                interaction_tool_changed_input_system.in_set(CompInteractionSystemSet::First),
                cursor_down_on_comp_system.in_set(CompInteractionSystemSet::Activation),
                cursor_down_on_entity_system
                    .in_set(CompInteractionSystemSet::Activation)
                    .after(cursor_down_on_comp_system),
                cursor_down_on_resize_handle_input_system
                    .in_set(CompInteractionSystemSet::Manipulation),
                cursor_down_on_rotate_handle_input_system
                    .in_set(CompInteractionSystemSet::Manipulation),
                cursor_moved_on_comp_input_system.in_set(CompInteractionSystemSet::Continuous),
                mouse_wheeled_on_comp_input_system.in_set(CompInteractionSystemSet::Continuous),
                cursor_up_on_comp_system.in_set(CompInteractionSystemSet::Continuous),
                cursor_exited_comp_input_system.in_set(CompInteractionSystemSet::Last),
            ),
        );
    }
}
