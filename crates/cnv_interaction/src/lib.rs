pub mod components;
pub mod events;
pub mod input;
pub mod resources;
mod systems;
mod utils;

use bevy_app::{App, Plugin, PreUpdate};
use bevy_ecs::schedule::{IntoSystemConfigs, IntoSystemSetConfigs, SystemSet};
use dyn_cnv_bundles::events::InputEvent;
use events::InteractionInputEvent;
use input::{
    keyboard::KeyCodeButtonInput,
    mouse::{
        MouseButtonButtonInputRes, MouseButtonOnEntityButtonInputRes,
        MouseButtonOnResizeHandleButtonInputRes, MouseButtonOnRotateHandleButtonInputRes,
    },
};
use resources::cnv_interaction::CnvInteractionRes;
use systems::{
    canvas::{
        cursor_down::{cursor_down_on_cnv_input_system, cursor_down_on_cnv_system},
        cursor_entered::cursor_entered_cnv_input_system,
        cursor_exited::cursor_exited_cnv_input_system,
        cursor_move::cursor_moved_on_cnv_input_system,
        cursor_up::{cursor_up_on_cnv_input_system, cursor_up_on_cnv_system},
        key_down::{key_down_input_system, remove_selected_entity_system},
        key_up::key_up_input_system,
        mouse_wheel::mouse_wheeled_on_cnv_input_system,
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

pub struct CnvInteractionPlugin;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum CnvInteractionSystemSet {
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

impl Plugin for CnvInteractionPlugin {
    fn build(&self, app: &mut App) {
        // Register events
        InteractionInputEvent::register_events(app);

        // Register resources
        app.init_resource::<CnvInteractionRes>();
        app.init_resource::<KeyCodeButtonInput>();
        app.init_resource::<MouseButtonButtonInputRes>();
        app.init_resource::<MouseButtonOnEntityButtonInputRes>();
        app.init_resource::<MouseButtonOnResizeHandleButtonInputRes>();
        app.init_resource::<MouseButtonOnRotateHandleButtonInputRes>();

        // Configure system sets
        app.configure_sets(
            PreUpdate,
            (
                CnvInteractionSystemSet::First,
                CnvInteractionSystemSet::Activation,
                CnvInteractionSystemSet::Manipulation,
                CnvInteractionSystemSet::Continuous,
                CnvInteractionSystemSet::Intermediate,
                CnvInteractionSystemSet::Last,
            )
                .chain(),
        );

        // Register systems
        app.add_systems(
            PreUpdate,
            (
                cursor_entered_cnv_input_system.in_set(CnvInteractionSystemSet::First),
                key_down_input_system.in_set(CnvInteractionSystemSet::First),
                key_up_input_system
                    .in_set(CnvInteractionSystemSet::First)
                    .after(key_down_input_system),
                cursor_down_on_cnv_input_system.in_set(CnvInteractionSystemSet::First),
                cursor_down_on_entity_input_system.in_set(CnvInteractionSystemSet::First),
                cursor_down_on_resize_handle_input_system.in_set(CnvInteractionSystemSet::First),
                cursor_down_on_rotate_handle_input_system.in_set(CnvInteractionSystemSet::First),
                cursor_up_on_cnv_input_system
                    .in_set(CnvInteractionSystemSet::First)
                    .after(cursor_down_on_cnv_input_system)
                    .after(cursor_down_on_entity_input_system)
                    .after(cursor_down_on_resize_handle_input_system)
                    .after(cursor_down_on_rotate_handle_input_system),
                interaction_tool_changed_input_system.in_set(CnvInteractionSystemSet::First),
                cursor_down_on_cnv_system.in_set(CnvInteractionSystemSet::Activation),
                cursor_down_on_entity_system
                    .in_set(CnvInteractionSystemSet::Activation)
                    .after(cursor_down_on_cnv_system),
                cursor_down_on_resize_handle_system.in_set(CnvInteractionSystemSet::Manipulation),
                cursor_down_on_rotate_handle_system.in_set(CnvInteractionSystemSet::Manipulation),
                remove_selected_entity_system.in_set(CnvInteractionSystemSet::Manipulation),
                cursor_moved_on_cnv_input_system.in_set(CnvInteractionSystemSet::Continuous),
                mouse_wheeled_on_cnv_input_system.in_set(CnvInteractionSystemSet::Continuous),
                cursor_up_on_cnv_system.in_set(CnvInteractionSystemSet::Last),
                cursor_exited_cnv_input_system.in_set(CnvInteractionSystemSet::Last),
            ),
        );
    }
}
