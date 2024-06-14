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
        queue::{
            queue_canvas_changes, queue_changed_components, queue_cursor_changes,
            queue_interaction_mode_changes, queue_interaction_tool_changes,
            queue_selected_entities_changes,
        },
    },
};
use crate::events::SvgCnvOutputEvent;
use bevy_app::{App, Last, Plugin};
use bevy_ecs::schedule::{IntoSystemConfigs, IntoSystemSetConfigs, SystemSet};
use std::sync::mpsc::Sender;

pub struct CnvWatchPlugin {
    pub output_event_sender: Sender<SvgCnvOutputEvent>,
    pub interactive: bool,
}

// TODO: Plan to refactor into a sub-application for potential multithreading
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum CnvWatchSystemSet {
    Extract,
    Queue,
}

impl Plugin for CnvWatchPlugin {
    fn build(&self, app: &mut App) {
        // Register resources
        app.world
            .insert_resource(OutputEventSenderRes::new(self.output_event_sender.clone()));
        app.world.init_resource::<WatchedEntitiesRes>();
        app.world.init_resource::<ChangedComponentsRes>();

        // Configure system set
        app.configure_sets(
            Last,
            (CnvWatchSystemSet::Extract, CnvWatchSystemSet::Queue).chain(),
        );

        // Register systems
        app.add_systems(
            Last,
            (
                extract_changed_components.in_set(CnvWatchSystemSet::Extract),
                queue_changed_components.in_set(CnvWatchSystemSet::Queue),
                queue_canvas_changes.in_set(CnvWatchSystemSet::Queue),
                queue_selected_entities_changes.in_set(CnvWatchSystemSet::Queue),
            ),
        );
        if self.interactive {
            app.add_systems(
                Last,
                (
                    queue_interaction_mode_changes.in_set(CnvWatchSystemSet::Queue),
                    queue_interaction_tool_changes.in_set(CnvWatchSystemSet::Queue),
                    queue_cursor_changes.in_set(CnvWatchSystemSet::Queue),
                ),
            );
        }
    }
}
