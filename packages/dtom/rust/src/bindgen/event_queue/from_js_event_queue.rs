use std::{
    collections::HashMap,
    mem::transmute,
    sync::{Arc, RwLock},
};

use crate::core::composition::events::{
    CursorEnteredComposition, CursorExitedComposition, CursorMovedOnComposition, EntityMoved,
};

use bevy_ecs::{
    entity::Entity,
    event::EventWriter,
    system::{ResMut, Resource},
    world::{FromWorld, World, WorldId},
};
use glam::Vec2;
use log::info;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use specta::Type;
use wasm_bindgen::{prelude::*, JsValue};

#[derive(Debug, Serialize, Deserialize, Clone, Type)]
pub enum FromJsEvent {
    // Pointer Events
    PointerDownEventOnEntity { entity: Entity },
    PointerMovedOnComposition { position: Vec2 },
    PointerEnteredComposition,
    PointerExitedComposition,

    // Entity Events
    EntityMoved { entity: Entity, dx: f32, dy: f32 },
}

#[derive(Resource, Debug)]
pub struct FromJsEventQueue {
    queue: Arc<RwLock<Vec<FromJsEvent>>>,
    // Reuse this Vec to avoid reallocations
    events: Vec<FromJsEvent>,
}

impl FromWorld for FromJsEventQueue {
    fn from_world(world: &mut World) -> Self {
        return FromJsEventQueue::new(world.id());
    }
}

static SENDER_MAP: Lazy<RwLock<HashMap<usize, Arc<RwLock<Vec<FromJsEvent>>>>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

impl FromJsEventQueue {
    pub fn new(world_id: WorldId) -> Self {
        let parsed_world_id: usize = unsafe { transmute(world_id) };

        // Store the queue globally too
        // so that it can be accessed from the enqueue_js_events() function (called by JS)
        let maybe_shared_queue = {
            let read_lock = SENDER_MAP.read().unwrap();
            read_lock.get(&parsed_world_id).cloned()
        };
        let shared_queue = match maybe_shared_queue {
            Some(queue) => queue,
            None => {
                let new_queue = Arc::new(RwLock::new(Vec::new()));
                let mut write_lock = SENDER_MAP.write().unwrap();
                write_lock.insert(parsed_world_id, new_queue.clone());
                new_queue
            }
        };

        Self {
            queue: shared_queue,
            events: Vec::new(),
        }
    }

    pub fn poll_events_from_js(&mut self) -> &Vec<FromJsEvent> {
        // Clear previous events
        self.events.clear();

        // Drain events from queue and push them into the events vec
        self.queue
            .write()
            .unwrap()
            .drain(..)
            .into_iter()
            .for_each(|event| {
                self.events.push(event.clone());
            });

        return &self.events;
    }
}

#[wasm_bindgen]
pub fn enqueue_js_events(world_id: usize, events: JsValue) {
    let parsed_events: Vec<FromJsEvent> = serde_wasm_bindgen::from_value(events).unwrap();
    if let Some(shared_queue) = SENDER_MAP.read().unwrap().get(&world_id) {
        let mut queue = shared_queue.write().unwrap();
        for event in parsed_events.iter() {
            queue.push(event.clone());
        }
    }
}

pub fn poll_events_from_js(
    mut event_queue: ResMut<FromJsEventQueue>,

    // Cursor Events
    mut cursor_moved_events: EventWriter<CursorMovedOnComposition>,
    mut cursor_entered_events: EventWriter<CursorEnteredComposition>,
    mut cursor_exited_events: EventWriter<CursorExitedComposition>,

    // Entity Events
    mut entity_moved_events: EventWriter<EntityMoved>,
) {
    // Poll events from JS
    let events = event_queue.poll_events_from_js();

    // Map JS events to Bevy events
    events.iter().for_each(|event| match event {
        // Cursor Events
        FromJsEvent::PointerDownEventOnEntity { entity } => {
            // TODO
            info!("PointerDownEvent: {:?}", entity);
        }
        FromJsEvent::PointerMovedOnComposition { position } => {
            cursor_moved_events.send(CursorMovedOnComposition {
                position: *position,
            });
            info!("PointerMoveEvent: {:?}", position);
        }
        FromJsEvent::PointerEnteredComposition => {
            cursor_entered_events.send(CursorEnteredComposition);
            info!("PointerEnteredComposition");
        }
        FromJsEvent::PointerExitedComposition => {
            cursor_exited_events.send(CursorExitedComposition);
            info!("PointerExitedComposition");
        }

        // Entity Events
        FromJsEvent::EntityMoved { entity, dx, dy } => {
            entity_moved_events.send(EntityMoved {
                entity: *entity,
                dx: *dx,
                dy: *dy,
            });
        }
    });
}
