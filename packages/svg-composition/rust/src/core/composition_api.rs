use std::collections::HashSet;
use std::sync::mpsc::{channel, Receiver};

use bevy_ecs::entity::Entity;
use dyn_bevy_render_skeleton::RenderApp;
use dyn_composition::core::composition::Composition;
use dyn_composition::core::dtif::DTIFComposition;
use dyn_composition::core::modules::node::components::bundles::{NodeBundle, RectangleNodeBundle};
use dyn_composition::core::modules::node::components::mixins::{
    DimensionMixin, Paint, RelativeTransformMixin,
};
use dyn_svg_render::events::output_event::RenderUpdateEvent;
use dyn_svg_render::mixin_change::{MixinChange, MixinChangeRelativeTransformMixin};
use dyn_svg_render::resources::svg_composition::SVGCompositionRes;
use dyn_svg_render::SvgRenderPlugin;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use crate::bindgen::utils::convert_optional_jsvalue;
use crate::core::events::input_event::AnyInputEvent;
use crate::core::modules::track::resources::tracked_entities::{
    TrackableMixinType, TrackedEntitiesRes,
};
use crate::core::modules::track::TrackPlugin;

use super::events::output_event::OutputEvent;
use super::events::output_event_queue::OutputEventQueueRes;

#[wasm_bindgen]
pub struct JsCompositionHandle {
    composition: Composition,
    event_callback: js_sys::Function,
    output_event_receiver: Receiver<OutputEvent>,
    render_event_receiver: Receiver<RenderUpdateEvent>,
}

#[wasm_bindgen]
impl JsCompositionHandle {
    #[wasm_bindgen(constructor)]
    pub fn new(dtif: JsValue, event_callback: js_sys::Function) -> Self {
        let parsed_dtif: Result<DTIFComposition, _> = serde_wasm_bindgen::from_value(dtif);
        let parsed_dtif = match parsed_dtif {
            Ok(dtif) => dtif,
            Err(e) => {
                panic!("Invalid DTIF provided: {}", e.to_string())
            }
        };
        let (output_event_sender, output_event_receiver) = channel::<OutputEvent>();
        let (render_event_sender, render_event_receiver) = channel::<RenderUpdateEvent>();

        // Initalize composition
        let mut composition = Composition::new(parsed_dtif);
        let app = composition.get_app_mut();

        // Register plugins
        app.add_plugins((
            SvgRenderPlugin {
                render_event_sender: Some(render_event_sender),
            },
            TrackPlugin,
        ));

        // Register resources
        app.world
            .insert_resource(OutputEventQueueRes::new(output_event_sender));

        return Self {
            composition,
            event_callback,
            output_event_receiver,
            render_event_receiver,
        };
    }

    // =========================================================================
    // Lifecycle
    // =========================================================================

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
        while let Ok(event) = self.output_event_receiver.try_recv() {
            output_events.push(event);
        }
        while let Ok(event) = self.render_event_receiver.try_recv() {
            output_events.push(OutputEvent::RenderUpdate(event));
        }

        // Invoke the JavaScript callback if with collected output events
        if !output_events.is_empty() {
            let js_events_value =
                serde_wasm_bindgen::to_value(&output_events).unwrap_or_else(|_| JsValue::NULL);

            let this = JsValue::NULL;
            let _ = self.event_callback.call1(&this, &js_events_value);
        }
    }

    // =========================================================================
    // Tracking
    // =========================================================================

    #[wasm_bindgen(js_name = trackEntity)]
    pub fn track_entity(
        &mut self,
        entity: JsValue,
        to_track_mixins: JsValue,
        initial_value: bool,
    ) -> JsValue {
        let entity: Entity = match serde_wasm_bindgen::from_value(entity) {
            Ok(entity) => entity,
            Err(_) => return JsValue::FALSE,
        };
        let to_track_mixins: Vec<TrackableMixinType> =
            match serde_wasm_bindgen::from_value(to_track_mixins) {
                Ok(to_track_mixins) => to_track_mixins,
                Err(_) => return JsValue::FALSE,
            };

        let app = self.composition.get_app_mut();

        // Collect intial values
        let mut changes: Vec<MixinChange> = Vec::with_capacity(to_track_mixins.len());
        if initial_value {
            for component_type in &to_track_mixins {
                match component_type {
                    TrackableMixinType::Dimension => {
                        if let Some(mixin) = app.world.get::<DimensionMixin>(entity) {
                            changes.push(MixinChange::Dimension(mixin.clone()))
                        }
                    }
                    TrackableMixinType::RelativeTransform => {
                        if let Some(mixin) = app.world.get::<RelativeTransformMixin>(entity) {
                            changes.push(MixinChange::RelativeTransform(
                                MixinChangeRelativeTransformMixin {
                                    relative_transform: mixin.clone(),
                                },
                            ))
                        }
                    }
                }
            }
        }

        // Update tracked entities
        let mut tracked_entities = app.world.get_resource_mut::<TrackedEntitiesRes>().unwrap();
        tracked_entities
            .tracked_entities
            .entry(entity)
            .or_insert_with(HashSet::new)
            .extend(to_track_mixins);

        if initial_value {
            return serde_wasm_bindgen::to_value(&changes).unwrap_or_else(|_| JsValue::TRUE);
        } else {
            return JsValue::TRUE;
        }
    }

    #[wasm_bindgen(js_name = untrackEntity)]
    pub fn untrack_entity(&mut self, entity: JsValue) -> bool {
        let entity: Entity = match serde_wasm_bindgen::from_value(entity) {
            Ok(entity) => entity,
            Err(_) => return false,
        };

        let mut tracked_entities = self
            .composition
            .get_app_mut()
            .world
            .get_resource_mut::<TrackedEntitiesRes>()
            .unwrap();

        // Remove entity from the tracked entities
        let removed = tracked_entities.tracked_entities.remove(&entity).is_some();

        return removed;
    }

    // =========================================================================
    // Spawn
    // =========================================================================

    // TODO: Create spawn events instead of these methods so that it can be dynamically changed

    #[wasm_bindgen(js_name = spawnPaint)]
    pub fn spawn_paint(&mut self, paint: JsValue) -> JsValue {
        let paint: Paint = match serde_wasm_bindgen::from_value(paint) {
            Ok(mixin) => mixin,
            Err(_) => return JsValue::NULL,
        };

        // Spawn a new paint in the composition
        let entity = self.composition.spawn_bundle(paint, None);

        return serde_wasm_bindgen::to_value(&entity).unwrap_or(JsValue::NULL);
    }

    #[wasm_bindgen(js_name = spawnRectangleNode)]
    pub fn spawn_rectangle_node(&mut self, mixin: JsValue, maybe_parent_id: JsValue) -> JsValue {
        let mixin: RectangleNodeBundle = match serde_wasm_bindgen::from_value(mixin) {
            Ok(mixin) => mixin,
            Err(_) => return JsValue::NULL,
        };
        let maybe_parent_id = convert_optional_jsvalue::<Entity>(maybe_parent_id);

        // Spawn a new rectangle node in the composition
        let entity = self
            .composition
            .spawn_node_bundle(NodeBundle::Rectangle(mixin), maybe_parent_id);

        return serde_wasm_bindgen::to_value(&entity).unwrap_or(JsValue::NULL);
    }

    // =========================================================================
    // Debug
    // =========================================================================

    #[wasm_bindgen(js_name = logEntityComponents)]
    pub fn log_entity_components(&self, entity: JsValue) {
        #[cfg(feature = "tracing")]
        {
            let entity: Entity = match serde_wasm_bindgen::from_value(entity) {
                Ok(entity) => entity,
                Err(_) => return,
            };

            dyn_composition::core::modules::node::utils::logging::log_entity_components(
                &self.composition.get_app().world,
                entity,
            );
        }

        #[cfg(not(feature = "tracing"))]
        log::warn!("Log entity components not supported in this build");
    }

    // =========================================================================
    // Other
    // =========================================================================

    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> Option<String> {
        self.get_svg_composition()?.to_string()
    }

    // =========================================================================
    // Helper
    // =========================================================================

    fn get_svg_composition(&self) -> Option<&SVGCompositionRes> {
        let app = self.composition.get_app();
        let sub_app = app.get_sub_app(RenderApp).ok()?;
        return sub_app.world.get_resource::<SVGCompositionRes>();
    }
}
