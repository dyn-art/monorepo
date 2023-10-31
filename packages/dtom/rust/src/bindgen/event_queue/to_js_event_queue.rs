use bevy_ecs::{
    entity::Entity,
    system::{ResMut, Resource},
    world::{FromWorld, World, WorldId},
};
use crossbeam_channel::{unbounded, Receiver, Sender};
use serde::Serialize;
use specta::Type;
use std::mem::transmute;

use crate::{
    bindgen::js_bindings, core::node::types::NodeType, plugins::bindgen_render_plugin::RenderChange,
};

#[derive(Debug, Serialize, Clone, Type)]
pub enum ToJsEvent {
    RenderUpdate {
        entity: Entity,
        node_type: NodeType,
        changes: Vec<RenderChange>,
    },
    // ..
}

#[derive(Resource, Debug)]
pub struct ToJsEventQueue {
    world_id: usize,
    receiver: Receiver<ToJsEvent>,
    sender: Sender<ToJsEvent>,
    // Reuse this Vec to avoid reallocations
    events: Vec<ToJsEvent>,
}

impl FromWorld for ToJsEventQueue {
    fn from_world(world: &mut World) -> Self {
        return ToJsEventQueue::new(world.id());
    }
}

impl ToJsEventQueue {
    pub fn new(world_id: WorldId) -> Self {
        let parsed_world_id: usize = unsafe { transmute(world_id) };
        let (sender, receiver) = unbounded();

        Self {
            sender,
            receiver,
            world_id: parsed_world_id,
            events: Vec::new(),
        }
    }

    /// Adds the incoming event via the sender.
    /// Sending over a channel is inherently thread-safe.
    pub fn push_event(&mut self, event: ToJsEvent) {
        self.sender.send(event);
    }

    pub fn forward_events_to_js(&mut self) {
        // Clear previous events
        self.events.clear();

        // Get events from receiver and push them into the events vec
        for event in self.receiver.try_iter() {
            self.events.push(event.clone());
        }

        // Send the events to JS
        if !self.events.is_empty() {
            let js_value = serde_wasm_bindgen::to_value(&self.events).unwrap();
            js_bindings::enqueue_rust_events(self.world_id, js_value);
        }
    }
}

pub fn forward_events_to_js(mut event_queue: ResMut<ToJsEventQueue>) {
    event_queue.forward_events_to_js();
}
