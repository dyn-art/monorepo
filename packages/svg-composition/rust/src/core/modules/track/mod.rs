use bevy_app::{App, Last, Plugin};
use bevy_ecs::schedule::{IntoSystemConfigs, SystemSet};

use self::{
    resources::{changed_components::ChangedComponents, trackable_entities::TrackedEntities},
    systems::{
        extract::{check_selected_changes, extract_tracked_mixin_changes},
        queue::queue_tracked_changes,
    },
};

pub mod resources;
mod systems;

pub struct TrackPlugin;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
enum TrackSet {
    Extract,
    Queue,
}

// TODO: Could run in separate sub app parallel to the core and rendering?
impl Plugin for TrackPlugin {
    fn build(&self, app: &mut App) {
        // Register resources
        app.init_resource::<TrackedEntities>();
        app.init_resource::<ChangedComponents>();

        // Register systems
        app.add_systems(
            Last,
            (
                extract_tracked_mixin_changes.in_set(TrackSet::Extract),
                check_selected_changes.in_set(TrackSet::Extract),
                queue_tracked_changes
                    .in_set(TrackSet::Queue)
                    .after(TrackSet::Extract),
            ),
        );
    }
}
