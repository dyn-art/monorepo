pub mod events;
pub mod resources;

use self::resources::{
    output_event_sender::OutputEventSenderRes, watched_entities::WatchedEntitiesRes,
};
use crate::events::SvgCompOutputEvent;
use bevy_app::{App, Plugin};
use std::sync::mpsc::Sender;

pub struct CompWatchPlugin {
    pub output_event_sender: Sender<SvgCompOutputEvent>,
}

impl Plugin for CompWatchPlugin {
    fn build(&self, app: &mut App) {
        // Register resources
        app.world
            .insert_resource(OutputEventSenderRes::new(self.output_event_sender.clone()));
        app.world.init_resource::<WatchedEntitiesRes>();

        // Register systems
        // TODO
    }
}
