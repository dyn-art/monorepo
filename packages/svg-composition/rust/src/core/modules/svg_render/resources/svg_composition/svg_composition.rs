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
    // Sender to enque events for frontend
    output_event_sender: Sender<OutputEvent>,
}

impl SVGComposition {
    pub fn new(output_event_sender: Sender<OutputEvent>) -> Self {
        SVGComposition {
            root: None,
            nodes: HashMap::new(),
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
        maybe_parent_id: &Option<Entity>,
    ) -> Option<&mut Box<dyn SVGNode>> {
        if !self.nodes.contains_key(&entity) {
            let maybe_node = self.create_node(node_type, maybe_parent_id);
            if let Some(node) = maybe_node {
                self.insert_node(entity, node, maybe_parent_id);
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

    fn create_node(
        &mut self,
        node_type: &NodeType,
        maybe_parent_id: &Option<Entity>,
    ) -> Option<Box<dyn SVGNode>> {
        let maybe_parent_element_id = maybe_parent_id
            .and_then(|parent_id| self.get_node_mut(&parent_id))
            .and_then(|parent| parent.get_external_child_append_id());
        match node_type {
            NodeType::Rectangle => Some(Box::new(ShapeSVGNode::new(maybe_parent_element_id))),
            NodeType::Frame => Some(Box::new(FrameSVGNode::new(maybe_parent_element_id))),
            _ => None,
        }
    }

    pub fn forward_node_updates(&mut self, entity: &Entity) {
        let maybe_node = self.nodes.get_mut(entity);
        if let Some(node) = maybe_node {
            let changes = node.get_base_mut().drain_updates();
            for change in changes {
                self.output_event_sender
                    .send(OutputEvent::RenderUpdate(change))
                    .expect("Failed to send RenderChange event");
            }
        }
    }

    pub fn to_string(&self) -> String {
        if let Some(root) = self.get_root_node() {
            let element = root.get_base().get_element();
            let mut result = format!(
                "<svg width=\"{}\" height=\"{}\" xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\">",
                element.get_attribute("width").unwrap().to_svg_string(), element.get_attribute("height").unwrap().to_svg_string() 
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
