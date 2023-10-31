use crossbeam_channel::{unbounded, Receiver, Sender};
use std::{collections::HashMap, mem::transmute, sync::Mutex};

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
    receiver: Receiver<FromJsEvent>,
    // Reuse this Vec to avoid reallocations
    events: Vec<FromJsEvent>,
}

impl FromWorld for FromJsEventQueue {
    fn from_world(world: &mut World) -> Self {
        return FromJsEventQueue::new(world.id());
    }
}

static SENDER_MAP: Lazy<Mutex<HashMap<usize, Sender<FromJsEvent>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

impl FromJsEventQueue {
    pub fn new(world_id: WorldId) -> Self {
        let parsed_world_id: usize = unsafe { transmute(world_id) };
        let (sender, receiver) = unbounded();

        // Store the sender in the global map
        // so that it can be accessed from the enqueue_js_events() function (called by JS)
        SENDER_MAP
            .lock()
            .unwrap()
            .insert(parsed_world_id, sender.clone());

        Self {
            receiver,
            events: Vec::new(),
        }
    }

    pub fn poll_events_from_js(&mut self) -> &Vec<FromJsEvent> {
        // Clear previous events
        self.events.clear();

        // Get events from receiver and push them into the events vec
        for event in self.receiver.try_iter() {
            self.events.push(event.clone());
        }

        return &self.events;
    }
}

#[wasm_bindgen]
pub fn enqueue_js_events(world_id: usize, events: JsValue) {
    let parsed_events: Vec<FromJsEvent> = serde_wasm_bindgen::from_value(events).unwrap();
    match SENDER_MAP.lock() {
        Ok(guard) => {
            if let Some(sender) = guard.get(&world_id) {
                for event in parsed_events.iter() {
                    sender.send(event.clone());
                }
            }
        }
        Err(_) => {
            // TODO: handle error
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
