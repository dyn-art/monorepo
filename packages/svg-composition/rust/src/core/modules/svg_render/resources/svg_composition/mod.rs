use std::{collections::HashMap, sync::mpsc::Sender};

use bevy_ecs::{entity::Entity, system::Resource};
use dyn_composition::core::modules::node::components::types::NodeType;

use crate::core::events::output_event::{OutputEvent, RenderUpdateEvent};

use self::{
    svg_element::{InCompositionContextType, SVGChildElementIdentifier},
    svg_node::{frame_svg_node::FrameSVGNode, shape_svg_node::ShapeSVGNode, SVGNode},
};

pub mod svg_bundle;
pub mod svg_element;
pub mod svg_fill;
pub mod svg_node;

#[derive(Resource, Debug)]
pub struct SVGComposition {
    // All nodes of the SVGComposition
    nodes: HashMap<Entity, Box<dyn SVGNode>>,
    // Root entities
    root_ids: Vec<Entity>,
    // Sender to enque events for frontend
    output_event_sender: Sender<OutputEvent>,
}

impl SVGComposition {
    pub fn new(output_event_sender: Sender<OutputEvent>) -> Self {
        SVGComposition {
            root_ids: vec![],
            nodes: HashMap::new(),
            output_event_sender,
        }
    }

    // =============================================================================
    // Getter & Setter
    // =============================================================================

    pub fn get_node(&self, entity: &Entity) -> Option<&Box<dyn SVGNode>> {
        self.nodes.get(&entity)
    }

    pub fn get_node_mut(&mut self, entity: &Entity) -> Option<&mut Box<dyn SVGNode>> {
        self.nodes.get_mut(entity)
    }

    // =============================================================================
    // Node Creation
    // =============================================================================

    pub fn get_or_create_node(
        &mut self,
        entity: Entity,
        node_type: &NodeType,
        maybe_parent_id: &Option<Entity>,
    ) -> Option<&mut Box<dyn SVGNode>> {
        // Create & insert a new node if it does not exist yet
        if !self.nodes.contains_key(&entity) {
            if let Some(new_node) = self.create_node(node_type, maybe_parent_id) {
                self.insert_node(entity, new_node, maybe_parent_id);
            } else {
                return None;
            }
        }

        return self.nodes.get_mut(&entity);
    }

    pub fn insert_node(
        &mut self,
        entity: Entity,
        node: Box<dyn SVGNode>,
        maybe_parent_id: &Option<Entity>,
    ) {
        // Insert the new node.
        self.nodes.insert(entity, node);

        match maybe_parent_id {
            // If the parent id exists, append this node as a child
            Some(parent_id) => {
                if let Some(parent_node) = self.get_node_mut(parent_id) {
                    let child_append_index =
                        parent_node.get_external_child_append_id().unwrap().index;
                    if let Some(svg_element) = parent_node
                        .get_bundle_mut()
                        .get_child_element_at_mut(child_append_index)
                    {
                        svg_element.append_child(SVGChildElementIdentifier::InCompositionContext(
                            InCompositionContextType::Node(entity),
                        ));
                    }
                }
            }

            // If there's no parent id, the node becomes a root node
            None => self.root_ids.push(entity),
        }
    }

    fn create_node(
        &self,
        node_type: &NodeType,
        maybe_parent_id: &Option<Entity>,
    ) -> Option<Box<dyn SVGNode>> {
        let maybe_parent_element_id = maybe_parent_id
            .and_then(|parent_id| self.get_node(&parent_id))
            .and_then(|parent| {
                parent
                    .get_external_child_append_id()
                    .map(|child_append_id| child_append_id.id)
            });

        return match node_type {
            NodeType::Rectangle => Some(Box::new(ShapeSVGNode::new(maybe_parent_element_id))),
            NodeType::Frame => Some(Box::new(FrameSVGNode::new(maybe_parent_element_id))),
            _ => None,
        };
    }

    // =============================================================================
    // Other
    // =============================================================================

    pub fn forward_node_updates(&mut self, updates: Vec<RenderUpdateEvent>) {
        for update in updates {
            let _ = self
                .output_event_sender
                .send(OutputEvent::RenderUpdate(update));
        }
    }

    pub fn to_string(&self) -> Option<String> {
        let mut svg_strings = Vec::new();

        // Construct SVG string
        for id in self.root_ids.iter() {
            if let Some(root) = self.get_node(id) {
                let element = root.get_bundle().get_element();
                let mut result = String::new();

                // Open the SVG tag
                result.push_str(&format!(
                    "<svg width=\"{}\" height=\"{}\" xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\">",
                    element.get_attribute("width")?.to_svg_string(),
                    element.get_attribute("height")?.to_svg_string()
                ));

                // Append the content from the root node
                result.push_str(&root.to_string(self));

                // Close the SVG tag
                result.push_str("</svg>");

                svg_strings.push(result);
            }
        }

        return if svg_strings.is_empty() {
            None
        } else {
            Some(svg_strings.join("\n"))
        };
    }
}
