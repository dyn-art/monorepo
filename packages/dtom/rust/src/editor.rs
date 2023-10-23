use std::mem::transmute;

use bevy_app::App;
use bevy_app::Last;
use bevy_app::PostUpdate;
use bevy_app::PreUpdate;
use bevy_app::Startup;
use bevy_app::Update;
use serde::Serialize;
use wasm_bindgen::prelude::*;

use crate::bindgen::event_queue::from_js_event_queue::FromJsEventQueue;
use crate::bindgen::event_queue::to_js_event_queue::ToJsEventQueue;
use crate::bindgen::{js_bindings, utils::set_panic_hook};
use crate::node::bundles::RectangleNodeBundle;
use crate::plugins::bindgen_render_plugin::BindgenRenderPlugin;
use crate::plugins::render_plugin::RenderApp;
use crate::plugins::render_plugin::RenderPlugin;
use crate::systems::construct_path::construct_rectangle_path;
use crate::systems::forward_events_to_js;
use crate::systems::poll_events_from_js;
use crate::systems::startup_system_log;
use crate::systems::update_system_log;
#[cfg(feature = "cli")]
use specta::Type;

#[cfg_attr(feature = "cli", derive(Type))]
#[derive(Serialize)]
pub struct WorldIds {
    main_world_id: usize,
    render_world_id: usize,
}

#[wasm_bindgen]
pub struct Editor {
    world_ids: WorldIds,
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
        app.init_resource::<ToJsEventQueue>();
        app.init_resource::<FromJsEventQueue>();

        // Register systems
        app.add_systems(Startup, startup_system_log)
            .add_systems(PreUpdate, poll_events_from_js)
            .add_systems(Update, update_system_log)
            .add_systems(PostUpdate, construct_rectangle_path)
            .add_systems(Last, forward_events_to_js);

        Self {
            world_ids: WorldIds {
                main_world_id: Editor::extract_main_world_id(&mut app),
                render_world_id: Editor::extract_render_world_id(&mut app),
            },
            app,
        }
    }

    fn extract_main_world_id(app: &mut App) -> usize {
        let main_world_id = app.world.id();
        let parsed_main_world_id: usize = unsafe { transmute(main_world_id) };
        return parsed_main_world_id;
    }

    fn extract_render_world_id(app: &mut App) -> usize {
        let render_app = app.get_sub_app_mut(RenderApp).unwrap();
        let render_world_id = render_app.world.id();
        let parsed_render_world_id: usize = unsafe { transmute(render_world_id) };
        return parsed_render_world_id;
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
