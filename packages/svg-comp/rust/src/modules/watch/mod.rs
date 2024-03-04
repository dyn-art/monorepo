pub mod component_change;
pub mod events;
pub mod resources;
mod systems;

use self::{
    resources::{
        changed_components::ChangedComponentsRes, output_event_sender::OutputEventSenderRes,
        watched_entities::WatchedEntitiesRes,
    },
    systems::{
        extract::extract_changed_components,
        queue::{queue_changed_components, queue_composition_changes},
    },
};
use crate::events::SvgCompOutputEvent;
use bevy_app::{App, Last, Plugin};
use bevy_ecs::schedule::{IntoSystemConfigs, IntoSystemSetConfigs, SystemSet};
use std::sync::mpsc::Sender;

pub struct CompWatchPlugin {
    pub output_event_sender: Sender<SvgCompOutputEvent>,
}

// TODO: Plan to refactor into a sub-application for potential multithreading
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum CompWatchSystemSet {
    Extract,
    Queue,
}

impl Plugin for CompWatchPlugin {
    fn build(&self, app: &mut App) {
        // Register resources
        app.world
            .insert_resource(OutputEventSenderRes::new(self.output_event_sender.clone()));
        app.world.init_resource::<WatchedEntitiesRes>();
        app.world.init_resource::<ChangedComponentsRes>();

        // Configure system set
        app.configure_sets(
            Last,
            (CompWatchSystemSet::Extract, CompWatchSystemSet::Queue).chain(),
        );

        // Register systems
        app.add_systems(
            Last,
            (
                extract_changed_components.in_set(CompWatchSystemSet::Extract),
                queue_changed_components.in_set(CompWatchSystemSet::Queue),
                queue_composition_changes.in_set(CompWatchSystemSet::Queue),
            ),
        );
    }
}
