pub mod components;
pub mod events;
pub mod input;
pub mod resources;
mod systems;
mod utils;

use bevy_app::{App, Plugin, PreUpdate};
use bevy_ecs::schedule::{IntoSystemConfigs, IntoSystemSetConfigs, SystemSet};
use dyn_arb_bundles::events::InputEvent;
use events::InteractionInputEvent;
use input::{
    keyboard::KeyCodeButtonInput,
    mouse::{
        MouseButtonButtonInputRes, MouseButtonOnEntityButtonInputRes,
        MouseButtonOnResizeHandleButtonInputRes, MouseButtonOnRotateHandleButtonInputRes,
    },
};
use resources::arb_interaction::ArbInteractionRes;
use systems::{
    artboard::{
        cursor_down::{cursor_down_on_arb_input_system, cursor_down_on_arb_system},
        cursor_entered::cursor_entered_arb_input_system,
        cursor_exited::cursor_exited_arb_input_system,
        cursor_move::cursor_moved_on_arb_input_system,
        cursor_up::{cursor_up_on_arb_input_system, cursor_up_on_arb_system},
        key_down::{key_down_input_system, remove_selected_entity_system},
        key_up::key_up_input_system,
        mouse_wheel::mouse_wheeled_on_arb_input_system,
    },
    entity::cursor_down::{cursor_down_on_entity_input_system, cursor_down_on_entity_system},
    ui::{
        interaction_tool::interaction_tool_changed_input_system,
        resize_handle::{
            cursor_down_on_resize_handle_input_system, cursor_down_on_resize_handle_system,
        },
        rotate_handle::{
            cursor_down_on_rotate_handle_input_system, cursor_down_on_rotate_handle_system,
        },
    },
};

pub struct ArbInteractionPlugin;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum ArbInteractionSystemSet {
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

impl Plugin for ArbInteractionPlugin {
    fn build(&self, app: &mut App) {
        // Register events
        InteractionInputEvent::register_events(app);

        // Register resources
        app.init_resource::<ArbInteractionRes>();
        app.init_resource::<KeyCodeButtonInput>();
        app.init_resource::<MouseButtonButtonInputRes>();
        app.init_resource::<MouseButtonOnEntityButtonInputRes>();
        app.init_resource::<MouseButtonOnResizeHandleButtonInputRes>();
        app.init_resource::<MouseButtonOnRotateHandleButtonInputRes>();

        // Configure system sets
        app.configure_sets(
            PreUpdate,
            (
                ArbInteractionSystemSet::First,
                ArbInteractionSystemSet::Activation,
                ArbInteractionSystemSet::Manipulation,
                ArbInteractionSystemSet::Continuous,
                ArbInteractionSystemSet::Intermediate,
                ArbInteractionSystemSet::Last,
            )
                .chain(),
        );

        // Register systems
        app.add_systems(
            PreUpdate,
            (
                cursor_entered_arb_input_system.in_set(ArbInteractionSystemSet::First),
                key_down_input_system.in_set(ArbInteractionSystemSet::First),
                key_up_input_system
                    .in_set(ArbInteractionSystemSet::First)
                    .after(key_down_input_system),
                cursor_down_on_arb_input_system.in_set(ArbInteractionSystemSet::First),
                cursor_down_on_entity_input_system.in_set(ArbInteractionSystemSet::First),
                cursor_down_on_resize_handle_input_system.in_set(ArbInteractionSystemSet::First),
                cursor_down_on_rotate_handle_input_system.in_set(ArbInteractionSystemSet::First),
                cursor_up_on_arb_input_system
                    .in_set(ArbInteractionSystemSet::First)
                    .after(cursor_down_on_arb_input_system)
                    .after(cursor_down_on_entity_input_system)
                    .after(cursor_down_on_resize_handle_input_system)
                    .after(cursor_down_on_rotate_handle_input_system),
                interaction_tool_changed_input_system.in_set(ArbInteractionSystemSet::First),
                cursor_down_on_arb_system.in_set(ArbInteractionSystemSet::Activation),
                cursor_down_on_entity_system
                    .in_set(ArbInteractionSystemSet::Activation)
                    .after(cursor_down_on_arb_system),
                cursor_down_on_resize_handle_system.in_set(ArbInteractionSystemSet::Manipulation),
                cursor_down_on_rotate_handle_system.in_set(ArbInteractionSystemSet::Manipulation),
                remove_selected_entity_system.in_set(ArbInteractionSystemSet::Manipulation),
                cursor_moved_on_arb_input_system.in_set(ArbInteractionSystemSet::Continuous),
                mouse_wheeled_on_arb_input_system.in_set(ArbInteractionSystemSet::Continuous),
                cursor_up_on_arb_system.in_set(ArbInteractionSystemSet::Last),
                cursor_exited_arb_input_system.in_set(ArbInteractionSystemSet::Last),
            ),
        );
    }
}
