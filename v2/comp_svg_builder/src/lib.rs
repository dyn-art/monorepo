use bevy_app::{App, Last, Plugin};
use bevy_ecs::schedule::{IntoSystemConfigs, SystemSet};
use resources::svg_context::SVGContextRes;
use systems::svg_node::{
    frame::{apply_frame_node_size_change, insert_frame_svg_node},
    shape::{apply_shape_node_size_change, insert_shape_svg_node},
};

pub mod events;
pub mod resources;
pub mod svg;
pub mod systems;

pub struct SVGBuilderPlugin {
    #[cfg(feature = "output_events")]
    pub output_event_sender: std::sync::mpsc::Sender<crate::events::SVGBuilderOutputEvent>,
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum SVGRBuilderSystem {
    /// After this lablel,  events got applied.
    Insert,
    /// After this label, the layout got applied to the compositions nodes.
    Apply,
}

impl Plugin for SVGBuilderPlugin {
    fn build(&self, app: &mut App) {
        // Register systems
        app.add_systems(
            Last,
            (
                // Frame SVG Node
                insert_frame_svg_node.in_set(SVGRBuilderSystem::Insert),
                apply_frame_node_size_change
                    .in_set(SVGRBuilderSystem::Apply)
                    .after(SVGRBuilderSystem::Insert),
                // Shape SVG Node
                insert_shape_svg_node.in_set(SVGRBuilderSystem::Insert),
                apply_shape_node_size_change
                    .in_set(SVGRBuilderSystem::Apply)
                    .after(SVGRBuilderSystem::Insert),
            ),
        );

        // Register resources
        app.init_resource::<SVGContextRes>();

        #[cfg(feature = "output_events")]
        build_render_app(app, self.output_event_sender.clone());
    }
}

#[cfg(feature = "output_events")]
fn build_render_app(app: &mut App, sender: std::sync::mpsc::Sender<events::SVGBuilderOutputEvent>) {
    use crate::systems::extract::extract_svg_nodes_generic;
    use bevy_render::{ExtractSchedule, Render, RenderApp};
    use resources::{
        changed_svg_nodes::ChangedSVGNodesRes, output_event_sender::OutputEventSenderRes,
    };
    use systems::{
        queue::queue_svg_node_changes,
        svg_node::{frame::FrameSVGNode, shape::ShapeSVGNode},
    };

    let render_app = match app.get_sub_app_mut(RenderApp) {
        Ok(render_app) => render_app,
        Err(_) => return,
    };

    // Register resources
    render_app.init_resource::<ChangedSVGNodesRes>();
    render_app.insert_resource(OutputEventSenderRes { sender });

    // Register systems
    render_app.add_systems(
        ExtractSchedule,
        (
            extract_svg_nodes_generic::<FrameSVGNode>,
            extract_svg_nodes_generic::<ShapeSVGNode>,
        ),
    );
    render_app.add_systems(Render, queue_svg_node_changes);
}
