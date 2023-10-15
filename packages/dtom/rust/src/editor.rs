use bevy_app::App;
use bevy_app::Startup;
use bevy_app::Update;
use wasm_bindgen::prelude::*;

use crate::svg_render_plugin::*;

use crate::js_bindings;
use crate::utils::set_panic_hook;

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

        app.add_plugins(SVGRenderPlugin)
            .add_systems(Update, update_system)
            .add_systems(Startup, startup_system);

        Self { app }
    }

    pub fn run(&mut self) {
        js_bindings::log("Run Editor");
        self.app.run();
    }

    pub fn update(&mut self) {
        js_bindings::log("Update Editor");
        self.app.update();
    }
}

fn update_system() {
    js_bindings::log("Inside update_system");
}

fn startup_system() {
    js_bindings::log("Inside startup_system");
}
