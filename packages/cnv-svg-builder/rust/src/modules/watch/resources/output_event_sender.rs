use crate::events::SvgCnvOutputEvent;
use bevy_ecs::system::Resource;
use std::sync::mpsc::Sender;

#[derive(Resource, Debug)]
pub struct OutputEventSenderRes {
    output_event_sender: Sender<SvgCnvOutputEvent>,
}

impl OutputEventSenderRes {
    pub fn new(output_event_sender: Sender<SvgCnvOutputEvent>) -> Self {
        Self {
            output_event_sender,
        }
    }

    pub fn push_event(&self, event: SvgCnvOutputEvent) {
        let _ = self.output_event_sender.send(event);
    }
}
