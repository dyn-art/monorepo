use bevy_app::{App, Last, Plugin};
use bevy_ecs::schedule::{IntoSystemConfigs, SystemSet};
use bevy_render::{ExtractSchedule, RenderApp};
use events::SVGRenderOutputEvent;
use resources::{
    changed_svg_nodes::ChangedSVGNodesRes, svg_context::SVGContextRes,
    svg_render_output_event_sender::SVGRenderOutputEventSenderRes,
};
use systems::{
    extract::extract_svg_nodes_generic,
    svg_node::frame::{apply_frame_node_transform_change, create_frame_svg_node, FrameSVGNode},
};

pub mod events;
pub mod resources;
pub mod svg;
pub mod systems;

pub struct SVGRenderPlugin {
    #[cfg(feature = "output_events")]
    pub output_event_sender: std::sync::mpsc::Sender<SVGRenderOutputEvent>,
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum SvgRenderSystem {
    /// After this lablel,  events got applied.
    Create,
    /// After this label, the layout got applied to the compositions nodes.
    Apply,
}

impl Plugin for SVGRenderPlugin {
    fn build(&self, app: &mut App) {
        // Register systems
        app.add_systems(
            Last,
            (
                create_frame_svg_node.in_set(SvgRenderSystem::Create),
                apply_frame_node_transform_change
                    .in_set(SvgRenderSystem::Apply)
                    .after(SvgRenderSystem::Create),
            ),
        );

        // Register resources
        app.init_resource::<SVGContextRes>();

        #[cfg(feature = "output_events")]
        build_render_app(app, self.output_event_sender.clone());
    }
}

fn build_render_app(app: &mut App, sender: std::sync::mpsc::Sender<SVGRenderOutputEvent>) {
    let render_app = match app.get_sub_app_mut(RenderApp) {
        Ok(render_app) => render_app,
        Err(_) => return,
    };

    // Register resources
    render_app.init_resource::<ChangedSVGNodesRes>();
    render_app.insert_resource(SVGRenderOutputEventSenderRes { sender });

    // Register systems
    render_app.add_systems(ExtractSchedule, (extract_svg_nodes_generic::<FrameSVGNode>));
}
