use std::mem::transmute;

use crate::bindgen::{
    event_queue::{from_js_event_queue::FromJsEventQueue, to_js_event_queue::ToJsEventQueue},
    systems::{forward_events_to_js, poll_events_from_js},
    utils::set_panic_hook,
};
use crate::core::{
    canvas::systems::{
        construct_path::construct_rectangle_path, startup_system_log, update_system_log,
    },
    node::bundles::RectangleNodeBundle,
};
use crate::plugins::bindgen_render_plugin::BindgenRenderPlugin;
use crate::plugins::render_plugin::RenderApp;
use crate::plugins::render_plugin::RenderPlugin;
use bevy_app::{App, Last, PostUpdate, PreUpdate, Startup, Update};
use bevy_ecs::component::Component;
use glam::Vec2;
use serde::Serialize;
#[cfg(feature = "cli")]
use specta::Type;
use wasm_bindgen::prelude::*;

use self::events::{CursorEnteredCanvas, CursorExitedCanvas, CursorMovedOnCanvas};

pub mod events;
mod systems;

#[cfg_attr(feature = "cli", derive(Type))]
#[derive(Serialize)]
pub struct WorldIds {
    main_world_id: usize,
    render_world_id: usize,
}

// =============================================================================
// Canvas App
// =============================================================================

#[wasm_bindgen]
pub struct CanvasApp {
    world_ids: WorldIds,
    app: App,
}

#[wasm_bindgen]
impl CanvasApp {
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

        // Register events
        app.add_event::<CursorEnteredCanvas>();
        app.add_event::<CursorExitedCanvas>();
        app.add_event::<CursorMovedOnCanvas>();

        // Create canvas entity (only one should exist).
        // Why entity? Because I see it as part of the "game" world
        // and it changes frequently as it keeps track of the interaction mode.
        app.world.spawn(Canvas::default());

        Self {
            world_ids: WorldIds {
                main_world_id: CanvasApp::extract_main_world_id(&mut app),
                render_world_id: CanvasApp::extract_render_world_id(&mut app),
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
        self.app.world.spawn(RectangleNodeBundle::default());
    }
}

// =============================================================================
// Canvas Entity
// =============================================================================

#[derive(Component, Debug, Default)]
pub struct Canvas {
    width: f32,
    height: f32,
    interaction_mode: InteractionMode,
}

#[derive(Debug)]
pub enum InteractionMode {
    None,
    SelectionNet { origin: Vec2, current: Vec2 },
    Translating { origin: Vec2, current: Vec2 },
    Pressing { origin: Vec2 },
    Resizing { inital_bounds: XYWH, corner: Corner },
}

impl Default for InteractionMode {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Default)]
pub struct XYWH {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

#[derive(Debug)]
pub enum Corner {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl Default for Corner {
    fn default() -> Self {
        Self::TopLeft
    }
}
