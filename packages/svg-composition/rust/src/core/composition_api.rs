use std::sync::mpsc::{channel, Receiver};

use bevy_ecs::entity::Entity;
use dyn_bevy_render_skeleton::RenderApp;
use dyn_composition::core::composition::Composition;
use dyn_composition::core::dtif::DTIFComposition;
use dyn_composition::core::modules::node::components::bundles::RectangleNodeBundle;
use serde::de::DeserializeOwned;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use crate::core::events::input_event::AnyInputEvent;
use crate::core::modules::svg_render::resources::svg_composition::svg_composition::SVGComposition;
use crate::core::modules::svg_render::SvgRenderPlugin;

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
        composition.add_plugins(SvgRenderPlugin {
            output_event_sender: output_event_sender.clone(),
        });

        return Self {
            composition,
            event_callback,
            event_receiver: output_event_receiver,
        };
    }

    pub fn update(&mut self, input_events: JsValue) {
        let parsed_input_events: Result<Vec<AnyInputEvent>, _> =
            serde_wasm_bindgen::from_value(input_events);

        // Emit input events into the world
        match parsed_input_events {
            Ok(events) => {
                for any_input_event in events {
                    match any_input_event {
                        AnyInputEvent::Core(any_event) => {
                            self.composition.register_events(any_event.events);
                        }
                        AnyInputEvent::Interaction(any_event) => {
                            self.composition.register_events(any_event.events);
                        }
                    }
                }
            }
            Err(e) => {
                // TODO
            }
        }

        // Update all registered worlds via schedules and thus the composition state
        self.composition.update();

        // Collect all output events emitted during the last update cycle
        let mut output_events = Vec::new();
        while let Ok(event) = self.event_receiver.try_recv() {
            output_events.push(event);
        }

        // Call the JavaScript callback with the collected output events
        if !output_events.is_empty() {
            let js_events_value =
                serde_wasm_bindgen::to_value(&output_events).unwrap_or_else(|_| JsValue::NULL);

            let this = JsValue::NULL;
            if let Err(e) = self.event_callback.call1(&this, &js_events_value) {
                // TODO
            }
        }
    }

    #[wasm_bindgen(js_name = spawnRectangleNode)]
    pub fn spawn_rectangle_node(
        &mut self,
        mixin: JsValue,
        maybe_parent_id: JsValue,
    ) -> Result<JsValue, JsValue> {
        let mixin: RectangleNodeBundle = serde_wasm_bindgen::from_value(mixin)
            .map_err(|e| JsValue::from_str(&format!("Error parsing mixin: {:?}", e)))?;
        let maybe_parent_id = convert_optional_jsvalue::<Entity>(maybe_parent_id);

        // Spawn rectangle into main world
        let entity = self.composition.spawn(mixin, maybe_parent_id);

        return serde_wasm_bindgen::to_value(&entity).map_err(|_| JsValue::NULL);
    }

    fn get_svg_composition(&self) -> Result<&SVGComposition, String> {
        let app = self.composition.get_app();

        let sub_app = app
            .get_sub_app(RenderApp)
            .map_err(|e| format!("RenderApp error: {:?}", e))?;

        return sub_app
            .world
            .get_resource::<SVGComposition>()
            .ok_or_else(|| "SVGComposition resource not found".to_string());
    }

    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> Option<String> {
        self.get_svg_composition().unwrap().to_string()
    }
}

fn convert_optional_jsvalue<T>(value: JsValue) -> Option<T>
where
    T: DeserializeOwned,
{
    if value.is_undefined() || value.is_null() {
        None
    } else {
        serde_wasm_bindgen::from_value(value).ok()
    }
}
