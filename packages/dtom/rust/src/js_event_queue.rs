use bevy_ecs::system::Resource;
use once_cell::sync::Lazy;
use serde::Serialize;
use std::sync::Mutex;
use wasm_bindgen::{prelude::*, JsValue};

// Static JS_EVENT_QUEUE serves as a global state to hold events for JS execution.
// We use a Mutex for safe concurrent modification. This way, events can be added to
// the queue from multiple parts of the Rust code, and later polled and drained for JS handling.
// A Lazy-initialized Mutex ensures thread-safe, one-time initialization.
pub static JS_EVENT_QUEUE: Lazy<Mutex<Vec<JsEvent>>> = Lazy::new(|| Mutex::new(Vec::new()));

#[derive(Debug, Serialize, Clone)]
pub enum JsEvent {
    RenderUpdate(String),
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
    let events_str = serde_json::to_string(&events).unwrap();
    JsValue::from_str(&events_str)
}
