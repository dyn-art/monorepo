use std::{collections::HashMap, sync::mpsc::Sender};

use bevy_ecs::{entity::Entity, system::Resource};
use dyn_composition::core::modules::node::components::{mixins::Paint, types::NodeType};

use crate::core::events::output_event::{OutputEvent, RenderUpdateEvent};

use self::{
    svg_bundle_variant::{get_bundle_mut, SVGBundleVariant},
    svg_element::SVGChildElementIdentifier,
    svg_node::{frame_svg_node::FrameSVGNode, shape_svg_node::ShapeSVGNode, SVGNode},
    svg_paint::{solid_svg_paint::SolidSVGPaint, SVGPaint},
};

pub mod svg_bundle;
pub mod svg_bundle_variant;
pub mod svg_element;
pub mod svg_node;
pub mod svg_paint;

#[derive(Resource, Debug)]
pub struct SVGCompositionRes {
    // All bundles of the SVGComposition
    bundles: HashMap<Entity, SVGBundleVariant>,
    // Root entities
    root_ids: Vec<Entity>,
    // Sender to enque events for frontend
    output_event_sender: Sender<OutputEvent>,
}

impl SVGCompositionRes {
    pub fn new(output_event_sender: Sender<OutputEvent>) -> Self {
        SVGCompositionRes {
            root_ids: Vec::new(),
            bundles: HashMap::new(),
            output_event_sender,
        }
    }

    // =========================================================================
    // Getter & Setter
    // =========================================================================

    pub fn get_bundle(&self, entity: &Entity) -> Option<&SVGBundleVariant> {
        self.bundles.get(&entity)
    }

    pub fn get_bundle_mut(&mut self, entity: &Entity) -> Option<&mut SVGBundleVariant> {
        self.bundles.get_mut(&entity)
    }

    pub fn get_node(&self, entity: &Entity) -> Option<&Box<dyn SVGNode>> {
        match self.bundles.get(&entity) {
            Some(SVGBundleVariant::Node(node)) => Some(node),
            _ => None,
        }
    }

    pub fn get_node_mut(&mut self, entity: &Entity) -> Option<&mut Box<dyn SVGNode>> {
        match self.bundles.get_mut(&entity) {
            Some(SVGBundleVariant::Node(node)) => Some(node),
            _ => None,
        }
    }

    pub fn get_paint(&self, entity: &Entity) -> Option<&Box<dyn SVGPaint>> {
        match self.bundles.get(&entity) {
            Some(SVGBundleVariant::Paint(paint)) => Some(paint),
            _ => None,
        }
    }

    pub fn get_paint_mut(&mut self, entity: &Entity) -> Option<&mut Box<dyn SVGPaint>> {
        match self.bundles.get_mut(&entity) {
            Some(SVGBundleVariant::Paint(paint)) => Some(paint),
            _ => None,
        }
    }

    // =========================================================================
    // Paint
    // =========================================================================

    pub fn get_or_create_paint(
        &mut self,
        entity: Entity,
        paint: &Paint,
        maybe_parent_id: &Option<Entity>,
    ) -> Option<&mut Box<dyn SVGPaint>> {
        // Create paint
        if !self.bundles.contains_key(&entity) {
            match self.create_paint(paint, entity.clone()) {
                Some(new_paint) => {
                    self.insert_bundle(entity, SVGBundleVariant::Paint(new_paint), maybe_parent_id);
                }
                _ => return None,
            }
        }

        return match self.bundles.get_mut(&entity) {
            Some(SVGBundleVariant::Paint(paint)) => Some(paint),
            _ => None,
        };
    }

    fn create_paint(&self, paint: &Paint, entity: Entity) -> Option<Box<dyn SVGPaint>> {
        match paint {
            Paint::Solid(..) => Some(Box::new(SolidSVGPaint::new(entity))),
        }
    }

    // =========================================================================
    // Node
    // =========================================================================

    pub fn get_or_create_node(
        &mut self,
        entity: Entity,
        node_type: &NodeType,
        maybe_parent_id: &Option<Entity>,
    ) -> Option<&mut Box<dyn SVGNode>> {
        // Create node
        if !self.bundles.contains_key(&entity) {
            match self.create_node(node_type, entity.clone()) {
                Some(new_node) => {
                    self.insert_bundle(entity, SVGBundleVariant::Node(new_node), maybe_parent_id);
                }
                _ => return None,
            }
        }

        return match self.bundles.get_mut(&entity) {
            Some(SVGBundleVariant::Node(node)) => Some(node),
            _ => None,
        };
    }

    fn create_node(&self, node_type: &NodeType, entity: Entity) -> Option<Box<dyn SVGNode>> {
        match node_type {
            NodeType::Rectangle => Some(Box::new(ShapeSVGNode::new(entity))),
            NodeType::Frame => Some(Box::new(FrameSVGNode::new(entity))),
            NodeType::Text => Some(Box::new(ShapeSVGNode::new(entity))),
            _ => None,
        }
    }

    // =========================================================================
    // Other
    // =========================================================================

    pub fn forward_render_updates(&mut self, updates: Vec<RenderUpdateEvent>) {
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
            let root = match self.get_node(id) {
                Some(root) => root,
                _ => continue,
            };

            let element = root.get_bundle().get_root();
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

        return if svg_strings.is_empty() {
            None
        } else {
            Some(svg_strings.join("\n"))
        };
    }

    // =========================================================================
    // Helper
    // =========================================================================

    fn insert_bundle(
        &mut self,
        entity: Entity,
        mut bundle_variant: SVGBundleVariant,
        maybe_parent_id: &Option<Entity>,
    ) {
        // If the parent id exists, append this bundle element as a child to the parent element
        if let Some(parent_id) = maybe_parent_id {
            let parent_node = match self.get_node_mut(parent_id) {
                Some(parent_node) => parent_node,
                None => return,
            };

            // Find child append index
            let maybe_child_append_id = match &bundle_variant {
                SVGBundleVariant::Node(_) => parent_node.get_node_append_id(),
                SVGBundleVariant::Paint(_) => parent_node.get_paint_append_id(),
            };
            let child_append_index = match maybe_child_append_id {
                Some(child_append_id) => child_append_id.index,
                None => return,
            };

            // Append child
            if let Some(parent_append_element) = parent_node
                .get_bundle_mut()
                .get_child_mut(child_append_index)
            {
                parent_append_element.apply_child(
                    get_bundle_mut(&mut bundle_variant).get_root_mut(),
                    SVGChildElementIdentifier::InCompositionContext(entity),
                );
            }
        }
        // If there's no parent id, the node becomes a root node
        else {
            self.root_ids.push(entity);
        }

        self.bundles.insert(entity, bundle_variant);
    }
}
