use bevy_ecs::{
    system::{ResMut, Resource},
    world::{FromWorld, World, WorldId},
};
use serde::Serialize;
use std::mem::transmute;

use crate::bindgen::js_bindings;

pub trait BaseToJsEvent: 'static + Send + Sync + Serialize {}

#[derive(Resource, Debug)]
pub struct ToJsEventQueue<E>
where
    E: BaseToJsEvent,
{
    world_id: usize,
    queue: Vec<E>,
}

impl<E> FromWorld for ToJsEventQueue<E>
where
    E: BaseToJsEvent,
{
    fn from_world(world: &mut World) -> Self {
        return ToJsEventQueue::new(world.id());
    }
}

impl<E> ToJsEventQueue<E>
where
    E: BaseToJsEvent,
{
    pub fn new(world_id: WorldId) -> Self {
        let parsed_world_id: usize = unsafe { transmute(world_id) };
        Self {
            queue: Vec::new(),
            world_id: parsed_world_id,
        }
    }

    pub fn push_event(&mut self, event: E) {
        self.queue.push(event);
    }

    pub fn forward_events_to_js(&mut self) {
        if self.queue.is_empty() {
            return;
        }

        // Serialize events directly from the queue, avoiding extra allocation
        let js_value = serde_wasm_bindgen::to_value(&self.queue).unwrap();
        js_bindings::enqueue_rust_events(self.world_id, js_value);

        // Clear the queue after sending the events
        self.queue.clear();
    }
}

// Forward events to JS system
pub fn forward_events_to_js<E>(mut event_queue: ResMut<ToJsEventQueue<E>>)
where
    E: BaseToJsEvent,
{
    event_queue.forward_events_to_js();
}
