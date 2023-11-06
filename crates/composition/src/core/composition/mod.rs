pub mod dtif;
pub mod events;
pub mod nodes;
mod systems;

use std::{collections::HashMap, mem::transmute};

use bevy_app::{App, Plugins, PostUpdate, Update};
use bevy_ecs::{bundle::Bundle, component::Component, entity::Entity, world::World};
use dyn_bevy_render_skeleton::{RenderApp, RenderPlugin};
use glam::Vec2;

use self::{
    dtif::{entity_to_eid, process_dtif_nodes, DTIFComposition},
    events::input_event::{
        CursorDownOnEntity, CursorEnteredComposition, CursorExitedComposition,
        CursorMovedOnComposition, EntityMoved, EntitySetPosition, InputEvent,
    },
    systems::{
        construct_path::construct_rectangle_path,
        layout::{handle_entity_moved_events, handle_entity_set_position_events},
    },
};

pub struct WorldIds {
    main_world_id: usize,
    render_world_id: usize,
}

pub struct Composition {
    world_ids: WorldIds,
    app: App,
}

impl Composition {
    pub fn new(dtif: DTIFComposition) -> Self {
        let mut app = App::new();

        // Register plugins
        app.add_plugins(RenderPlugin);

        // Register resources
        // TODO

        // Register systems
        app.add_systems(
            Update,
            (
                handle_entity_moved_events,
                handle_entity_set_position_events,
            ),
        )
        .add_systems(PostUpdate, construct_rectangle_path);

        // Register events
        app.add_event::<CursorMovedOnComposition>();
        app.add_event::<CursorEnteredComposition>();
        app.add_event::<CursorExitedComposition>();
        app.add_event::<CursorDownOnEntity>();
        app.add_event::<EntityMoved>();
        app.add_event::<EntitySetPosition>();

        insert_dtif(&mut app.world, dtif);

        return Self {
            world_ids: WorldIds {
                main_world_id: extract_main_world_id(&mut app),
                render_world_id: extract_render_world_id(&mut app),
            },
            app,
        };
    }

    pub fn get_world_ids(&self) -> &WorldIds {
        return &self.world_ids;
    }

    pub fn add_plugins<M>(&mut self, plugins: impl Plugins<M>) {
        self.app.add_plugins(plugins);
    }

    pub fn update(&mut self) {
        self.app.update();
    }

    pub fn spawn<B: Bundle>(&mut self, bundle: B) -> Entity {
        return self.app.world.spawn::<B>(bundle).id();
    }

    pub fn register_events(&mut self, events: Vec<InputEvent>) {
        for event in events {
            self.register_event(event);
        }
    }

    pub fn register_event(&mut self, event: InputEvent) {
        match event {
            // Cursor Events
            InputEvent::CursorMovedOnComposition(event) => {
                self.app.world.send_event(event);
            }
            InputEvent::CursorEnteredComposition(event) => {
                self.app.world.send_event(event);
            }
            InputEvent::CursorExitedComposition(event) => {
                self.app.world.send_event(event);
            }
            InputEvent::CursorDownOnEntity(event) => {
                self.app.world.send_event(event);
            }

            // Entity Events
            InputEvent::EntityMoved(event) => {
                self.app.world.send_event(event);
            }
            InputEvent::EntitySetPosition(event) => {
                self.app.world.send_event(event);
            }
        }
    }
}

fn insert_dtif(world: &mut World, dtif: DTIFComposition) {
    let root_node_eid = entity_to_eid(&dtif.root_node_id);
    let mut eid_to_entity_map: HashMap<String, Entity> = HashMap::new();

    // Spawn and process nodes recursively
    let root_node_entity =
        process_dtif_nodes(world, &dtif.nodes, &root_node_eid, &mut eid_to_entity_map).unwrap();

    // Spawn composition as entity (only one should exist).
    // Why entity? Because I see it as part of the "game" world,
    // and to spawn it with values passed from JS.
    world.spawn(CompositionMixin {
        version: dtif.version,
        name: dtif.name,
        width: dtif.width,
        height: dtif.height,
        root_node: root_node_entity,
    });
    world.spawn(CompositionInteractionMixin::default());
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

// TODO: where to put that:

#[derive(Component, Debug)]
pub struct CompositionMixin {
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
