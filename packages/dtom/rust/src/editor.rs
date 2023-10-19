use bevy_app::App;
use bevy_app::Last;
use bevy_app::Startup;
use bevy_app::Update;
use bevy_ecs::system::ResMut;
use wasm_bindgen::prelude::*;

use crate::bindgen::{js_bindings, utils::set_panic_hook};
use crate::bundles::RectangleBundle;
use crate::js_event_queue::JsEventQueue;
use crate::plugins::bindgen_render_plugin::BindgenRenderPlugin;
use crate::plugins::render_plugin::RenderPlugin;

#[wasm_bindgen]
pub struct Editor {
    app: App,
}

#[wasm_bindgen]
impl Editor {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        set_panic_hook();

        let mut app = App::new();

        // Register plugins
        app.add_plugins((RenderPlugin, BindgenRenderPlugin));

        // Register resources
        app.init_resource::<JsEventQueue>();

        // Register systems
        app.add_systems(Update, update_system_log)
            .add_systems(Startup, startup_system_log)
            .add_systems(Last, forward_events_to_js);

        Self { app }
    }

    pub fn create_rect(&mut self) {
        js_bindings::log("Creating rect");
        self.app.world.spawn(RectangleBundle::default());
    }

    pub fn update(&mut self) {
        self.app.update();
    }
}

fn update_system_log() {
    js_bindings::log("Inside update_system");
}

fn startup_system_log() {
    js_bindings::log("----> Inside startup_system");
}

fn forward_events_to_js(mut event_queue: ResMut<JsEventQueue>) {
    event_queue.forward_events_to_js();
}
