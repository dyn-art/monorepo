use std::{collections::HashMap, sync::mpsc::Sender};

use bevy_ecs::system::Resource;
use serde::Serialize;
use serde_with::serde_as;
use specta::Type;

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

#[derive(Debug, Serialize, Clone, Type)]
pub enum RenderChange {
    ElementCreated(ElementCreated),
    ElementDeleted(ElementDeleted),
    AttributeUpdated(AttributeUpdated),
    StyleUpdated(StyleUpdated),
    ElementUpdated(ElementUpdated),
}

/// Emitted when a new SVGElement is created
#[serde_as]
#[derive(Debug, Serialize, Clone, Type)]
pub struct ElementCreated {
    #[serde(rename = "tagName")]
    pub tag_name: String,
    #[serde_as(as = "Vec<(_, _)>")]
    pub attributes: HashMap<String, String>,
    #[serde_as(as = "Vec<(_, _)>")]
    pub styles: HashMap<String, String>,
    #[serde(rename = "parentId")]
    pub parent_id: Option<u32>, // Optional parent ID, if it's a child element
}

/// Emitted when an SVGElement is deleted
#[derive(Debug, Serialize, Clone, Type)]
pub struct ElementDeleted;

/// Emitted when an attribute of an SVGElement is updated
#[derive(Debug, Serialize, Clone, Type)]
pub struct AttributeUpdated {
    pub name: String,
    #[serde(rename = "newValue")]
    pub new_value: Option<String>, // None indicates removal of the attribute
}

/// Emitted when a style property of an SVGElement is updated
#[derive(Debug, Serialize, Clone, Type)]
pub struct StyleUpdated {
    pub name: String,
    #[serde(rename = "newValue")]
    pub new_value: Option<String>, // None indicates removal of the style
}

/// Emitted for bulk updates to an SVGElement
#[derive(Debug, Serialize, Clone, Type)]
pub struct ElementUpdated {
    #[serde(rename = "updatedAttributes")]
    pub updated_attributes: HashMap<String, String>,
    #[serde(rename = "updatedStyles")]
    pub updated_styles: HashMap<String, String>,
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
