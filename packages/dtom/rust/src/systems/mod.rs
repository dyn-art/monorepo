use bevy_ecs::system::ResMut;

use crate::bindgen::{
    event_queue::{from_js_event_queue::FromJsEventQueue, to_js_event_queue::ToJsEventQueue},
    js_bindings,
};

pub mod construct_path;

pub fn update_system_log() {
    js_bindings::log("---- Inside update_system");
}

pub fn startup_system_log() {
    js_bindings::log("Inside startup_system");
}

pub fn forward_events_to_js(mut event_queue: ResMut<ToJsEventQueue>) {
    event_queue.forward_events_to_js();
}

pub fn poll_events_from_js(mut event_queue: ResMut<FromJsEventQueue>) {
    let events = event_queue.poll_events_from_js();
    js_bindings::log(&format!("Received events from JS: {:?}", events));
    // TODO:
}
