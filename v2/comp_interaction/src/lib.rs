pub mod events;
pub mod resources;

use bevy_app::{App, Plugin, PreUpdate};
use bevy_ecs::schedule::{IntoSystemSetConfigs, SystemSet};
use events::{
    CursorDownOnComposition, CursorDownOnEntity, CursorDownOnResizeHandle,
    CursorDownOnRotateHandle, CursorEnteredComposition, CursorExitedComposition,
    CursorMovedOnComposition, CursorUpOnComposition, WheeledOnComposition,
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
        app.add_event::<CursorMovedOnComposition>();
        app.add_event::<CursorEnteredComposition>();
        app.add_event::<CursorExitedComposition>();
        app.add_event::<CursorDownOnEntity>();
        app.add_event::<CursorDownOnComposition>();
        app.add_event::<CursorUpOnComposition>();
        app.add_event::<WheeledOnComposition>();
        app.add_event::<CursorDownOnResizeHandle>();
        app.add_event::<CursorDownOnRotateHandle>();

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
