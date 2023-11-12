use std::{collections::HashMap, sync::mpsc::Sender};

use bevy_ecs::{entity::Entity, system::Resource};

use crate::core::events::output_event::OutputEvent;

use super::svg_node::SVGNode;

#[derive(Resource, Debug)]
pub struct SVGComposition {
    // TODO: enum better? Is more performant but not so flexible
    // https://users.rust-lang.org/t/how-much-slower-is-a-dynamic-dispatch-really/98181/5
    // https://doc.rust-lang.org/book/ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types
    // All nodes of the SVGComposition
    pub nodes: HashMap<Entity, Box<dyn SVGNode>>,
    // Root entity
    pub root: Option<Entity>,
    // Map of updates from SVGElements
    pub updated: HashMap<u32, SVGNodeUpdate>,
    output_event_sender: Sender<OutputEvent>,
}

#[derive(Debug, Default)]
pub struct SVGNodeUpdate {
    pub changed_attributes: HashMap<String, String>,
    pub changed_styles: HashMap<String, String>,
}

impl SVGComposition {
    pub fn new(output_event_sender: Sender<OutputEvent>) -> Self {
        SVGComposition {
            root: None,
            nodes: HashMap::new(),
            updated: HashMap::new(),
            output_event_sender,
        }
    }

    pub fn to_string(&self) -> String {
        self.nodes.get(&self.root.unwrap()).unwrap().to_string(self)
    }
}
