use std::sync::mpsc::Sender;

use bevy_app::{App, Plugin};
use bevy_ecs::schedule::IntoSystemConfigs;
use dyn_bevy_render_skeleton::{ExtractSchedule, Render, RenderApp, RenderSet};
use dyn_composition::core::modules::node::components::mixins::{
    BlendMixin, DimensionMixin, NodeCompositionMixin, PathMixin, RelativeTransformMixin,
};
use events::output_event::RenderUpdateEvent;

use self::{
    resources::{changed_components::ChangedComponentsRes, svg_composition::SVGCompositionRes},
    systems::{
        extract::{extract_children, extract_mixin_generic, extract_paint},
        queue::queue_render_changes,
    },
};

pub mod events;
pub mod mixin_change;
pub mod render_change;
pub mod resources;
mod systems;

pub struct SvgRenderPlugin {
    pub render_event_sender: Option<Sender<RenderUpdateEvent>>,
}

impl Plugin for SvgRenderPlugin {
    fn build(&self, app: &mut App) {
        let render_app = match app.get_sub_app_mut(RenderApp) {
            Ok(render_app) => render_app,
            Err(_) => return,
        };

        // Register resources
        render_app.init_resource::<ChangedComponentsRes>();
        render_app.insert_resource(SVGCompositionRes::new(self.render_event_sender.clone()));

        // Register systems
        render_app
            .add_systems(
                ExtractSchedule,
                (
                    extract_mixin_generic::<DimensionMixin>,
                    extract_mixin_generic::<RelativeTransformMixin>,
                    extract_mixin_generic::<NodeCompositionMixin>,
                    extract_mixin_generic::<BlendMixin>,
                    extract_mixin_generic::<PathMixin>,
                    extract_children,
                    extract_paint,
                ),
            )
            .add_systems(Render, (queue_render_changes.in_set(RenderSet::Queue),));
    }
}
