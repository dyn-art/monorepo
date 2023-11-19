use std::sync::mpsc::Sender;

use bevy_app::{App, Plugin};
use bevy_ecs::schedule::IntoSystemConfigs;
use dyn_bevy_render_skeleton::{ExtractSchedule, Render, RenderApp, RenderSet};
use dyn_composition::core::modules::node::components::mixins::{
    BlendMixin, DimensionMixin, FillMixin, NodeCompositionMixin, PathMixin, RelativeTransformMixin,
};

use crate::core::events::output_event::OutputEvent;

use self::{
    resources::{changed_components::ChangedComponents, svg_composition::SVGComposition},
    systems::{
        extract::{extract_mixin_generic, extract_paint},
        queue_render_changes::queue_render_changes,
    },
};

pub mod resources;
mod systems;

pub struct SvgRenderPlugin {
    pub output_event_sender: Sender<OutputEvent>,
}

impl Plugin for SvgRenderPlugin {
    fn build(&self, app: &mut App) {
        let render_app = match app.get_sub_app_mut(RenderApp) {
            Ok(render_app) => render_app,
            Err(_) => return,
        };

        // Register resources
        render_app.init_resource::<ChangedComponents>();
        render_app.insert_resource(SVGComposition::new(self.output_event_sender.clone()));

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
                    extract_mixin_generic::<FillMixin>,
                    extract_paint,
                ),
            )
            .add_systems(Render, (queue_render_changes.in_set(RenderSet::Queue),));
    }
}
