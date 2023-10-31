use bevy_ecs::{
    entity::Entity,
    system::{ResMut, Resource},
    world::{FromWorld, World, WorldId},
};
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
    queue: Vec<ToJsEvent>,
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
        Self {
            queue: Vec::new(),
            world_id: parsed_world_id,
            events: Vec::new(),
        }
    }

    pub fn push_event(&mut self, event: ToJsEvent) {
        self.queue.push(event);
    }

    pub fn forward_events_to_js(&mut self) {
        // Clear previous events
        self.events.clear();

        // Drain events from queue and push them into the events vec
        self.queue.drain(..).into_iter().for_each(|event| {
            self.events.push(event.clone());
        });

        // Send events to JS
        if !self.events.is_empty() {
            let js_value = serde_wasm_bindgen::to_value(&self.events).unwrap();
            js_bindings::enqueue_rust_events(self.world_id, js_value);
        }
    }
}

pub fn forward_events_to_js(mut event_queue: ResMut<ToJsEventQueue>) {
    event_queue.forward_events_to_js();
}
