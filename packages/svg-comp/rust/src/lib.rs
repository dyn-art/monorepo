mod bindgen;
pub mod events;
mod logging;
pub mod modules;

use bevy_app::App;
use bevy_ecs::{
    query::{With, Without},
    system::{Query, SystemState},
    world::World,
};
use dyn_comp_core::{resources::composition::CompositionRes, CompCorePlugin};
use dyn_comp_dtif::CompDtif;
use dyn_comp_interaction::CompInteractionPlugin;
use dyn_comp_svg_builder::{
    events::SvgBuilderOutputEvent,
    svg::svg_bundle::{NodeSvgBundle, NodeSvgBundleMixin},
    CompSvgBuilderPlugin,
};
use dyn_comp_types::{events::InputEvent, mixins::Root};
use events::{SvgCompInputEvent, SvgCompOutputEvent};
use modules::watch::CompWatchPlugin;
use std::sync::mpsc::{channel, Receiver};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
pub struct SvgCompHandle {
    app: App,
    svg_builder_output_event_receiver: Receiver<SvgBuilderOutputEvent>,
    output_event_receiver: Receiver<SvgCompOutputEvent>,
}

#[wasm_bindgen]
impl SvgCompHandle {
    pub fn create(js_dtif: JsValue, interactive: bool) -> Result<SvgCompHandle, JsValue> {
        let dtif: CompDtif = serde_wasm_bindgen::from_value(js_dtif)?;
        let mut app = App::new();

        let (svg_builder_output_event_sender, svg_builder_output_event_receiver) =
            channel::<SvgBuilderOutputEvent>();
        let (output_event_sender, output_event_receiver) = channel::<SvgCompOutputEvent>();

        // Register plugins
        app.add_plugins((
            CompCorePlugin { dtif },
            CompWatchPlugin {
                output_event_sender,
            },
            CompSvgBuilderPlugin {
                output_event_sender: svg_builder_output_event_sender,
            },
        ));
        if interactive {
            app.add_plugins(CompInteractionPlugin);
        }

        return Ok(Self {
            app,
            svg_builder_output_event_receiver,
            output_event_receiver,
        });
    }

    pub fn update(&mut self, js_input_events: JsValue) -> Result<JsValue, JsValue> {
        let maybe_input_events: Result<Vec<SvgCompInputEvent>, _> =
            serde_wasm_bindgen::from_value(js_input_events);

        // Emit input events in ECS world
        if let Ok(input_events) = maybe_input_events {
            for input_event in input_events {
                match input_event {
                    SvgCompInputEvent::Comp { event } => {
                        event.send_into_ecs(&mut self.app.world);
                    }
                    SvgCompInputEvent::Interaction { event } => {
                        event.send_into_ecs(&mut self.app.world);
                    }
                }
            }
        }

        // Advance the execution of the schedule by one cycle
        self.app.update();

        // Collect output events that were emitted during the last update cycle
        let mut output_events = Vec::new();
        while let Ok(event) = self.output_event_receiver.try_recv() {
            output_events.push(event);
        }
        while let Ok(event) = self.svg_builder_output_event_receiver.try_recv() {
            match event {
                SvgBuilderOutputEvent::SvgElementChanges(event) => {
                    output_events.push(SvgCompOutputEvent::SvgElementChanges(event))
                }
                _ => {}
            }
        }

        return Ok(serde_wasm_bindgen::to_value(&output_events)?);
    }

    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&mut self) -> Option<String> {
        let mut result = String::new();
        let comp_res = self.app.world.get_resource::<CompositionRes>()?;

        // Open SVG tag
        result.push_str(&format!(
            "<svg width=\"{}\" height=\"{}\" xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\">",
            comp_res.size.0.x, comp_res.size.0.y
        ));

        let mut system_state: SystemState<(
            Query<&NodeSvgBundleMixin, With<Root>>,
            Query<&NodeSvgBundleMixin, Without<Root>>,
        )> = SystemState::new(&mut self.app.world);
        let (root_node_query, node_query) = system_state.get(&mut self.app.world);

        // Construct SVG string starting from root nodes
        root_node_query
            .iter()
            .for_each(|bundle| result.push_str(&bundle.0.to_string(&node_query)));

        // Close the SVG tag
        result.push_str("</svg>");

        return Some(result);
    }

    #[wasm_bindgen(js_name = logEntityComponentsRaw)]
    pub fn log_entity_components_raw(&self, js_entity: JsValue) {
        #[cfg(feature = "tracing")]
        {
            use crate::logging::log_entity_components;
            use bevy_ecs::entity::Entity;

            let entity_raw: u32 = match serde_wasm_bindgen::from_value(js_entity) {
                Ok(raw) => raw,
                Err(e) => {
                    log::warn!(
                        "[log_entity_components_raw] Failed to parse u32 from JsValue: {:?}",
                        e
                    );
                    return;
                }
            };
            let entity: Entity = Entity::from_raw(entity_raw);

            log_entity_components(&self.app.world, entity);
        }

        #[cfg(not(feature = "tracing"))]
        log::warn!(
            "[log_entity_components_raw] Log entity components not supported in this build! Build with feature 'tracing'."
        );
    }

    #[wasm_bindgen(js_name = logEntityComponents)]
    pub fn log_entity_components(&self, js_entity: JsValue) {
        #[cfg(feature = "tracing")]
        {
            use crate::logging::log_entity_components;
            use bevy_ecs::entity::Entity;

            let entity: Entity = match serde_wasm_bindgen::from_value(js_entity) {
                Ok(entity) => entity,
                Err(e) => {
                    log::warn!(
                        "[log_entity_components] Failed to parse Entity from JsValue: {:?}",
                        e
                    );
                    return;
                }
            };

            log_entity_components(&self.app.world, entity);
        }

        #[cfg(not(feature = "tracing"))]
        log::warn!(
            "[log_entity_components] Log entity components not supported in this build! Build with feature 'tracing'."
        );
    }
}
