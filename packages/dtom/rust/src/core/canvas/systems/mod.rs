use crate::bindgen::js_bindings;

pub mod construct_path;

pub fn update_system_log() {
    js_bindings::log("---- Inside update_system");
}

pub fn startup_system_log() {
    js_bindings::log("Inside startup_system");
}
