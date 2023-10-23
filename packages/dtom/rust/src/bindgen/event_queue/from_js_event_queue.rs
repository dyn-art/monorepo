use std::{
    collections::HashMap,
    mem::transmute,
    sync::{
        mpsc::{channel, Receiver, Sender, TryRecvError},
        Arc, Mutex,
    },
};

use bevy_ecs::{
    system::Resource,
    world::{FromWorld, World, WorldId},
};
use glam::Vec2;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use wasm_bindgen::{prelude::*, JsValue};

#[cfg_attr(feature = "cli", derive(Type))]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FromJsEvent {
    PointerDownEventOnEntity { entity: u32 },
    PointerMovedOnCanvas { position: Vec2 },
    PointerEnteredCanvas,
    PointerExitedCanvas,
    // ..
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
    if let Some(sender) = SENDER_MAP.lock().unwrap().get(&world_id) {
        parsed_events
            .iter()
            .for_each(|event| sender.send(event.clone()).unwrap());
    } else {
        // TODO:
    }
}
