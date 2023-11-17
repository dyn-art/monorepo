use std::sync::mpsc::{channel, Receiver};

use dyn_composition::core::composition::Composition;
use dyn_composition::core::dtif::DTIFComposition;
use dyn_composition::core::modules::node::components::bundles::RectangleNodeBundle;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use crate::core::events::input_event::AnyInputEvent;
use crate::core::modules::bindgen_render::BindgenRenderPlugin;

use super::events::output_event::OutputEvent;

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
        let parsed_input_events: Vec<AnyInputEvent> =
            serde_wasm_bindgen::from_value(input_events).unwrap();

        // Emit input events
        for any_event in parsed_input_events {
            match any_event {
                AnyInputEvent::Core(any_event) => {
                    self.composition.register_events(any_event.events);
                }
                AnyInputEvent::Interaction(any_event) => {
                    self.composition.register_events(any_event.events);
                }
            }
        }

        // Update the internal composition state
        self.composition.update();

        // Collect all (in the last update cycle) emited events into a vector
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
        let entity = self.composition.spawn(mixin, None);
        return serde_wasm_bindgen::to_value(&entity).unwrap();
    }
}
