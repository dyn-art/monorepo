use std::sync::mpsc::Sender;

use bevy_ecs::system::Resource;
use serde::Serialize;
use specta::Type;

use crate::core::modules::svg_render::resources::svg_composition::svg_element::events::RenderChange;

/// OutputEvent Enum
/// This enum represents the different types of events that can be emitted by SVGElement
/// to synchronize its state with the frontend.
///
/// Note on Child Element Management:
/// - Child elements are managed implicitly through their own lifecycle events rather than
///   explicit child addition or removal events.
/// - When a child element is created (`ElementCreated`), it includes an optional `parent_id`
///   indicating its parent. This way, the frontend knows to append this new child element
///   to the specified parent element.
/// - When a child element is deleted (`ElementDeleted`), it is responsible for removing itself
///   from the DOM. The parent element implicitly recognizes this removal.
/// - This approach avoids the need for separate `ChildAdded` or `ChildRemoved` events, simplifying
///   the event model and reducing the number of events needed to manage the DOM structure.
#[derive(Debug, Serialize, Clone, Type)]
#[serde(tag = "type")]
pub enum OutputEvent {
    RenderUpdate(RenderUpdateEvent),
}

#[derive(Debug, Serialize, Clone, Type)]
pub struct RenderUpdateEvent {
    pub id: u32,
    pub updates: Vec<RenderChange>,
}

#[derive(Resource, Debug)]
pub struct OutputEventQueue {
    output_event_sender: Sender<OutputEvent>,
}

impl OutputEventQueue {
    pub fn new(output_event_sender: Sender<OutputEvent>) -> Self {
        Self {
            output_event_sender,
        }
    }

    pub fn push_event(&mut self, event: OutputEvent) {
        match self.output_event_sender.send(event) {
            Ok(_) => {}
            Err(e) => {
                // TODO
            }
        }
    }
}
