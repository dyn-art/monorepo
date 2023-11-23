use bevy_app::{App, Last, Plugin};
use bevy_ecs::schedule::{IntoSystemConfigs, SystemSet};
use dyn_composition::core::modules::node::components::mixins::{
    DimensionMixin, RelativeTransformMixin,
};

use self::{
    resources::{changed_components::ChangedComponents, trackable_entities::TrackedEntities},
    systems::{queue_tracked_changes, track_changes},
};

pub mod resources;
mod systems;

pub struct TrackPlugin;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
enum TrackSet {
    Extract,
    Queue,
}

// #[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
// pub struct TrackSchedule;

// impl TrackSchedule {
//     pub fn base_schedule() -> Schedule {
//         use TrackSet::*;

//         let mut schedule = Schedule::new(Self);

//         schedule.configure_sets((Extract, Queue).chain());

//         return schedule;
//     }
// }

// TODO: Could run in separate sub app parallel to the core and rendering?
impl Plugin for TrackPlugin {
    fn build(&self, app: &mut App) {
        // Register resources
        app.init_resource::<TrackedEntities>();
        app.init_resource::<ChangedComponents>();

        // Register schedules
        // app.add_schedule(TrackSchedule::base_schedule());

        // Register systems
        app.add_systems(
            Last,
            (
                track_changes::<DimensionMixin>.in_set(TrackSet::Extract),
                track_changes::<RelativeTransformMixin>.in_set(TrackSet::Extract),
                queue_tracked_changes
                    .in_set(TrackSet::Queue)
                    .after(TrackSet::Extract),
            ),
        );
    }
}
