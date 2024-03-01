pub mod events;
mod resources;
pub mod svg;
mod systems;

use bevy_app::{App, Last, Plugin};
use bevy_ecs::schedule::{IntoSystemConfigs, IntoSystemSetConfigs, SystemSet};
use resources::{
    delayed_svg_bundle_modifications::DelayedSvgBundleModificationsRes, svg_context::SvgContextRes,
};
use systems::{
    apply::{
        apply_blend_mode_mixin_changes, apply_node_children_changes, apply_opacity_mixin_changes,
        apply_size_mixin_changes, apply_solid_paint_changes, apply_transform_changes,
        apply_visibility_mixin_changes, collect_node_children_changes,
    },
    insert::{insert_fills, insert_frame_node_svg_bundle, insert_shape_node_svg_bundle},
};

pub struct CompSvgBuilderPlugin {
    #[cfg(any(feature = "output_svg_element_changes", feature = "output_svg_string"))]
    pub output_event_sender: std::sync::mpsc::Sender<crate::events::SvgBuilderOutputEvent>,
}

// TODO: Plan to refactor into a sub-application for potential multithreading
// Currently, the challenge lies in managing the spawning (when absent)
// and modification of the SvgBundle component alongside its associated entity,
// due to the deferred execution nature of entity spawn commands within the ECS schedule.
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum SvgBuilderSystemSet {
    Insert,
    PostInsert,
    Collect,
    Apply,
    Extract,
    Queue,
}

impl Plugin for CompSvgBuilderPlugin {
    fn build(&self, app: &mut App) {
        // Register resources
        app.init_resource::<SvgContextRes>();
        app.init_resource::<DelayedSvgBundleModificationsRes>();

        // Configure system set
        app.configure_sets(
            Last,
            (
                SvgBuilderSystemSet::Insert,
                SvgBuilderSystemSet::PostInsert,
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
                insert_frame_node_svg_bundle.in_set(SvgBuilderSystemSet::Insert),
                insert_shape_node_svg_bundle.in_set(SvgBuilderSystemSet::Insert),
                insert_fills.in_set(SvgBuilderSystemSet::PostInsert),
                collect_node_children_changes.in_set(SvgBuilderSystemSet::Collect),
                apply_node_children_changes.in_set(SvgBuilderSystemSet::Apply),
                apply_visibility_mixin_changes.in_set(SvgBuilderSystemSet::Apply),
                apply_size_mixin_changes.in_set(SvgBuilderSystemSet::Apply),
                apply_transform_changes.in_set(SvgBuilderSystemSet::Apply),
                apply_opacity_mixin_changes.in_set(SvgBuilderSystemSet::Apply),
                apply_blend_mode_mixin_changes.in_set(SvgBuilderSystemSet::Apply),
                apply_solid_paint_changes.in_set(SvgBuilderSystemSet::Apply),
            ),
        );

        #[cfg(feature = "output_svg_string")]
        {
            // TODO
        }

        #[cfg(feature = "output_svg_element_changes")]
        {
            use crate::resources::{
                changed_svg_bundles::ChangedSvgBundlesRes,
                output_event_sender::OutputEventSenderRes,
            };
            use crate::systems::{extract::extract_svg_bundles, queue::queue_svg_bundle_changes};

            // Register resources
            app.init_resource::<ChangedSvgBundlesRes>();
            app.insert_resource(OutputEventSenderRes::new(self.output_event_sender.clone()));

            // Register systems
            app.add_systems(
                Last,
                (
                    extract_svg_bundles.in_set(SvgBuilderSystemSet::Extract),
                    queue_svg_bundle_changes.in_set(SvgBuilderSystemSet::Queue),
                ),
            );
        }
    }
}
