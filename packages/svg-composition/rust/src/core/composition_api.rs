use std::sync::mpsc::{channel, Receiver};

use bevy_ecs::entity::Entity;
use dyn_bevy_render_skeleton::RenderApp;
use dyn_composition::core::composition::Composition;
use dyn_composition::core::dtif::DTIFComposition;
use dyn_composition::core::modules::node::components::bundles::RectangleNodeBundle;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use crate::core::events::input_event::AnyInputEvent;
use crate::core::helper::convert_optional_jsvalue;
use crate::core::modules::svg_render::resources::svg_composition::SVGComposition;
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
        let parsed_dtif: Result<DTIFComposition, _> = serde_wasm_bindgen::from_value(dtif);
        let parsed_dtif = match parsed_dtif {
            Ok(dtif) => dtif,
            Err(_) => {
                panic!("Invalid DTIF provided!")
            }
        };
        let (output_event_sender, output_event_receiver) = channel::<OutputEvent>();

        // Initalize composition
        let mut composition = Composition::new(Some(parsed_dtif));
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

        // Emit input events into the Bevy world
        if let Ok(events) = parsed_input_events {
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

        // Execute Bevy update cycle (-> Advance the execution of the schedule by one cycle)
        self.composition.update();

        // Collect output events that were emitted during the last update cycle
        let mut output_events = Vec::new();
        while let Ok(event) = self.event_receiver.try_recv() {
            output_events.push(event);
        }

        // Invoke the JavaScript callback if with collected output events
        if !output_events.is_empty() {
            let js_events_value =
                serde_wasm_bindgen::to_value(&output_events).unwrap_or_else(|_| JsValue::NULL);

            let this = JsValue::NULL;
            let _ = self.event_callback.call1(&this, &js_events_value);
        }
    }

    #[wasm_bindgen(js_name = spawnRectangleNode)]
    pub fn spawn_rectangle_node(&mut self, mixin: JsValue, maybe_parent_id: JsValue) -> JsValue {
        let mixin: RectangleNodeBundle = match serde_wasm_bindgen::from_value(mixin) {
            Ok(mixin) => mixin,
            Err(_) => return JsValue::NULL,
        };
        let maybe_parent_id = convert_optional_jsvalue::<Entity>(maybe_parent_id);

        // Spawn a new rectangle node in the composition
        let entity = self.composition.spawn(mixin, maybe_parent_id);

        return serde_wasm_bindgen::to_value(&entity).unwrap_or(JsValue::NULL);
    }

    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> Option<String> {
        self.get_svg_composition()?.to_string()
    }

    // =============================================================================
    // Helper
    // =============================================================================

    fn get_svg_composition(&self) -> Option<&SVGComposition> {
        let app = self.composition.get_app();
        let sub_app = app.get_sub_app(RenderApp).ok()?;
        return sub_app.world.get_resource::<SVGComposition>();
    }
}
