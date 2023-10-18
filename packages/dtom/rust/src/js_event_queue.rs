use bevy_ecs::system::Resource;
use serde::Serialize;
use std::sync::Mutex;
use wasm_bindgen::{prelude::*, JsValue};

use crate::plugins::bindgen_render_plugin::ChangeSet;

// Static JS_EVENT_QUEUE serves as a global state to hold events for JS execution.
// We use a Mutex for safe concurrent modification. This way, events can be added to
// the queue from multiple parts of the Rust code, and later polled and drained for JS handling.
// A Lazy-initialized Mutex ensures thread-safe, one-time initialization.
pub static JS_EVENT_QUEUE: Mutex<Vec<JsEvent>> = Mutex::new(Vec::new());

#[derive(Debug, Serialize, Clone)]
pub enum JsEvent {
    RenderUpdate(Vec<ChangeSet>),
}

#[derive(Resource, Default, Debug)]
pub struct JsEventQueue;

impl JsEventQueue {
    // Adds the incoming event to the global JS_EVENT_QUEUE.
    // Mutex guarantees that the addition is thread-safe.
    pub fn push_event(&mut self, event: JsEvent) {
        let mut js_event_queue = JS_EVENT_QUEUE.lock().unwrap();
        js_event_queue.push(event);
    }
}

#[wasm_bindgen]
pub fn poll_js_event_queue() -> JsValue {
    let mut event_queue = JS_EVENT_QUEUE.lock().unwrap();
    let events = event_queue.drain(..).collect::<Vec<_>>();
    serde_wasm_bindgen::to_value(&events).unwrap()
}

// use bevy_ecs::system::Resource;
// use serde::Serialize;
// use std::sync::{
//     mpsc::{channel, Receiver, Sender, TryRecvError},
//     Mutex,
// };
// use wasm_bindgen::{prelude::*, JsValue};

// use crate::plugins::bindgen_render_plugin::ChangeSet;

// // Static receiver and sender for JS poll to use
// static RECEIVER: Mutex<Option<Receiver<JsEvent>>> = Mutex::new(None);
// static SENDER: Mutex<Option<Sender<JsEvent>>> = Mutex::new(None);

// #[derive(Debug, Serialize, Clone)]
// pub enum JsEvent {
//     RenderUpdate(Vec<ChangeSet>),
// }

// #[derive(Resource, Debug)]
// pub struct JsEventQueue {
//     sender: Sender<JsEvent>,
// }

// impl Default for JsEventQueue {
//     fn default() -> Self {
//         Self::new()
//     }
// }

// impl JsEventQueue {
//     pub fn new() -> Self {
//         let mut receiver = RECEIVER.lock().unwrap();
//         let mut sender = SENDER.lock().unwrap();

//         if receiver.is_none() || sender.is_none() {
//             let (tx, rx) = channel();
//             *receiver = Some(rx);
//             *sender = Some(tx);
//         }
//         Self {
//             // The sender endpoint can be copied
//             sender: sender.clone().unwrap(),
//         }
//     }

//     // Adds the incoming event via the Sender.
//     // Sending over a channel is thread-safe.
//     pub fn push_event(&self, event: JsEvent) {
//         self.sender.send(event).unwrap();
//     }
// }

// #[wasm_bindgen]
// pub fn poll_js_event_queue() -> JsValue {
//     let mut events = Vec::new();
//     let receiver = RECEIVER.lock().unwrap();

//     if let Some(receiver) = &*receiver {
//         loop {
//             match receiver.try_recv() {
//                 Ok(event) => events.push(event),
//                 Err(TryRecvError::Empty) => break,
//                 Err(TryRecvError::Disconnected) => break,
//             }
//         }
//     }

//     serde_wasm_bindgen::to_value(&events).unwrap()
// }
