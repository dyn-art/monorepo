use std::sync::mpsc::Sender;

use bevy_app::{App, Plugin};

use bevy_ecs::schedule::IntoSystemConfigs;
use dyn_bevy_render_skeleton::{ExtractSchedule, Render, RenderApp, RenderSet};
use dyn_composition::core::modules::node::components::{
    mixins::{
        BlendMixin, DimensionMixin, NodeCompositionMixin, PaintCompositionMixin, PathMixin,
        RelativeTransformMixin,
    },
    types::SolidPaint,
};
use events::output_event::SVGRenderOutputEvent;
use resources::{changed_entities::ChangedEntitiesRes, svg_composition::SVGCompositionRes};
use systems::{
    extract::{extract_children, extract_node_mixin_generic, extract_paint_mixin_generic},
    queue::queue_element_changes,
};

pub mod element_change;
pub mod events;
pub mod mixin_change;
pub mod resources;
pub mod systems;

pub struct SVGRenderPlugin {
    #[cfg(feature = "output-event")]
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
        #[cfg(feature = "output-event")]
        render_app.insert_resource(SVGCompositionRes::new(self.output_event_sender.clone()));
        #[cfg(not(feature = "output-event"))]
        render_app.insert_resource(SVGCompositionRes::new());

        // Register systems
        render_app
            .add_systems(
                ExtractSchedule,
                (
                    extract_children,
                    // Node
                    extract_node_mixin_generic::<DimensionMixin>,
                    extract_node_mixin_generic::<RelativeTransformMixin>,
                    extract_node_mixin_generic::<NodeCompositionMixin>,
                    extract_node_mixin_generic::<BlendMixin>,
                    extract_node_mixin_generic::<PathMixin>,
                    // Paint
                    extract_paint_mixin_generic::<DimensionMixin>,
                    extract_paint_mixin_generic::<BlendMixin>,
                    extract_paint_mixin_generic::<PaintCompositionMixin>,
                    extract_paint_mixin_generic::<SolidPaint>,
                ),
            )
            .add_systems(Render, queue_element_changes.in_set(RenderSet::Queue));
    }
}
