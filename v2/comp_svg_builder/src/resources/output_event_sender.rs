use std::sync::mpsc::Sender;

use bevy_ecs::system::Resource;

use crate::events::SvgBuilderOutputEvent;

#[derive(Resource, Debug)]
pub struct OutputEventSenderRes {
    pub sender: Sender<SvgBuilderOutputEvent>,
}
