use bevy_app::{App, Plugins};
use bevy_ecs::{bundle::Bundle, entity::Entity};
use dyn_bevy_render_skeleton::RenderPlugin;

use super::{
    dtif::DTIFComposition,
    modules::{
        composition::{events::input_event::InputEvent, CompositionPlugin},
        node::NodePlugin,
    },
};

pub struct Composition {
    app: App,
}

impl Composition {
    pub fn new(dtif: Option<DTIFComposition>) -> Self {
        let mut app = App::new();

        // Register plugins
        app.add_plugins((RenderPlugin, CompositionPlugin { dtif }, NodePlugin));

        // Register resources
        // TODO

        // Register systems
        // TODO

        // Register events
        // TODO

        return Self { app };
    }

    pub fn add_plugins<M>(&mut self, plugins: impl Plugins<M>) {
        self.app.add_plugins(plugins);
    }

    pub fn update(&mut self) {
        self.app.update();
    }

    pub fn spawn<B: Bundle>(&mut self, bundle: B) -> Entity {
        return self.app.world.spawn::<B>(bundle).id();
    }

    pub fn register_events(&mut self, events: Vec<InputEvent>) {
        for event in events {
            self.register_event(event);
        }
    }

    pub fn register_event(&mut self, event: InputEvent) {
        match event {
            // Cursor Events
            InputEvent::CursorMovedOnComposition(event) => {
                self.app.world.send_event(event);
            }
            InputEvent::CursorEnteredComposition(event) => {
                self.app.world.send_event(event);
            }
            InputEvent::CursorExitedComposition(event) => {
                self.app.world.send_event(event);
            }
            InputEvent::CursorDownOnEntity(event) => {
                self.app.world.send_event(event);
            }

            // Entity Events
            InputEvent::EntityMoved(event) => {
                self.app.world.send_event(event);
            }
            InputEvent::EntitySetPosition(event) => {
                self.app.world.send_event(event);
            }
        }
    }
}
