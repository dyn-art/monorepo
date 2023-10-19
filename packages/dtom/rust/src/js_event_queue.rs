use bevy_ecs::{
    system::Resource,
    world::{FromWorld, World, WorldId},
};
use serde::Serialize;
use std::{
    collections::HashMap,
    mem::transmute,
    sync::mpsc::{channel, Receiver, Sender, TryRecvError},
};
use wasm_bindgen::{prelude::*, JsValue};

use crate::plugins::bindgen_render_plugin::ChangeSet;

#[wasm_bindgen]
extern "C" {
    fn receiveRustEvents(id: usize, events: JsValue);
}

static mut WORLD_CHANNELS: Option<HashMap<usize, (Sender<JsEvent>, Receiver<JsEvent>)>> = None;

#[derive(Debug, Serialize, Clone)]
pub enum JsEvent {
    RenderUpdate(Vec<ChangeSet>),
}

#[derive(Resource, Debug)]
pub struct JsEventQueue {
    id: usize,
    sender: Sender<JsEvent>,
}

impl FromWorld for JsEventQueue {
    fn from_world(world: &mut World) -> Self {
        JsEventQueue::new(world.id())
    }
}

impl JsEventQueue {
    pub fn new(world_id: WorldId) -> Self {
        unsafe {
            if WORLD_CHANNELS.is_none() {
                WORLD_CHANNELS = Some(HashMap::new());
            }

            let world_id_parsed: usize = unsafe { transmute(world_id) };

            // Create a new channel if it doesn't exist yet
            let (sender, _) = WORLD_CHANNELS
                .as_mut()
                .unwrap()
                .entry(world_id_parsed)
                .or_insert_with(|| {
                    let (tx, rx) = channel();
                    (tx, rx)
                });

            Self {
                sender: sender.clone(), // The sender endpoint can be cloned
                id: world_id_parsed,
            }
        }
    }

    /// Adds the incoming event via the sender.
    /// Sending over a channel is inherently thread-safe.
    pub fn push_event(&self, event: JsEvent) {
        self.sender.send(event).unwrap();
    }

    pub fn forward_events_to_js(&mut self) {
        let mut events = Vec::new();

        unsafe {
            // Find the correct receiver by index
            let optional_receiver = WORLD_CHANNELS
                .as_mut()
                .unwrap()
                .get(&self.id)
                .map(|(_, rx)| rx);

            // Drain the receiver and push all events to the events vector
            if let Some(receiver) = optional_receiver {
                loop {
                    match receiver.try_recv() {
                        Ok(event) => events.push(event),
                        Err(TryRecvError::Empty) => break,
                        Err(TryRecvError::Disconnected) => break,
                    }
                }
            }
        }

        // Send the events to JS
        if !events.is_empty() {
            let js_value = serde_wasm_bindgen::to_value(&events).unwrap();
            receiveRustEvents(self.id, js_value);
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
