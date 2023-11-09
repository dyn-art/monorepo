use std::sync::mpsc::Sender;

use bevy_ecs::{entity::Entity, system::Resource};
use dyn_composition::core::modules::node::components::types::NodeType;
use serde::Serialize;
use specta::Type;

use crate::core::modules::bindgen_render::RenderChange;

#[derive(Debug, Serialize, Clone, Type)]
#[serde(tag = "type")]
pub enum OutputEvent {
    RenderUpdate(RenderUpdateEvent),
}

#[derive(Debug, Serialize, Clone, Type)]
pub struct RenderUpdateEvent {
    pub entity: Entity,
    pub parent: Option<Entity>,
    #[serde(rename = "nodeType")]
    pub node_type: NodeType,
    pub changes: Vec<RenderChange>,
}

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
        match self.output_event_sender.send(event) {
            Ok(_) => {}
            Err(e) => {
                // TODO
            }
        }
    }
}
