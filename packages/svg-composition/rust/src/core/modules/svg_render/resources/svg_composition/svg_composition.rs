use std::{collections::HashMap, sync::mpsc::Sender};

use bevy_ecs::{entity::Entity, system::Resource};
use dyn_composition::core::modules::node::components::types::NodeType;

use crate::core::events::output_event::OutputEvent;

use super::svg_node::{frame_svg_node::FrameSVGNode, shape_svg_node::ShapeSVGNode, SVGNode};

#[derive(Resource, Debug)]
pub struct SVGComposition {
    // All nodes of the SVGComposition
    nodes: HashMap<Entity, Box<dyn SVGNode>>,
    // Root entity
    root: Option<Entity>,
    // Map of updates from SVGElements
    changes: HashMap<u32, SVGElementChange>,
    // Sender to enque events for frontend
    output_event_sender: Sender<OutputEvent>,
}

// TODO: Improve
#[derive(Debug, Default)]
pub struct SVGElementChange {
    pub changed_attributes: HashMap<String, String>,
    pub changed_styles: HashMap<String, String>,
}

impl SVGComposition {
    pub fn new(output_event_sender: Sender<OutputEvent>) -> Self {
        SVGComposition {
            root: None,
            nodes: HashMap::new(),
            changes: HashMap::new(),
            output_event_sender,
        }
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

    pub fn get_node(&self, entity: &Entity) -> Option<&Box<dyn SVGNode>> {
        self.nodes.get(&entity)
    }

    pub fn get_node_mut(&mut self, entity: &Entity) -> Option<&mut Box<dyn SVGNode>> {
        self.nodes.get_mut(entity)
    }

    pub fn get_or_insert_node(
        &mut self,
        entity: Entity,
        node_type: &NodeType,
        parent_id: &Option<Entity>,
    ) -> Option<&mut Box<dyn SVGNode>> {
        if !self.nodes.contains_key(&entity) {
            if let Some(node) = SVGComposition::create_node(node_type) {
                self.insert_node(entity, node, parent_id);
            }
        }
        return self.get_node_mut(&entity);
    }

    pub fn insert_node(
        &mut self,
        entity: Entity,
        node: Box<dyn SVGNode>,
        maybe_parent_id: &Option<Entity>,
    ) {
        self.nodes.insert(entity, node);

        // First inserted node without known parent will become root node
        if !self.has_root_node() && maybe_parent_id.is_none() {
            self.root = Some(entity);
        }

        // Append child node to parent node
        if let Some(parent_id) = maybe_parent_id {
            if let Some(node) = self.get_node_mut(parent_id) {
                node.append_external_child(entity);
            }
        }
    }

    fn create_node(node_type: &NodeType) -> Option<Box<dyn SVGNode>> {
        match node_type {
            NodeType::Rectangle => Some(Box::new(ShapeSVGNode::new())),
            NodeType::Frame => Some(Box::new(FrameSVGNode::new())),
            _ => None,
        }
    }

    fn enqueue_changes(&mut self) -> () {
        // TODO:
    }

    pub fn to_string(&self) -> String {
        if let Some(root) = self.get_root_node() {
            let element = root.get_base().get_element();
            let mut result = format!(
                "<svg width=\"{}\" height=\"{}\" xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\">",
                element.get_attribute("width").unwrap(), element.get_attribute("height").unwrap() 
            );

            // Append the content from the root node
            result.push_str(&root.to_string(self));

            // Close the SVG tag
            result.push_str("</svg>");

            return result;
        } else {
            String::from("undefined")
        }
    }
}
