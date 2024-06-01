mod bindgen;
pub mod events;
mod logging;
pub mod modules;

use crate::modules::watch::{
    component_change::{ComponentChange, ToComponentChange},
    resources::watched_entities::WatchableComponentVariant,
};
use bevy_app::App;
use bevy_ecs::{
    entity::Entity,
    query::{With, Without},
    system::{Query, SystemState},
};
use bevy_transform::components::{GlobalTransform, Transform};
use dyn_comp_bundles::{
    components::{marker::Root, mixins::SizeMixin},
    events::InputEvent,
};
use dyn_comp_core::{resources::composition::CompositionRes, CompCorePlugin};
use dyn_comp_dtif::{
    lua::script::{LuaScript, ToRunLuaScript, ToRunLuaScripts},
    DtifComposition,
};
use dyn_comp_interaction::CompInteractionPlugin;
use dyn_comp_svg_builder::{
    events::SvgBuilderOutputEvent, svg::svg_bundle::SvgBundleVariant, CompSvgBuilderPlugin,
};
use events::{SvgCompInputEvent, SvgCompOutputEvent};
use modules::watch::{resources::watched_entities::WatchedEntitiesRes, CompWatchPlugin};
use std::{
    collections::HashMap,
    sync::mpsc::{channel, Receiver},
};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
pub struct SvgCompHandle {
    app: App,
    svg_builder_output_event_receiver: Receiver<SvgBuilderOutputEvent>,
    output_event_receiver: Receiver<SvgCompOutputEvent>,
    lua_scripts: HashMap<String, LuaScript>,
}

#[wasm_bindgen]
impl SvgCompHandle {
    pub fn create(js_dtif: JsValue, interactive: bool) -> Result<SvgCompHandle, JsValue> {
        let mut dtif: DtifComposition = serde_wasm_bindgen::from_value(js_dtif)?;
        let mut app = App::new();

        let (svg_builder_output_event_sender, svg_builder_output_event_receiver) =
            channel::<SvgBuilderOutputEvent>();
        let (output_event_sender, output_event_receiver) = channel::<SvgCompOutputEvent>();

        // Register plugins
        app.add_plugins((
            CompCorePlugin {
                version: dtif.version,
                size: dtif.size,
                viewport: dtif.viewport,
            },
            CompWatchPlugin {
                output_event_sender,
                interactive,
            },
            CompSvgBuilderPlugin {
                output_event_sender: svg_builder_output_event_sender,
            },
        ));
        if interactive {
            app.add_plugins(CompInteractionPlugin);
        }

        dtif.send_into_world(&mut app.world);

        return Ok(Self {
            app,
            svg_builder_output_event_receiver,
            output_event_receiver,
            lua_scripts: dtif
                .scripts
                .drain(..)
                .map(|script_with_id| script_with_id.into_lua_script())
                .collect(),
        });
    }

    #[wasm_bindgen(js_name = runScripts)]
    pub fn run_scripts(&mut self, js_to_run_lua_scripts: JsValue) -> Result<JsValue, JsValue> {
        let maybe_to_run_lua_scripts: Result<ToRunLuaScripts, _> =
            serde_wasm_bindgen::from_value(js_to_run_lua_scripts);

        return match maybe_to_run_lua_scripts {
            Ok(to_run_lua_scripts) => Ok(serde_wasm_bindgen::to_value(
                &to_run_lua_scripts.find_and_run_batch(&self.lua_scripts, &mut self.app.world),
            )?),
            Err(_) => Err(JsValue::from_str("Failed to run scripts!")),
        };
    }

    #[wasm_bindgen(js_name = runScript)]
    pub fn run_script(&mut self, js_to_run_lua_script: JsValue) -> Result<JsValue, JsValue> {
        let maybe_to_run_lua_script: Result<ToRunLuaScript, _> =
            serde_wasm_bindgen::from_value(js_to_run_lua_script);

        return match maybe_to_run_lua_script {
            Ok(to_run_lua_script) => {
                if let Err(e) =
                    to_run_lua_script.find_and_run(&self.lua_scripts, &mut self.app.world)
                {
                    Ok(serde_wasm_bindgen::to_value(&e)?)
                } else {
                    Ok(JsValue::NULL)
                }
            }
            Err(_) => Err(JsValue::from_str("Failed to run script!")),
        };
    }

    pub fn update(&mut self, js_input_events: JsValue) -> Result<JsValue, JsValue> {
        let maybe_input_events: Result<Vec<SvgCompInputEvent>, _> =
            serde_wasm_bindgen::from_value(js_input_events);

        // Emit input events in ECS world
        if let Ok(input_events) = maybe_input_events {
            for input_event in input_events {
                match input_event {
                    SvgCompInputEvent::Core { event } => {
                        event.send_into_world(&mut self.app.world);
                    }
                    SvgCompInputEvent::Interaction { event } => {
                        event.send_into_world(&mut self.app.world);
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
                    output_events.push(SvgCompOutputEvent::SvgElementChange(event))
                }
                _ => {}
            }
        }

        return Ok(serde_wasm_bindgen::to_value(&output_events)?);
    }

    #[wasm_bindgen(js_name = watchEntity)]
    pub fn watch_entity(
        &mut self,
        js_entity: JsValue,
        js_to_watch_components: JsValue,
        initial_value: bool,
    ) -> Result<JsValue, JsValue> {
        let entity: Entity = serde_wasm_bindgen::from_value(js_entity)?;
        let to_watch_components: Vec<WatchableComponentVariant> =
            serde_wasm_bindgen::from_value(js_to_watch_components)?;

        // Collect intial values
        let mut changes: Vec<ComponentChange> = Vec::with_capacity(to_watch_components.len());
        if initial_value {
            for component_variant in &to_watch_components {
                match component_variant {
                    WatchableComponentVariant::Size => {
                        if let Some(component) = self.app.world.get::<SizeMixin>(entity) {
                            changes.push(component.to_component_change())
                        }
                    }
                    WatchableComponentVariant::Transform => {
                        if let Some(component) = self.app.world.get::<Transform>(entity) {
                            changes.push(component.to_component_change())
                        }
                    }
                    WatchableComponentVariant::GlobalTransform => {
                        if let Some(component) = self.app.world.get::<GlobalTransform>(entity) {
                            changes.push(component.to_component_change())
                        }
                    }
                }
            }
        }

        // Update watched entities ressource
        match self.app.world.get_resource_mut::<WatchedEntitiesRes>() {
            Some(mut watched_entities_res) => {
                watched_entities_res.watch_entity(entity, to_watch_components);
            }
            None => {
                return Err(JsValue::from_str(
                    "Failed to watch Entity because required resource couldn't be accessed!",
                ));
            }
        }

        return if initial_value {
            Ok(serde_wasm_bindgen::to_value(&changes)?)
        } else {
            Ok(JsValue::TRUE)
        };
    }

    #[wasm_bindgen(js_name = unregisterEntityCallback)]
    pub fn unregister_entity_callback(&mut self, js_entity: JsValue) -> Result<bool, JsValue> {
        let entity: Entity = serde_wasm_bindgen::from_value(js_entity)?;
        return match self.app.world.get_resource_mut::<WatchedEntitiesRes>() {
            Some(mut watched_entities_res) => Ok(watched_entities_res.unregister_entity(entity)),
            None => Ok(false),
        };
    }

    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&mut self) -> Option<String> {
        let mut result = String::new();
        let comp_res = self.app.world.get_resource::<CompositionRes>()?;

        // Open SVG tag
        result.push_str(&format!(
            "<svg width=\"{}\" height=\"{}\" xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\">",
            comp_res.size.width(),
            comp_res.size.height()
        ));

        let mut system_state: SystemState<(
            Query<&SvgBundleVariant, With<Root>>,
            Query<&SvgBundleVariant, Without<Root>>,
        )> = SystemState::new(&mut self.app.world);
        let (root_bundle_variant_query, bundle_variant_query) =
            system_state.get(&mut self.app.world);

        // Construct SVG string starting from root nodes
        for root_bundle_variant in root_bundle_variant_query.iter() {
            result.push_str(&root_bundle_variant.to_string(&bundle_variant_query))
        }

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
            "[log_entity_components_raw] Not supported in this build! Build with feature 'tracing'."
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
            "[log_entity_components] Not supported in this build! Build with feature 'tracing'."
        );
    }

    // TODO: Remove at some point
    #[wasm_bindgen(js_name = tempDevLog)]
    pub fn temp_dev_log(&mut self) {
        #[cfg(feature = "tracing")]
        {
            use crate::logging::log_entity_components;
            use bevy_ecs::query::{Or, With, Without};
            use dyn_comp_bundles::components::{
                marker::Root,
                nodes::{CompNode, FrameCompNode},
            };
            use dyn_comp_interaction::components::{Locked, Selected};

            let mut system_state: SystemState<Query<Entity>> =
                SystemState::new(&mut self.app.world);
            let query = system_state.get(&mut self.app.world);

            let entities: Vec<Entity> = query.iter().collect();

            for entity in entities.iter() {
                log_entity_components(&self.app.world, *entity);
            }
        }

        #[cfg(not(feature = "tracing"))]
        log::warn!("[temp_dev_log] Not supported in this build! Build with feature 'tracing'.");
    }
}
