use std::sync::mpsc::Sender;

use bevy_ecs::system::Resource;

use super::output_event::OutputEvent;

#[derive(Resource, Debug)]
pub struct OutputEventQueue {
    output_event_sender: Sender<OutputEvent>,
}

impl OutputEventQueue {
    pub fn new(output_event_sender: Sender<OutputEvent>) -> Self {
        Self {
            output_event_sender,
        }
    }

    pub fn push_event(&mut self, event: OutputEvent) {
        let _ = self.output_event_sender.send(event);
    }
}
