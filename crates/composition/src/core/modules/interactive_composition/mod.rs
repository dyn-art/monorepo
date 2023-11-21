use bevy_app::{Plugin, Update};

use self::{
    components::InteractiveCompositionMixin,
    events::{
        CursorDownOnEntity, CursorEnteredComposition, CursorExitedComposition,
        CursorMovedOnComposition,
    },
    systems::handle_cursor_down_on_entity_event,
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
        app.add_systems(Update, handle_cursor_down_on_entity_event);

        // Spawn interactive composition entity
        app.world.spawn(InteractiveCompositionMixin::default());
    }
}
