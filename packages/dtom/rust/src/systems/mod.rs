use bevy_ecs::system::ResMut;

use crate::{bindgen::js_bindings, event_queue::js_event_queue::JsEventQueue};

pub mod construct_path;

pub fn update_system_log() {
    js_bindings::log("---- Inside update_system");
}

pub fn startup_system_log() {
    js_bindings::log("Inside startup_system");
}

pub fn forward_events_to_js(mut event_queue: ResMut<JsEventQueue>) {
    event_queue.forward_events_to_js();
}
