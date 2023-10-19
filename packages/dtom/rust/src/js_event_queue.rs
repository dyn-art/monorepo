use bevy_ecs::system::Resource;
use serde::Serialize;
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use wasm_bindgen::{prelude::*, JsValue};

use crate::{bindgen::js_bindings, plugins::bindgen_render_plugin::ChangeSet};

#[wasm_bindgen]
extern "C" {
    fn receiveRustEvents(events: JsValue);
}

static mut RECEIVER: Option<Receiver<JsEvent>> = None;
static mut SENDER: Option<Sender<JsEvent>> = None;

#[derive(Debug, Serialize, Clone)]
pub enum JsEvent {
    RenderUpdate(Vec<ChangeSet>),
}

#[derive(Resource, Debug)]
pub struct JsEventQueue {
    sender: Sender<JsEvent>,
}

impl Default for JsEventQueue {
    fn default() -> Self {
        Self::new()
    }
}

impl JsEventQueue {
    pub fn new() -> Self {
        unsafe {
            if RECEIVER.is_none() || SENDER.is_none() {
                let (tx, rx) = channel();
                RECEIVER = Some(rx);
                SENDER = Some(tx);
            }
            Self {
                // The sender endpoint can be cloned
                sender: SENDER.clone().unwrap(),
            }
        }
    }

    // Adds the incoming event via the ender.
    // Sending over a channel is inherently thread-safe.
    pub fn push_event(&self, event: JsEvent) {
        self.sender.send(event).unwrap();
    }

    pub fn forward_events_to_js(&mut self) {
        js_bindings::log("Forwarding events to JS");
        let mut events = Vec::new();
        unsafe {
            if let Some(receiver) = &RECEIVER {
                loop {
                    match receiver.try_recv() {
                        Ok(event) => events.push(event),
                        Err(TryRecvError::Empty) => break,
                        Err(TryRecvError::Disconnected) => break,
                    }
                }
            }
        }

        if !events.is_empty() {
            let js_value = serde_wasm_bindgen::to_value(&events).unwrap();
            receiveRustEvents(js_value);
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
