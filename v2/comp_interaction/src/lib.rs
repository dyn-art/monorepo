pub mod events;
pub mod resources;

use bevy_app::{App, Plugin, PreUpdate};
use bevy_ecs::schedule::{IntoSystemSetConfigs, SystemSet};
use events::{
    CursorDownOnCompInputEvent, CursorDownOnEntityInputEvent, CursorDownOnResizeHandleInputEvent,
    CursorDownOnRotateHandleInputEvent, CursorEnteredCompInputEvent, CursorExitedCompInputEvent,
    CursorMovedOnCompInputEvent, CursorUpOnCompInputEvent, WheeledOnCompInputEvent,
};
use resources::comp_interaction::CompInteractionRes;

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
    ContinuousFeedback,

    /// Addresses multi-step interactions, preparing for their conclusion.
    Intermediate,

    /// Finalizes interactions, stabilizing states before concluding the active sequence.
    Termination,

    /// Marks the completion of interaction processing, readying the system for new input.
    Last,
}

impl Plugin for CompInteractionPlugin {
    fn build(&self, app: &mut App) {
        // Register events
        app.add_event::<CursorMovedOnCompInputEvent>();
        app.add_event::<CursorEnteredCompInputEvent>();
        app.add_event::<CursorExitedCompInputEvent>();
        app.add_event::<CursorDownOnEntityInputEvent>();
        app.add_event::<CursorDownOnCompInputEvent>();
        app.add_event::<CursorUpOnCompInputEvent>();
        app.add_event::<WheeledOnCompInputEvent>();
        app.add_event::<CursorDownOnResizeHandleInputEvent>();
        app.add_event::<CursorDownOnRotateHandleInputEvent>();

        // Register resources
        app.world.init_resource::<CompInteractionRes>();

        // Configure system sets
        app.configure_sets(
            PreUpdate,
            (
                CompInteractionSystemSet::First,
                CompInteractionSystemSet::Activation,
                CompInteractionSystemSet::Manipulation,
                CompInteractionSystemSet::ContinuousFeedback,
                CompInteractionSystemSet::Intermediate,
                CompInteractionSystemSet::Termination,
                CompInteractionSystemSet::Last,
            )
                .chain(),
        );

        // Register systems
        // TODO
    }
}
