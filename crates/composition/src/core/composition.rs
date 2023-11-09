use bevy_app::{App, Plugins};
use bevy_ecs::{bundle::Bundle, entity::Entity};
use dyn_bevy_render_skeleton::RenderPlugin;

use crate::core::modules::{
    composition::CompositionPlugin, interactive_composition::InteractiveCompositionPlugin,
};

use super::{dtif::DTIFComposition, events::input_event::InputEvent, modules::node::NodePlugin};

pub struct Composition {
    app: App,
}

impl Composition {
    pub fn new(dtif: Option<DTIFComposition>) -> Self {
        let mut app = App::new();

        // Register plugins
        app.add_plugins((
            RenderPlugin,
            CompositionPlugin { dtif },
            InteractiveCompositionPlugin, // TODO: only include if "interaction" feature active?
            NodePlugin,
        ));

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

    pub fn spawn<B: Bundle + std::fmt::Debug>(&mut self, bundle: B) -> Entity {
        return self.app.world.spawn::<B>(bundle).id();
    }

    pub fn register_events<T: InputEvent>(&mut self, events: Vec<T>) {
        for event in events {
            self.register_event(event)
        }
    }

    pub fn register_event<T: InputEvent>(&mut self, event: T) {
        event.send_to_ecs(&mut self.app.world);
    }

    pub fn clear(&mut self) {
        self.app.world.clear_all();
    }
}
