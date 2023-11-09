use bevy_app::Plugin;

use self::{
    components::InteractiveCompositionMixin,
    events::{
        CursorDownOnEntity, CursorEnteredComposition, CursorExitedComposition,
        CursorMovedOnComposition,
    },
};

pub mod components;
pub mod events;

pub struct InteractiveCompositionPlugin;

impl Plugin for InteractiveCompositionPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        // Register events
        app.add_event::<CursorMovedOnComposition>();
        app.add_event::<CursorEnteredComposition>();
        app.add_event::<CursorExitedComposition>();
        app.add_event::<CursorDownOnEntity>();

        // Spawn interactive composition entity
        app.world.spawn(InteractiveCompositionMixin::default());
    }
}
