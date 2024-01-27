use std::sync::mpsc::Sender;

use bevy_app::{App, Plugin};

use dyn_bevy_render_skeleton::RenderApp;
use events::output_event::SVGRenderOutputEvent;
use resources::{changed_entities::ChangedEntitiesRes, svg_composition::SVGCompositionRes};

mod element_change;
mod events;
mod mixin_change;
mod resources;
mod systems;

pub struct SVGRenderPlugin {
    pub output_event_sender: Option<Sender<SVGRenderOutputEvent>>,
}

impl Plugin for SVGRenderPlugin {
    fn build(&self, app: &mut App) {
        let render_app = match app.get_sub_app_mut(RenderApp) {
            Ok(render_app) => render_app,
            Err(_) => return,
        };

        // Register resources
        render_app.init_resource::<ChangedEntitiesRes>();
        render_app.insert_resource(SVGCompositionRes::new(self.output_event_sender.clone()));
    }
}
