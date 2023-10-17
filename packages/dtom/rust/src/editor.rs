use bevy_app::App;
use bevy_app::Startup;
use bevy_app::Update;
use js_sys::Function;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::bindgen::{js_bindings, utils::set_panic_hook};
use crate::plugins::bindgen_render_plugin::bundles::RectangleBundle;
use crate::plugins::bindgen_render_plugin::render_event_queue::RenderEvent;
use crate::plugins::bindgen_render_plugin::render_event_queue::RenderEventQueue;
use crate::plugins::bindgen_render_plugin::BindgenRenderPlugin;
use crate::plugins::render_plugin::RenderApp;
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

        js_bindings::log("Init Editor");

        // Register plugins
        app.add_plugins((RenderPlugin, BindgenRenderPlugin))
            .add_systems(Update, update_system)
            .add_systems(Startup, startup_system);

        Self { app }
    }

    pub fn create_rect(&mut self) {
        self.app.world.spawn(RectangleBundle::default());
    }

    pub fn update(&mut self) {
        js_bindings::log("Update Editor");
        self.app.update();
    }

    pub fn register_js_callback(&mut self, js_callback: Function) {
        // Access the sub app
        match self.app.get_sub_app_mut(RenderApp) {
            Ok(render_app) => {
                // Access the world of the sub app to get the RenderEventQueue resource
                if let Some(mut event_queue) =
                    render_app.world.get_resource_mut::<RenderEventQueue>()
                {
                    // Register the JS callback
                    event_queue.add_js_callback(js_callback);
                } else {
                    js_bindings::log("RenderEventQueue resource not found");
                }
            }
            Err(_) => {
                js_bindings::log("RenderApp sub app not found");
            }
        }
    }
}

fn update_system() {
    js_bindings::log("Inside update_system");
}

fn startup_system() {
    js_bindings::log("Inside startup_system");
}
