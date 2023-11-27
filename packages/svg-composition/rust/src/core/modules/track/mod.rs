use bevy_app::{App, Last, Plugin};
use bevy_ecs::schedule::{IntoSystemConfigs, IntoSystemSetConfigs, SystemSet};

use self::{
    resources::{changed_components::ChangedComponentsRes, trackable_entities::TrackedEntitiesRes},
    systems::{
        check::{check_interactive_composition_changes, check_selection_changes},
        extract::extract_tracked_mixin_changes,
        queue::queue_tracked_changes,
    },
};

pub mod resources;
mod systems;

pub struct TrackPlugin;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
enum TrackSet {
    Extract,
    Check,
    Queue,
}

// TODO: Could run in separate sub app parallel to the core and rendering?
impl Plugin for TrackPlugin {
    fn build(&self, app: &mut App) {
        // Register resources
        app.init_resource::<TrackedEntitiesRes>();
        app.init_resource::<ChangedComponentsRes>();

        // Configure system sets
        app.configure_sets(
            Last,
            (TrackSet::Extract, TrackSet::Check, TrackSet::Queue).chain(),
        );

        // Register systems
        app.add_systems(
            Last,
            (
                extract_tracked_mixin_changes.in_set(TrackSet::Extract),
                check_selection_changes.in_set(TrackSet::Check),
                check_interactive_composition_changes.in_set(TrackSet::Check),
                queue_tracked_changes.in_set(TrackSet::Queue),
            ),
        );
    }
}
