pub mod events;
mod resources;
pub mod svg;
mod systems;

use bevy_app::{App, Last, Plugin};
use bevy_ecs::schedule::{IntoSystemConfigs, IntoSystemSetConfigs, SystemSet};
use resources::svg_context::SvgContextRes;
use systems::svg_node::{
    frame::{apply_frame_node_size_change, insert_frame_svg_node},
    shape::{apply_shape_node_size_change, insert_shape_svg_node},
};

pub struct CompSvgBuilderPlugin {
    #[cfg(feature = "output_events")]
    pub output_event_sender: std::sync::mpsc::Sender<crate::events::SvgBuilderOutputEvent>,
}

// TODO: Plan to refactor into a sub-application for potential multithreading
// Currently, the challenge lies in managing the spawning (when absent)
// and modification of the SvgNode bundle component alongside its associated entity,
// due to the deferred execution nature of entity spawn commands within the ECS schedule.
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum SvgBuilderSystemSet {
    Insert,
    Apply,
    Extract,
    Queue,
}

impl Plugin for CompSvgBuilderPlugin {
    fn build(&self, app: &mut App) {
        // Register resources
        app.init_resource::<SvgContextRes>();

        // Configure system set
        app.configure_sets(
            Last,
            (
                SvgBuilderSystemSet::Insert,
                SvgBuilderSystemSet::Apply,
                SvgBuilderSystemSet::Extract,
                SvgBuilderSystemSet::Queue,
            )
                .chain(),
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
        {
            use crate::resources::{
                changed_svg_nodes::ChangedSvgNodesRes, output_event_sender::OutputEventSenderRes,
            };
            use crate::systems::{
                extract::extract_svg_nodes_generic,
                queue::queue_svg_node_changes,
                svg_node::{frame::FrameSvgNode, shape::ShapeSvgNode},
            };

            // Register resources
            app.init_resource::<ChangedSvgNodesRes>();
            app.insert_resource(OutputEventSenderRes::new(self.output_event_sender.clone()));

            // Register systems
            app.add_systems(
                Last,
                (
                    extract_svg_nodes_generic::<FrameSvgNode>.in_set(SvgBuilderSystemSet::Extract),
                    extract_svg_nodes_generic::<ShapeSvgNode>.in_set(SvgBuilderSystemSet::Extract),
                    queue_svg_node_changes.in_set(SvgBuilderSystemSet::Queue),
                ),
            );
        }
    }
}
