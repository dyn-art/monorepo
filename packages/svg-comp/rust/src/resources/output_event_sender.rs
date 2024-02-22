use std::sync::mpsc::Sender;

use bevy_ecs::system::Resource;

use crate::events::SvgCompOutputEvent;

#[derive(Resource, Debug)]
pub struct OutputEventSenderRes {
    output_event_sender: Sender<SvgCompOutputEvent>,
}

impl OutputEventSenderRes {
    pub fn new(output_event_sender: Sender<SvgCompOutputEvent>) -> Self {
        Self {
            output_event_sender,
        }
    }

    pub fn push_event(&self, event: SvgCompOutputEvent) {
        let _ = self.output_event_sender.send(event);
    }
}
