use bevy_ecs::{
    system::{ResMut, Resource},
    world::{FromWorld, World, WorldId},
};
use serde::Serialize;
use specta::Type;
use std::{collections::VecDeque, mem::transmute};

use crate::{
    bindgen::js_bindings, core::node::types::NodeType, plugins::bindgen_render_plugin::RenderChange,
};

#[derive(Debug, Serialize, Clone, Type)]
pub enum ToJsEvent {
    RenderUpdate {
        entity: u32,
        node_type: NodeType,
        changes: Vec<RenderChange>,
    },
    // ..
}

#[derive(Resource, Debug)]
pub struct ToJsEventQueue {
    world_id: usize,
    // https://users.rust-lang.org/t/mpsc-channels-vs-arc-mutex-vecdeque/92909
    queue: VecDeque<ToJsEvent>,
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
            queue: VecDeque::new(),
            world_id: parsed_world_id,
        }
    }

    /// Adds the incoming event via the sender.
    /// Sending over a channel is inherently thread-safe.
    pub fn push_event(&mut self, event: ToJsEvent) {
        self.queue.push_back(event);
    }

    pub fn forward_events_to_js(&mut self) {
        let mut events: Vec<ToJsEvent> = Vec::new();

        // Drain events push all events to the events vector
        self.queue.drain(..).for_each(|event| events.push(event));

        // Send the events to JS
        if !events.is_empty() {
            let js_value = serde_wasm_bindgen::to_value(&events).unwrap();
            js_bindings::enqueue_rust_events(self.world_id, js_value);
        }
    }
}

pub fn forward_events_to_js(mut event_queue: ResMut<ToJsEventQueue>) {
    event_queue.forward_events_to_js();
}
