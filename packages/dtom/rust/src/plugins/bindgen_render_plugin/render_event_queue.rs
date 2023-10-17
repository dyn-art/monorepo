use bevy_ecs::system::Resource;
use js_sys::Function;
use serde::Serialize;
use std::fmt;
use wasm_bindgen::prelude::*;

#[derive(Debug, Serialize, Clone)]
pub enum RenderEvent {
    Update(String),
}

#[derive(Resource)]
pub struct RenderEventQueue {
    events: Vec<RenderEvent>,
    callbacks: Vec<Box<dyn Fn(RenderEvent) + Send + Sync + 'static>>,
}

impl fmt::Debug for RenderEventQueue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RenderEventQueue")
            .field("events", &self.events)
            .field("callbacks", &"Omitted")
            .finish()
    }
}

impl Default for RenderEventQueue {
    fn default() -> Self {
        Self {
            events: Vec::new(),
            callbacks: Vec::new(),
        }
    }
}

impl RenderEventQueue {
    pub fn push_event(&mut self, event: RenderEvent) {
        self.events.push(event);
    }

    pub fn add_js_callback(&mut self, js_callback: Function) {
        let callback = Box::new(|event: RenderEvent| {
            let this = JsValue::null();
            if let Ok(event_str) = serde_json::to_string(&event) {
                let js_event = JsValue::from(event_str);
                let _ = js_callback.call1(&this, &js_event);
            }
        }) as Box<dyn Fn(RenderEvent) + Send + Sync + 'static>;

        self.add_callback(callback);
    }

    pub fn add_callback(&mut self, callback: Box<dyn Fn(RenderEvent) + Send + Sync + 'static>) {
        self.callbacks.push(callback);
    }

    pub fn trigger_callbacks(&mut self) {
        for event in self.events.drain(..) {
            for callback in &mut self.callbacks {
                callback(event.clone());
            }
        }
    }
}
