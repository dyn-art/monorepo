use std::{
    collections::HashMap,
    mem::transmute,
    sync::{
        mpsc::{channel, Receiver, Sender, TryRecvError},
        Arc, Mutex,
    },
};

use crate::{
    bindgen::js_bindings,
    core::composition::events::{
        CursorEnteredComposition, CursorExitedComposition, CursorMovedOnComposition, EntityMoved,
    },
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
    world_id: usize,
    sender: Sender<FromJsEvent>,
    receiver: Arc<Mutex<Receiver<FromJsEvent>>>,
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
        let (tx, rx) = channel();

        // Store the sender in the global map
        // so that it can be accessed from the enqueue_js_events function
        SENDER_MAP
            .lock()
            .unwrap()
            .insert(parsed_world_id, tx.clone());

        Self {
            sender: tx,
            receiver: Arc::new(Mutex::new(rx)),
            world_id: parsed_world_id,
        }
    }

    pub fn poll_events_from_js(&mut self) -> Vec<FromJsEvent> {
        let mut events = Vec::new();

        // Drain the receiver and push all events to the events vector
        loop {
            match self.receiver.lock().unwrap().try_recv() {
                Ok(event) => events.push(event),
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => break,
            }
        }

        return events;
    }
}

#[wasm_bindgen]
pub fn enqueue_js_events(world_id: usize, events: JsValue) {
    let parsed_events: Vec<FromJsEvent> = serde_wasm_bindgen::from_value(events).unwrap();
    info!("Received {:?} events", parsed_events);
    if let Some(sender) = SENDER_MAP.lock().unwrap().get(&world_id) {
        parsed_events
            .iter()
            .for_each(|event| sender.send(event.clone()).unwrap());
    } else {
        // TODO:
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
            info!("MoveEntity: {:?}, {:?}, {:?}", entity, dx, dy);
        }
    });
}
