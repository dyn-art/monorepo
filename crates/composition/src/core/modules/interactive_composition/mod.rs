use bevy_app::{Plugin, PreUpdate};
use bevy_ecs::schedule::IntoSystemConfigs;

use self::{
    events::{
        CursorDownOnComposition, CursorDownOnEntity, CursorEnteredComposition,
        CursorExitedComposition, CursorMovedOnComposition, CursorUpOnComposition,
    },
    resources::InteractiveCompositionRes,
    systems::{
        handle_cursor_down_on_composition, handle_cursor_down_on_entity_event,
        handle_cursor_entered_composition, handle_cursor_exited_composition,
        handle_cursor_moved_on_composition, handle_cursor_up_on_composition,
    },
};

pub mod events;
pub mod resources;
mod systems;

pub struct InteractiveCompositionPlugin;

impl Plugin for InteractiveCompositionPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        // Register events
        app.add_event::<CursorMovedOnComposition>();
        app.add_event::<CursorEnteredComposition>();
        app.add_event::<CursorExitedComposition>();
        app.add_event::<CursorDownOnEntity>();
        app.add_event::<CursorDownOnComposition>();
        app.add_event::<CursorUpOnComposition>();

        // Register resources
        app.world.init_resource::<InteractiveCompositionRes>();

        // Register systems
        app.add_systems(
            PreUpdate,
            (
                handle_cursor_entered_composition,
                handle_cursor_exited_composition,
                handle_cursor_moved_on_composition,
                handle_cursor_down_on_entity_event.after(handle_cursor_down_on_composition),
                handle_cursor_down_on_composition,
                handle_cursor_up_on_composition,
            ),
        );
    }
}
