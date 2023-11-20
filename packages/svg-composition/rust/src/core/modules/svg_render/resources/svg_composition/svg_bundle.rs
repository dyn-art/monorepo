use crate::core::events::output_event::RenderUpdateEvent;

use super::{
    svg_element::{
        attributes::SVGAttribute, styles::SVGStyle, SVGChildElementIdentifier, SVGElement,
    },
    svg_node::SVGNode,
    SVGComposition,
};

pub trait SVGBundle {
    fn get_bundle(&self) -> &BaseSVGBundle;
    fn get_bundle_mut(&mut self) -> &mut BaseSVGBundle;
}

// Wrapped SVGElement with its static children (are known from compile time) for quick access
#[derive(Debug)]
pub struct BaseSVGBundle {
    // The primary SVG element associated with this bundle
    element: SVGElement,
    // Children that are directly related to this bundles's context
    // whose order doesn't change.
    // Using a Vector for child_elements as:
    // - The size is known at compile time, minimizing dynamic changes
    // - Offers efficient O(1) access by index, suitable for this use case
    // - More memory-efficient and simpler than a HashMap for fixed-size collections
    child_elements: Vec<SVGElement>,
}

impl BaseSVGBundle {
    pub fn new(element: SVGElement) -> Self {
        Self {
            element,
            child_elements: Vec::new(),
        }
    }

    // =============================================================================
    // Getter & Setter
    // =============================================================================

    pub fn get_children(&self) -> &Vec<SVGElement> {
        &self.child_elements
    }

    pub fn get_element(&self) -> &SVGElement {
        &self.element
    }

    pub fn get_element_mut(&mut self) -> &mut SVGElement {
        &mut self.element
    }

    pub fn get_child_element_at_mut(&mut self, index: usize) -> Option<&mut SVGElement> {
        self.child_elements.get_mut(index)
    }

    // =============================================================================
    // Attributes
    // =============================================================================

    pub fn set_attributes_at(&mut self, index: usize, attributes: Vec<SVGAttribute>) {
        if let Some(element) = self.get_child_element_at_mut(index) {
            for attribute in attributes {
                element.set_attribute(attribute);
            }
        }
    }

    pub fn set_attributes(&mut self, attributes: Vec<SVGAttribute>) {
        for attribute in attributes {
            self.element.set_attribute(attribute);
        }
    }

    // =============================================================================
    // Styles
    // =============================================================================

    pub fn set_styles_at(&mut self, index: usize, styles: Vec<SVGStyle>) {
        if let Some(element) = self.get_child_element_at_mut(index) {
            for style in styles {
                element.set_style(style);
            }
        }
    }

    pub fn set_styles(&mut self, styles: Vec<SVGStyle>) {
        for style in styles {
            self.element.set_style(style);
        }
    }

    // =============================================================================
    // Children
    // =============================================================================

    pub fn append_child_element_to(
        &mut self,
        index: usize,
        mut element: SVGElement,
    ) -> Option<usize> {
        let next_index = self.get_next_child_index();
        if let Some(target_element) = self.child_elements.get_mut(index) {
            let target_element_id = target_element.get_id();
            target_element.append_child(
                &mut element,
                SVGChildElementIdentifier::InBundleContext(next_index),
            );
            self.child_elements.push(element);
            return Some(next_index);
        }
        return None;
    }

    pub fn append_child_element(&mut self, mut element: SVGElement) -> usize {
        let next_index = self.get_next_child_index();
        self.element.append_child(
            &mut element,
            SVGChildElementIdentifier::InBundleContext(next_index),
        );
        self.child_elements.push(element);
        return next_index;
    }

    #[inline]
    pub fn get_next_child_index(&self) -> usize {
        self.child_elements.len()
    }

    // =============================================================================
    // Render Updates
    // =============================================================================

    pub fn drain_updates(&mut self) -> Vec<RenderUpdateEvent> {
        let mut drained_updates = Vec::new();

        // Drain updates from root
        drained_updates.push(RenderUpdateEvent {
            id: self.element.get_id(),
            updates: self.element.drain_updates(),
        });

        // Drain updates from children
        for child in &mut self.child_elements {
            let updates = child.drain_updates();
            if !updates.is_empty() {
                drained_updates.push(RenderUpdateEvent {
                    id: child.get_id(),
                    updates: updates,
                })
            }
        }

        return drained_updates;
    }

    // =============================================================================
    // Other
    // =============================================================================

    pub fn to_string(&self, node: &dyn SVGNode, composition: &SVGComposition) -> String {
        self.element.to_string(self, node, composition)
    }
}
