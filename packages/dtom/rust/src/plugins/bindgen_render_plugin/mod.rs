pub mod bundles;
pub mod shapes;

use bevy_app::{App, Plugin};
use bevy_ecs::{
    query::With,
    system::{Commands, Query},
};

use crate::bindgen::js_bindings;

use self::shapes::{Fill, Shape, Transform};

use super::render_plugin::{extract_param::Extract, ExtractSchedule, RenderApp, RenderSchedule};

// =============================================================================
// Systems
// =============================================================================

fn render_system() {
    js_bindings::log("Inside render_system - bindgen");
}

fn extract_shapes(mut commands: Commands, query: Extract<Query<(&Transform), With<Shape>>>) {
    js_bindings::log("Inside extract_shapes");
    query.for_each(|(transform)| {
        js_bindings::log(&format!("transform: {:?}", transform));
    });
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
            .add_systems(ExtractSchedule, extract_shapes)
            .add_systems(RenderSchedule, render_system);
    }
}
