use bevy_app::{Plugin, PreUpdate};

use self::{
    components::InteractiveCompositionMixin,
    events::{
        CursorDownOnEntity, CursorEnteredComposition, CursorExitedComposition,
        CursorMovedOnComposition,
    },
    systems::{
        handle_cursor_down_on_entity_event, handle_cursor_entered_composition,
        handle_cursor_exited_composition, handle_cursor_moved_on_composition,
    },
};

pub mod components;
pub mod events;
mod systems;

pub struct InteractiveCompositionPlugin;

impl Plugin for InteractiveCompositionPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        // Register events
        app.add_event::<CursorMovedOnComposition>();
        app.add_event::<CursorEnteredComposition>();
        app.add_event::<CursorExitedComposition>();
        app.add_event::<CursorDownOnEntity>();

        // Register systems
        app.add_systems(
            PreUpdate,
            (
                handle_cursor_entered_composition,
                handle_cursor_exited_composition,
                handle_cursor_moved_on_composition,
                handle_cursor_down_on_entity_event,
            ),
        );

        // Spawn interactive composition entity
        app.world.spawn(InteractiveCompositionMixin::default());
    }
}
