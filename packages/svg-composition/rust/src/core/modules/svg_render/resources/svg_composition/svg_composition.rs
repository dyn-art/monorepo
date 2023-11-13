use std::{collections::HashMap, sync::mpsc::Sender};

use bevy_ecs::{entity::Entity, system::Resource};
use dyn_composition::core::modules::node::components::types::NodeType;

use crate::core::{
    events::output_event::OutputEvent, modules::svg_render::mixin_change::MixinChange,
};

use super::svg_node::{FrameSVGNode, SVGNode, ShapeSVGNode};

#[derive(Resource, Debug)]
pub struct SVGComposition {
    // TODO: enum better? Is more performant but not so flexible
    // https://users.rust-lang.org/t/how-much-slower-is-a-dynamic-dispatch-really/98181/5
    // https://doc.rust-lang.org/book/ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types
    // All nodes of the SVGComposition
    nodes: HashMap<Entity, Box<dyn SVGNode>>,
    // Root entity
    root: Option<Entity>,
    // Map of updates from SVGElements
    updated: HashMap<u32, SVGNodeUpdate>,
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

    pub fn get_node(&self, entity: &Entity) -> Option<&Box<dyn SVGNode>> {
        self.nodes.get(&entity)
    }

    pub fn has_root_node(&self) -> bool {
        self.root.is_some()
    }

    pub fn get_root_node(&self) -> Option<&Box<dyn SVGNode>> {
        if let Some(root_entity) = self.root {
            return self.nodes.get(&root_entity);
        }
        return None;
    }

    pub fn get_or_insert_node(
        &mut self,
        entity: Entity,
        node_type: &NodeType,
    ) -> Option<&mut Box<dyn SVGNode>> {
        if !self.nodes.contains_key(&entity) {
            if let Some(node) = SVGComposition::create_node(node_type) {
                self.nodes.insert(entity, node);
            }
        }
        return self.nodes.get_mut(&entity);
    }

    fn create_node(node_type: &NodeType) -> Option<Box<dyn SVGNode>> {
        match node_type {
            NodeType::Rectangle => Some(Box::new(ShapeSVGNode::new())),
            NodeType::Frame => Some(Box::new(FrameSVGNode::new())),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        self.nodes.get(&self.root.unwrap()).unwrap().to_string(self)
    }
}
