use bevy_ecs::{
    system::Resource,
    world::{FromWorld, World, WorldId},
};
use serde::Serialize;
#[cfg(feature = "cli")]
use specta::Type;
use std::{
    mem::transmute,
    sync::{
        mpsc::{channel, Receiver, Sender, TryRecvError},
        Arc, Mutex,
    },
};
use wasm_bindgen::{prelude::*, JsValue};

use crate::plugins::bindgen_render_plugin::RenderChange;

#[wasm_bindgen]
extern "C" {
    fn enqueue_rust_events(world_id: usize, events: JsValue);
}

#[cfg_attr(feature = "cli", derive(Type))]
#[derive(Debug, Serialize, Clone)]
pub enum ToJsEvent {
    RenderUpdate {
        entity: u32,
        changes: Vec<RenderChange>,
    },
    // ..
}

#[derive(Resource, Debug)]
pub struct ToJsEventQueue {
    world_id: usize,
    sender: Sender<ToJsEvent>,
    receiver: Arc<Mutex<Receiver<ToJsEvent>>>,
}

impl FromWorld for ToJsEventQueue {
    fn from_world(world: &mut World) -> Self {
        return ToJsEventQueue::new(world.id());
    }
}

impl ToJsEventQueue {
    pub fn new(world_id: WorldId) -> Self {
        let parsed_world_id: usize = unsafe { transmute(world_id) };
        let (tx, rx) = channel();
        Self {
            sender: tx,
            receiver: Arc::new(Mutex::new(rx)),
            world_id: parsed_world_id,
        }
    }

    /// Adds the incoming event via the sender.
    /// Sending over a channel is inherently thread-safe.
    pub fn push_event(&self, event: ToJsEvent) {
        self.sender.send(event).unwrap();
    }

    pub fn forward_events_to_js(&mut self) {
        let mut events: Vec<ToJsEvent> = Vec::new();

        // Drain the receiver and push all events to the events vector
        loop {
            match self.receiver.lock().unwrap().try_recv() {
                Ok(event) => events.push(event),
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => break,
            }
        }

        // Send the events to JS
        if !events.is_empty() {
            let js_value = serde_wasm_bindgen::to_value(&events).unwrap();
            enqueue_rust_events(self.world_id, js_value); // Call into JS
        }
    }
}

// use bevy_ecs::system::Resource;
// use serde::Serialize;
// use std::sync::Mutex;
// use wasm_bindgen::{prelude::*, JsValue};

// use crate::plugins::bindgen_render_plugin::ChangeSet;

// // Static JS_EVENT_QUEUE serves as a global state to hold events for JS execution.
// // We use a Mutex for safe concurrent modification. This way, events can be added to
// // the queue from multiple parts of the Rust code, and later polled and drained for JS handling.
// // A Lazy-initialized Mutex ensures thread-safe, one-time initialization.
// pub static JS_EVENT_QUEUE: Mutex<Vec<JsEvent>> = Mutex::new(Vec::new());

// #[derive(Debug, Serialize, Clone)]
// pub enum JsEvent {
//     RenderUpdate(Vec<ChangeSet>),
// }

// #[derive(Resource, Default, Debug)]
// pub struct JsEventQueue;

// impl JsEventQueue {
//     // Adds the incoming event to the global JS_EVENT_QUEUE.
//     // Mutex guarantees that the addition is thread-safe.
//     pub fn push_event(&mut self, event: JsEvent) {
//         let mut js_event_queue = JS_EVENT_QUEUE.lock().unwrap();
//         js_event_queue.push(event);
//     }
// }

// #[wasm_bindgen]
// pub fn poll_js_event_queue() -> JsValue {
//     let mut event_queue = JS_EVENT_QUEUE.lock().unwrap();
//     let events = event_queue.drain(..).collect::<Vec<_>>();
//     serde_wasm_bindgen::to_value(&events).unwrap()
// }
