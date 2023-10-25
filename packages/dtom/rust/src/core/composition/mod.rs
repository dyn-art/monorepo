use std::collections::HashMap;
use std::mem::transmute;

use crate::bindgen::js_bindings;
use crate::core::composition::systems::{
    construct_path::construct_rectangle_path, startup_system_log, update_system_log,
};
use crate::plugins::bindgen_render_plugin::BindgenRenderPlugin;
use crate::plugins::render_plugin::RenderApp;
use crate::plugins::render_plugin::RenderPlugin;
use crate::{
    bindgen::{
        event_queue::{from_js_event_queue::FromJsEventQueue, to_js_event_queue::ToJsEventQueue},
        systems::{forward_events_to_js, poll_events_from_js},
        utils::set_panic_hook,
    },
    core::node::bundles::FrameNodeBundle,
};
use bevy_app::{App, Last, PostUpdate, PreUpdate, Startup, Update};
use bevy_ecs::world::World;
use bevy_ecs::{component::Component, entity::Entity};
use glam::Vec2;
use serde::{Deserialize, Serialize};
use specta::Type;
use wasm_bindgen::prelude::*;

use self::events::{CursorEnteredComposition, CursorExitedComposition, CursorMovedOnComposition};

use super::node::bundles::{GroupNodeBundle, RectangleNodeBundle};
use super::node::mixins::ChildrenMixin;

pub mod events;
mod systems;

#[derive(Serialize, Type)]
pub struct WorldIds {
    main_world_id: usize,
    render_world_id: usize,
}

// =============================================================================
// Composition App
// =============================================================================

#[wasm_bindgen]
pub struct CompositionApp {
    world_ids: WorldIds,
    app: App,
}

#[wasm_bindgen]
impl CompositionApp {
    #[wasm_bindgen(constructor)]
    pub fn new(dtif: JsValue) -> Self {
        set_panic_hook();

        let parsed_dtif: DTIFComposition = serde_wasm_bindgen::from_value(dtif).unwrap();
        js_bindings::log(format!("Parsed DTIF: {:?}", parsed_dtif).as_str());

        let mut app = App::new();

        // Register plugins
        app.add_plugins((RenderPlugin, BindgenRenderPlugin));

        // Register resources
        app.init_resource::<ToJsEventQueue>();
        app.init_resource::<FromJsEventQueue>();

        // Register systems
        app.add_systems(Startup, startup_system_log)
            .add_systems(PreUpdate, poll_events_from_js)
            .add_systems(Update, update_system_log)
            .add_systems(PostUpdate, construct_rectangle_path)
            .add_systems(Last, forward_events_to_js);

        // Register events
        app.add_event::<CursorEnteredComposition>();
        app.add_event::<CursorExitedComposition>();
        app.add_event::<CursorMovedOnComposition>();

        let root_node_id = parsed_dtif.root_node_id;
        let mut eid_to_entity_map: HashMap<String, Entity> = HashMap::new();

        // Spawn and process nodes recursively
        process_dtif_nodes(
            &mut app.world,
            &parsed_dtif.nodes,
            &root_node_id,
            &mut eid_to_entity_map,
        );

        // Spawn composition as entity (only one should exist).
        // Why entity? Because I see it as part of the "game" world,
        // and to spawn it with values passed from JS.
        let new_root_id = *eid_to_entity_map
            .get(&root_node_id)
            .expect("Root node not found in id_map");
        app.world.spawn(Composition {
            version: parsed_dtif.version,
            name: parsed_dtif.name,
            width: parsed_dtif.width,
            height: parsed_dtif.height,
            root_node: new_root_id,
        });
        app.world.spawn(CompositionInteractionMixin::default());

        Self {
            world_ids: WorldIds {
                main_world_id: CompositionApp::extract_main_world_id(&mut app),
                render_world_id: CompositionApp::extract_render_world_id(&mut app),
            },
            app,
        }
    }

    fn extract_main_world_id(app: &mut App) -> usize {
        let main_world_id = app.world.id();
        let parsed_main_world_id: usize = unsafe { transmute(main_world_id) };
        return parsed_main_world_id;
    }

    fn extract_render_world_id(app: &mut App) -> usize {
        let render_app = app.get_sub_app_mut(RenderApp).unwrap();
        let render_world_id = render_app.world.id();
        let parsed_render_world_id: usize = unsafe { transmute(render_world_id) };
        return parsed_render_world_id;
    }

    pub fn update(&mut self) {
        self.app.update();
    }

    pub fn get_world_ids(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.world_ids).unwrap()
    }
}

fn spawn_node(world: &mut World, node: &DTIFNode) -> Entity {
    match node {
        DTIFNode::Frame(bundle) => world.spawn(bundle.clone()).id(),
        DTIFNode::Rectangle(bundle) => world.spawn(bundle.clone()).id(),
        DTIFNode::Group(bundle) => world.spawn(bundle.clone()).id(),
    }
}

fn entity_to_id(entity: &Entity) -> String {
    format!("{}:{}", entity.generation(), entity.index())
}

fn process_dtif_nodes(
    world: &mut World,
    dtif_nodes: &HashMap<String, DTIFNode>,
    parent_id: &String,
    eid_to_entity: &mut HashMap<String, Entity>,
) {
    if let Some(node) = dtif_nodes.get(parent_id) {
        let new_entity = spawn_node(world, node);
        eid_to_entity.insert(parent_id.clone(), new_entity);

        // Recursive call for children
        let mut new_children: Vec<Entity> = vec![];
        if let DTIFNode::Frame(FrameNodeBundle { children_mixin, .. })
        | DTIFNode::Group(GroupNodeBundle { children_mixin, .. }) = node
        {
            for child in &children_mixin.children {
                let child_id = entity_to_id(child);
                process_dtif_nodes(world, dtif_nodes, &child_id, eid_to_entity);
                let new_child_id = *eid_to_entity
                    .get(&child_id)
                    .expect("Child node not found in id_map");
                new_children.push(new_child_id);
            }

            // Update parent with new children (override old ones)
            if !new_children.is_empty() {
                world.entity_mut(new_entity).insert(ChildrenMixin {
                    children: new_children,
                });
            }
        }
    }
}

// =============================================================================
// Composition Entity
// =============================================================================

#[derive(Component, Debug)]
pub struct Composition {
    version: String,
    name: String,
    width: f32,
    height: f32,
    root_node: Entity,
}

#[derive(Component, Debug, Default)]
pub struct CompositionInteractionMixin {
    interaction_mode: InteractionMode,
}

#[derive(Debug)]
pub enum InteractionMode {
    None,
    Translating { origin: Vec2, current: Vec2 },
    Pressing { origin: Vec2 },
}

impl Default for InteractionMode {
    fn default() -> Self {
        Self::None
    }
}

// =============================================================================
// DTIF
// =============================================================================

#[derive(Serialize, Deserialize, Debug, Type)]
pub struct DTIFComposition {
    version: String,
    name: String,
    width: f32,
    height: f32,
    root_node_id: String,
    nodes: HashMap<String, DTIFNode>,
}

#[derive(Serialize, Deserialize, Debug, Type)]
enum DTIFNode {
    Rectangle(RectangleNodeBundle),
    Frame(FrameNodeBundle),
    Group(GroupNodeBundle),
}
