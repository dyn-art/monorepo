use std::sync::mpsc::{channel, Receiver};

use dyn_composition::core::composition::Composition;
use dyn_composition::core::dtif::DTIFComposition;
use dyn_composition::core::modules::composition::events::input_event::InputEvent;
use dyn_composition::core::modules::node::components::bundles::RectangleNodeBundle;
use log::info;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use crate::core::modules::bindgen_render::BindgenRenderPlugin;

use super::modules::output_event::OutputEvent;

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

        info!("Create new Composition Interface to Javascript");

        // Initalize composition
        let mut composition = Composition::new(Option::from(parsed_dtif));
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

        // Call the JavaScript callback with the vector
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

    pub fn spawn_rectangle_node(&mut self, mixin: JsValue) -> JsValue {
        let mixin: RectangleNodeBundle = serde_wasm_bindgen::from_value(mixin).unwrap();
        let entity = self.composition.spawn(mixin);
        return serde_wasm_bindgen::to_value(&entity).unwrap();
    }
}
