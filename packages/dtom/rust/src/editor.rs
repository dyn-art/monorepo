use std::mem::transmute;

use bevy_app::App;
use bevy_app::Last;
use bevy_app::Startup;
use bevy_app::Update;
use bevy_ecs::system::ResMut;
use wasm_bindgen::prelude::*;

use crate::bindgen::{js_bindings, utils::set_panic_hook};
use crate::js_event_queue::JsEventQueue;
use crate::node::bundles::RectangleNodeBundle;
use crate::plugins::bindgen_render_plugin::BindgenRenderPlugin;
use crate::plugins::render_plugin::RenderApp;
use crate::plugins::render_plugin::RenderPlugin;

#[wasm_bindgen]
pub struct Editor {
    world_ids: Vec<usize>,
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

        Self {
            world_ids: Editor::extract_world_ids(&mut app),
            app,
        }
    }

    /// Extracts the world id of the main world and the render world
    fn extract_world_ids(app: &mut App) -> Vec<usize> {
        let mut world_ids: Vec<usize> = Vec::new();

        let main_world_id = app.world.id();
        let parsed_main_world_id: usize = unsafe { transmute(main_world_id) };
        world_ids.push(parsed_main_world_id);

        let render_app = app.get_sub_app_mut(RenderApp).unwrap();
        let render_world_id = render_app.world.id();
        let parsed_render_world_id: usize = unsafe { transmute(render_world_id) };
        world_ids.push(parsed_render_world_id);

        return world_ids;
    }

    pub fn update(&mut self) {
        self.app.update();
    }

    pub fn get_world_ids(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.world_ids).unwrap()
    }

    pub fn create_rect(&mut self) {
        js_bindings::log("Creating rect");
        self.app.world.spawn(RectangleNodeBundle::default());
    }
}

fn update_system_log() {
    js_bindings::log("---- Inside update_system");
}

fn startup_system_log() {
    js_bindings::log("Inside startup_system");
}

fn forward_events_to_js(mut event_queue: ResMut<JsEventQueue>) {
    event_queue.forward_events_to_js();
}
