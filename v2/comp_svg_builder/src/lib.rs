pub mod events;
mod resources;
pub mod svg;
mod systems;

use bevy_app::{App, Last, Plugin};
use bevy_ecs::schedule::{IntoSystemConfigs, IntoSystemSetConfigs, SystemSet};
use resources::{
    delayed_node_modifications::DelayedNodeModificationsRes, svg_context::SvgContextRes,
};
use systems::{
    apply::{
        apply_children_changes, apply_size_mixin_changes, apply_transform_changes,
        collect_children_changes,
    },
    insert::{insert_frame_svg_node, insert_shape_svg_node},
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
    Collect,
    Apply,
    Extract,
    Queue,
}

impl Plugin for CompSvgBuilderPlugin {
    fn build(&self, app: &mut App) {
        // Register resources
        app.init_resource::<SvgContextRes>();
        app.init_resource::<DelayedNodeModificationsRes>();

        // Configure system set
        app.configure_sets(
            Last,
            (
                SvgBuilderSystemSet::Insert,
                SvgBuilderSystemSet::Collect,
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
                insert_frame_svg_node.in_set(SvgBuilderSystemSet::Insert),
                insert_shape_svg_node.in_set(SvgBuilderSystemSet::Insert),
                collect_children_changes.in_set(SvgBuilderSystemSet::Collect),
                apply_children_changes.in_set(SvgBuilderSystemSet::Apply),
                apply_size_mixin_changes.in_set(SvgBuilderSystemSet::Apply),
                apply_transform_changes.in_set(SvgBuilderSystemSet::Apply),
            ),
        );

        #[cfg(feature = "output_events")]
        {
            use crate::resources::{
                changed_svg_nodes::ChangedSvgNodesRes, output_event_sender::OutputEventSenderRes,
            };
            use crate::systems::{extract::extract_svg_nodes, queue::queue_svg_node_changes};

            // Register resources
            app.init_resource::<ChangedSvgNodesRes>();
            app.insert_resource(OutputEventSenderRes::new(self.output_event_sender.clone()));

            // Register systems
            app.add_systems(
                Last,
                (
                    extract_svg_nodes.in_set(SvgBuilderSystemSet::Extract),
                    queue_svg_node_changes.in_set(SvgBuilderSystemSet::Queue),
                ),
            );
        }
    }
}
