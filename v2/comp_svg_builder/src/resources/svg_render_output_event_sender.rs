use std::sync::mpsc::Sender;

use bevy_ecs::system::Resource;

use crate::events::SVGRenderOutputEvent;

#[derive(Resource, Debug)]
pub struct SVGRenderOutputEventSenderRes {
    pub sender: Sender<SVGRenderOutputEvent>,
}
