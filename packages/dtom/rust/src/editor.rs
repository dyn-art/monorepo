use bevy_app::App;
use bevy_app::Startup;
use bevy_app::Update;
use wasm_bindgen::prelude::*;

use crate::bindgen::{js_bindings, utils::set_panic_hook};
use crate::js_event_queue::JsEventQueue;
use crate::plugins::bindgen_render_plugin::bundles::RectangleBundle;
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
        app.add_systems(Update, update_system)
            .add_systems(Startup, startup_system);

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

fn update_system() {
    js_bindings::log("Inside update_system");
}

fn startup_system() {
    js_bindings::log("Inside startup_system");
}
