use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use resources::svg_context::SvgContextRes;
use systems::svg_node::{
    frame::{apply_frame_node_size_change, insert_frame_svg_node},
    shape::{apply_shape_node_size_change, insert_shape_svg_node},
};

pub mod events;
mod resources;
pub mod svg;
mod systems;

pub mod prelude {
    pub use super::events::*;
    pub use super::svg::*;
    pub use super::CompSvgBuilderPlugin;
}

pub struct CompSvgBuilderPlugin {
    #[cfg(feature = "output_events")]
    pub output_event_sender: std::sync::mpsc::Sender<crate::events::SvgBuilderOutputEvent>,
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum SvgBuilderSystemSet {
    /// After this lablel,  events got applied.
    Insert,
    /// After this label, the layout got applied to the compositions nodes.
    Apply,
}

impl Plugin for CompSvgBuilderPlugin {
    fn build(&self, app: &mut App) {
        // Register resources
        app.init_resource::<SvgContextRes>();

        // Configure system set
        app.configure_sets(
            Last,
            (SvgBuilderSystemSet::Insert, SvgBuilderSystemSet::Apply).chain(),
        );

        // Register systems
        app.add_systems(
            Last,
            (
                // Frame Svg Node
                insert_frame_svg_node.in_set(SvgBuilderSystemSet::Insert),
                apply_frame_node_size_change.in_set(SvgBuilderSystemSet::Apply),
                // Shape Svg Node
                insert_shape_svg_node.in_set(SvgBuilderSystemSet::Insert),
                apply_shape_node_size_change.in_set(SvgBuilderSystemSet::Apply),
            ),
        );

        #[cfg(feature = "output_events")]
        build_render_app(app, self.output_event_sender.clone());
    }
}

#[cfg(feature = "output_events")]
fn build_render_app(
    app: &mut App,
    output_event_sender: std::sync::mpsc::Sender<events::SvgBuilderOutputEvent>,
) {
    use crate::systems::extract::extract_svg_nodes_generic;
    use bevy_render::{ExtractSchedule, Render, RenderApp};
    use resources::{
        changed_svg_nodes::ChangedSvgNodesRes, output_event_sender::OutputEventSenderRes,
    };
    use systems::{
        queue::queue_svg_node_changes,
        svg_node::{frame::FrameSvgNode, shape::ShapeSvgNode},
    };

    let render_app = match app.get_sub_app_mut(RenderApp) {
        Ok(render_app) => render_app,
        Err(_) => return,
    };

    // Register resources
    render_app.init_resource::<ChangedSvgNodesRes>();
    render_app.insert_resource(OutputEventSenderRes::new(output_event_sender));

    // Register systems
    render_app.add_systems(
        ExtractSchedule,
        (
            extract_svg_nodes_generic::<FrameSvgNode>,
            extract_svg_nodes_generic::<ShapeSvgNode>,
        ),
    );
    render_app.add_systems(Render, queue_svg_node_changes);
}

#[cfg(test)]
mod tests {
    use specta::{
        export,
        ts::{BigIntExportBehavior, ExportConfig},
    };

    use super::*;

    #[test]
    fn specta_works() {
        export::ts_with_cfg(
            "./bindings.ts",
            "".into(),
            &ExportConfig::default().bigint(BigIntExportBehavior::Number),
        )
        .unwrap();
    }
}
