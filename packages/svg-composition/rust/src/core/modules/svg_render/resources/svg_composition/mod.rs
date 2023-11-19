use std::{collections::HashMap, sync::mpsc::Sender};

use bevy_ecs::{entity::Entity, system::Resource};
use dyn_composition::core::modules::node::components::{mixins::Paint, types::NodeType};

use crate::core::events::output_event::{OutputEvent, RenderUpdateEvent};

use self::{
    svg_element::{InCompositionContextType, SVGChildElementIdentifier},
    svg_node::{frame_svg_node::FrameSVGNode, shape_svg_node::ShapeSVGNode, SVGNode},
    svg_paint::{solid_svg_paint::SolidSVGPaint, SVGPaint},
};

pub mod svg_bundle;
pub mod svg_element;
pub mod svg_node;
pub mod svg_paint;

#[derive(Resource, Debug)]
pub struct SVGComposition {
    // All nodes of the SVGComposition
    nodes: HashMap<Entity, Box<dyn SVGNode>>,
    // All paints of the SVGComposition
    paints: HashMap<Entity, Box<dyn SVGPaint>>,
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
            paints: HashMap::new(),
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

    pub fn get_paint(&self, entity: &Entity) -> Option<&Box<dyn SVGPaint>> {
        self.paints.get(&entity)
    }

    pub fn get_paint_mut(&mut self, entity: &Entity) -> Option<&mut Box<dyn SVGPaint>> {
        self.paints.get_mut(entity)
    }

    // =============================================================================
    // Paint Creation
    // =============================================================================

    pub fn get_or_create_paint(
        &mut self,
        entity: Entity,
        paint: &Paint,
    ) -> Option<&mut Box<dyn SVGPaint>> {
        if !self.paints.contains_key(&entity) {
            if let Some(new_paint) = self.create_paint(paint) {
                self.paints.insert(entity, new_paint);
            } else {
                return None;
            }
        }
        return self.paints.get_mut(&entity);
    }

    fn create_paint(&self, paint: &Paint) -> Option<Box<dyn SVGPaint>> {
        match paint {
            Paint::Solid(..) => Some(Box::new(SolidSVGPaint::new())),
        }
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
        if !self.nodes.contains_key(&entity) {
            if let Some(new_node) = self.create_node(node_type) {
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
        mut node: Box<dyn SVGNode>,
        maybe_parent_id: &Option<Entity>,
    ) {
        // If the parent id exists, append this node as a child
        if let Some(parent_id) = maybe_parent_id {
            if let Some(parent_node) = self.get_node_mut(parent_id) {
                let child_append_reference = parent_node.get_external_child_append_id().unwrap();
                let child_append_index = child_append_reference.index;
                let child_append_id = child_append_reference.id;

                if let Some(svg_element) = parent_node
                    .get_bundle_mut()
                    .get_child_element_at_mut(child_append_index)
                {
                    svg_element.append_child(SVGChildElementIdentifier::InCompositionContext(
                        InCompositionContextType::Node(entity),
                    ));
                    node.get_bundle_mut().append_to_parent(child_append_id);
                }
            }
        }
        // If there's no parent id, the node becomes a root node
        else {
            self.root_ids.push(entity);
        }

        self.nodes.insert(entity, node);
    }

    fn create_node(&self, node_type: &NodeType) -> Option<Box<dyn SVGNode>> {
        match node_type {
            NodeType::Rectangle => Some(Box::new(ShapeSVGNode::new())),
            NodeType::Frame => Some(Box::new(FrameSVGNode::new())),
            _ => None,
        }
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
