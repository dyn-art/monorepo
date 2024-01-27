use std::{collections::HashMap, sync::mpsc::Sender};

use bevy_ecs::entity::Entity;
use dyn_composition::core::utils::continuous_id::ContinuousId;

use crate::{
    events::output_event::{ElementChangeEvent, SVGRenderOutputEvent},
    resources::changed_entities::{ChangedEntity, ChangedEntityType},
};

use super::{
    bundles::{
        frame_node_svg_bundle::FrameNodeSVGBundle, shape_node_svg_bundle::ShapeNodeSVGBundle,
        solid_paint_svg_bundle::SolidPaintSVGBundle,
    },
    svg_bundle::SVGBundle,
    svg_element::{SVGElement, SVGTag},
};

#[derive(Debug)]
pub struct SVGContext {
    root_bundle_ids: Vec<Entity>,
    bundles: HashMap<Entity, Box<dyn SVGBundle>>,
    changed_entities: Vec<ChangedEntity>,
    output_event_sender: Option<Sender<SVGRenderOutputEvent>>,
    pub id_generator: ContinuousId,
}

impl SVGContext {
    pub fn new(output_event_sender: Option<Sender<SVGRenderOutputEvent>>) -> Self {
        SVGContext {
            root_bundle_ids: Vec::new(),
            bundles: HashMap::new(),
            changed_entities: Vec::new(),
            output_event_sender,
            id_generator: ContinuousId::ZERO,
        }
    }

    // =========================================================================
    // Bundle
    // =========================================================================

    pub fn get_bundle(&self, entity: &Entity) -> Option<&Box<dyn SVGBundle>> {
        self.bundles.get(&entity)
    }

    pub fn get_bundle_mut(&mut self, entity: &Entity) -> Option<&mut Box<dyn SVGBundle>> {
        self.bundles.get_mut(&entity)
    }

    pub fn remove_bundle(&mut self, entity: &Entity) {
        // TODO
    }

    pub fn insert_bundle(
        &mut self,
        bundle: Box<dyn SVGBundle>,
        maybe_parent_id: Option<Entity>,
    ) -> () {
        let entity = bundle.get_entity().clone();
        if !self.bundles.contains_key(&entity) {
            if maybe_parent_id.is_none() {
                self.root_bundle_ids.push(*bundle.get_entity());
            }
            self.bundles.insert(bundle.get_entity().clone(), bundle);
        }
    }

    pub fn create_bundle(
        &mut self,
        entity: Entity,
        entity_type: ChangedEntityType,
    ) -> Option<Box<dyn SVGBundle>> {
        match entity_type {
            // Node
            ChangedEntityType::FrameNode => Some(Box::new(FrameNodeSVGBundle::new(entity, self))),
            ChangedEntityType::ShapeNode => Some(Box::new(ShapeNodeSVGBundle::new(entity, self))),
            // Paint
            ChangedEntityType::SolidPaint => Some(Box::new(SolidPaintSVGBundle::new(entity, self))),
            _ => None,
        }
    }

    // =========================================================================
    // Element
    // =========================================================================

    pub fn create_element(&mut self, tag: SVGTag) -> SVGElement {
        SVGElement::new(tag, self.id_generator.next_id())
    }

    pub fn create_bundle_root_element(&mut self, tag: SVGTag, entity: Entity) -> SVGElement {
        SVGElement::new_as_bundle_root(tag, entity, self.id_generator.next_id())
    }

    // =========================================================================
    // Changed Entity
    // =========================================================================

    pub fn add_changed_entity(&mut self, changed_entity: ChangedEntity) {
        self.changed_entities.push(changed_entity);
    }

    pub fn process_changed_entities(&mut self) {
        let changed_entities: Vec<ChangedEntity> = self.changed_entities.drain(..).collect();

        // TODO: Improve so its not necessary to remove element?
        for changed_entity in changed_entities {
            if let Some(mut bundle) = self.bundles.remove(&changed_entity.entity) {
                bundle.update(changed_entity, self);
                self.forward_element_change_events(bundle.drain_changes());
                self.bundles.insert(*bundle.get_entity(), bundle);
            }
        }
    }

    fn forward_element_change_events(&mut self, element_change_events: Vec<ElementChangeEvent>) {
        if let Some(output_event_sender) = &self.output_event_sender {
            for element_change_event in element_change_events {
                let _ = output_event_sender
                    .send(SVGRenderOutputEvent::ElementChange(element_change_event));
            }
        }
    }

    // =========================================================================
    // Other
    // =========================================================================

    pub fn to_string(&self) -> Option<String> {
        let mut svg_strings = Vec::new();

        // Construct SVG string
        for bundle_id in self.root_bundle_ids.iter() {
            let root = match self.get_bundle(bundle_id) {
                Some(root) => root,
                _ => continue,
            };

            let element = root.get_root_element();
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
}
