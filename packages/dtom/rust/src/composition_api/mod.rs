mod events;
mod plugins;

use std::sync::mpsc::{channel, Receiver};

use dyn_dtom::core::composition::dtif::DTIFComposition;
use dyn_dtom::core::composition::events::input_event::InputEvent;
use dyn_dtom::core::composition::nodes::bundles::RectangleNodeBundle;
use dyn_dtom::core::composition::Composition;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use crate::composition_api::plugins::bindgen_render_plugin::BindgenRenderPlugin;

use self::events::output_event::OutputEvent;

#[wasm_bindgen]
pub struct JsCompositionHandle {
    composition: Composition,
    event_callback: js_sys::Function,
    event_receiver: Receiver<OutputEvent>,
}

#[wasm_bindgen]
impl JsCompositionHandle {
    #[wasm_bindgen(constructor)]
    pub fn new(dtif: JsValue, event_callback: js_sys::Function) -> Self {
        let parsed_dtif: DTIFComposition = serde_wasm_bindgen::from_value(dtif).unwrap();

        let (output_event_sender, output_event_receiver) = channel::<OutputEvent>();

        // Initalize composition
        let mut composition = Composition::new(parsed_dtif);
        composition.add_plugins(BindgenRenderPlugin {
            output_event_sender: output_event_sender.clone(),
        });

        return Self {
            composition,
            event_callback,
            event_receiver: output_event_receiver,
        };
    }

    pub fn update(&mut self, input_events: JsValue) {
        // Emit input events
        let parsed_input_events: Vec<InputEvent> =
            serde_wasm_bindgen::from_value(input_events).unwrap();
        self.composition.register_events(parsed_input_events);

        // Update the internal composition state
        self.composition.update();

        // Collect all available events into a vector
        let mut output_events = Vec::new();
        while let Ok(event) = self.event_receiver.try_recv() {
            output_events.push(event);
        }

        // If we have collected events, call the JavaScript callback with the vector
        if !output_events.is_empty() {
            let js_events_value =
                serde_wasm_bindgen::to_value(&output_events).unwrap_or_else(|e| JsValue::NULL);

            let this = JsValue::NULL;
            match self.event_callback.call1(&this, &js_events_value) {
                Ok(_) => {}
                Err(e) => {
                    // TODO
                }
            }
        }
    }

    // TODO: make this an event
    // Problem: no reference to the spawned entity
    pub fn spawn_rectangle(&mut self, mixin: JsValue) -> JsValue {
        let mixin: RectangleNodeBundle = serde_wasm_bindgen::from_value(mixin).unwrap();
        return serde_wasm_bindgen::to_value(&self.composition.spawn(mixin)).unwrap();
    }
}
