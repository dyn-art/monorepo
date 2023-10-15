use bevy_app::{App, Plugin};

use crate::bindgen::js_bindings;

use super::render_plugin::{ExtractSchedule, RenderApp, RenderSchedule};

// =============================================================================
// Systems
// =============================================================================

fn render_system() {
    js_bindings::log("Inside render_system - bindgen");
}

fn extract_system() {
    js_bindings::log("Inside extract_system - bindgen");
}

// =============================================================================
// Plugin
// =============================================================================

pub struct BindgenRenderPlugin;

impl Plugin for BindgenRenderPlugin {
    fn build(&self, app: &mut App) {
        let render_app = match app.get_sub_app_mut(RenderApp) {
            Ok(render_app) => render_app,
            Err(_) => return,
        };

        render_app
            .add_systems(ExtractSchedule, extract_system)
            .add_systems(RenderSchedule, render_system);
    }
}
