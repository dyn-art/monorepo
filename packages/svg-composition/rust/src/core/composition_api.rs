use std::sync::mpsc::Receiver;

use dyn_composition::core::composition::Composition;
use js_sys::wasm_bindgen;
use wasm_bindgen::prelude::wasm_bindgen;

use super::events::output_event::OutputEvent;

#[wasm_bindgen]
pub struct JsCompositionHandle {
    composition: Composition,
    event_callback: js_sys::Function,
    event_receiver: Receiver<OutputEvent>,
}
