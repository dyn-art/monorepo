use bevy_ecs::entity::Entity;

use crate::events::output_event::ElementChangeEvent;

use super::{
    svg_element::{SVGChildElementIdentifier, SVGElement},
    SVGCompositionRes,
};

pub trait SVGBundle {
    fn get_bundle(&self) -> &BaseSVGBundle;
    fn get_bundle_mut(&mut self) -> &mut BaseSVGBundle;
    fn drain_changes(&mut self) -> Vec<ElementChangeEvent>;
    fn to_string(&self, composition: &SVGCompositionRes) -> String;
}

/// Wrapped SVGElement with static children (known from compile time) for quick access.
#[derive(Debug)]
pub struct BaseSVGBundle {
    entity: Entity,
    // The primary SVG element associated with this bundle
    element: SVGElement,
    // Children that are directly related to this bundles's context.
    // The Vector is fixed and the items order shouldn't change after initialization
    // because SVGElements point to SVGElements in this Vector who is the owner!
    // Using a Vector for child_elements as:
    // - The size is known at compile time, minimizing dynamic changes
    // - Offers efficient O(1) access by index, suitable for this use case
    // - More memory-efficient and simpler than a HashMap for fixed-size collections
    children: Vec<SVGBundleChildElement>, // SmallVec<[SVGBundleChildElement; 6]>,
}

#[derive(Debug)]
pub enum SVGBundleChildElement {
    // Fixed SVGElement
    Element(SVGElement),
    // Dynamic Vector of SVGElement whose content can change without side effects.
    // But these SVGElement have to be end branches of the SVG Tree
    // and can't have dynamic children.
    Portal(Vec<SVGElement>),
}

impl BaseSVGBundle {
    pub fn new(mut element: SVGElement, entity: Entity) -> Self {
        element.define_as_bundle_root(entity);
        Self {
            entity,
            element,
            children: Vec::new(),
        }
    }

    // =========================================================================
    // Getter & Setter
    // =========================================================================

    pub fn get_children(&self) -> &Vec<SVGBundleChildElement> {
        &self.children
    }

    pub fn get_root(&self) -> &SVGElement {
        &self.element
    }

    pub fn get_root_mut(&mut self) -> &mut SVGElement {
        &mut self.element
    }

    pub fn get_child_element(&self, index: usize) -> Option<&SVGElement> {
        let maybe_child = self.children.get(index);
        if let Some(child) = maybe_child {
            match child {
                SVGBundleChildElement::Element(element) => {
                    return Some(element);
                }
                _ => {}
            }
        }
        return None;
    }

    pub fn get_child_element_mut(&mut self, index: usize) -> Option<&mut SVGElement> {
        let maybe_child = self.children.get_mut(index);
        if let Some(child) = maybe_child {
            match child {
                SVGBundleChildElement::Element(element) => {
                    return Some(element);
                }
                _ => {}
            }
        }
        return None;
    }

    pub fn get_child_portal_mut(&mut self, index: usize) -> Option<&mut Vec<SVGElement>> {
        let maybe_child = self.children.get_mut(index);
        if let Some(child) = maybe_child {
            match child {
                SVGBundleChildElement::Portal(elements) => {
                    return Some(elements);
                }
                _ => {}
            }
        }
        return None;
    }

    // =========================================================================
    // Children
    // =========================================================================

    pub fn append_child_element_to(
        &mut self,
        index: usize,
        mut element: SVGElement,
    ) -> Option<usize> {
        let next_index = self.get_next_child_index();
        if let Some(target_element) = self.children.get_mut(index) {
            if let SVGBundleChildElement::Element(target_element) = target_element {
                target_element.append_child_element(
                    &mut element,
                    SVGChildElementIdentifier::InBundleContext(self.entity, next_index),
                );
                self.children.push(SVGBundleChildElement::Element(element));
                return Some(next_index);
            }
        }
        return None;
    }

    pub fn append_child_portal_to(&mut self, index: usize) -> Option<usize> {
        let next_index = self.get_next_child_index();
        if let Some(target_element) = self.children.get_mut(index) {
            if let SVGBundleChildElement::Element(target_element) = target_element {
                target_element.append_child_portal(
                    &mut Vec::new(),
                    SVGChildElementIdentifier::InBundleContext(self.entity, next_index),
                );
                self.children
                    .push(SVGBundleChildElement::Portal(Vec::new()));
                return Some(next_index);
            }
        }
        return None;
    }

    pub fn append_child_element(&mut self, mut element: SVGElement) -> usize {
        let next_index = self.get_next_child_index();
        self.element.append_child_element(
            &mut element,
            SVGChildElementIdentifier::InBundleContext(self.entity, next_index),
        );
        self.children.push(SVGBundleChildElement::Element(element));
        return next_index;
    }

    #[inline]
    pub fn get_next_child_index(&self) -> usize {
        self.children.len()
    }

    // =========================================================================
    // Other
    // =========================================================================

    pub fn drain_changes(&mut self) -> Vec<ElementChangeEvent> {
        let mut drained_updates: Vec<ElementChangeEvent> = Vec::new();

        // Drain updates from root element
        drained_updates.push(ElementChangeEvent {
            id: self.element.get_id(),
            changes: self.element.drain_changes(),
        });

        // Drain updates from child elements
        for child in &mut self.children {
            match child {
                SVGBundleChildElement::Element(element) => {
                    BaseSVGBundle::drain_svg_element_changes(&mut drained_updates, element);
                }
                SVGBundleChildElement::Portal(elements) => {
                    for element in elements {
                        BaseSVGBundle::drain_svg_element_changes(&mut drained_updates, element);
                    }
                }
            }
        }

        return drained_updates;
    }

    fn drain_svg_element_changes(
        drained_updates: &mut Vec<ElementChangeEvent>,
        element: &mut SVGElement,
    ) {
        let changes = element.drain_changes();
        if !changes.is_empty() {
            drained_updates.push(ElementChangeEvent {
                id: element.get_id(),
                changes,
            })
        }
    }

    pub fn to_string(&self, composition: &SVGCompositionRes) -> String {
        self.element.to_string(self, composition)
    }
}
