use std::collections::HashMap;

use crate::core::events::output_event::RenderUpdateEvent;

use super::{
    svg_element::{
        attributes::SVGAttribute,
        events::{AttributeUpdated, ElementCreated, RenderChange, StyleUpdated},
        styles::SVGStyle,
        SVGChildElementIdentifier, SVGElement,
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
    // Maps element ids to a list of render changes.
    // Group here by element id to avoid grouping or frequent lookups of elements on the JS site.
    updates: HashMap<u32, Vec<RenderChange>>,
    updates_order: Vec<u32>,
}

impl BaseSVGBundle {
    pub fn new(element: SVGElement) -> Self {
        let element_id = element.get_id();
        let initial_updates = HashMap::from([(
            element_id,
            vec![RenderChange::ElementCreated(ElementCreated {
                parent_id: None, // Set in append_to_parent() if required
                tag_name: element.get_tag_name().as_str(),
                attributes: element.get_attributes().clone(),
                styles: element.get_styles().clone(),
            })],
        )]);

        return Self {
            element,
            child_elements: vec![],
            updates: initial_updates,
            updates_order: vec![element_id],
        };
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
            let mut render_changes: Vec<RenderChange> = vec![];
            let element_id = element.get_id();

            for attribute in attributes {
                // Record the attribute update as a render change
                render_changes.push(RenderChange::AttributeUpdated(AttributeUpdated {
                    new_value: attribute.clone(),
                }));

                element.set_attribute(attribute);
            }

            for render_change in render_changes {
                self.register_render_change(element_id, render_change);
            }
        }
    }

    pub fn set_attributes(&mut self, attributes: Vec<SVGAttribute>) {
        for attribute in attributes {
            // Record the attribute update as a render change
            self.register_render_change(
                self.element.get_id(),
                RenderChange::AttributeUpdated(AttributeUpdated {
                    new_value: attribute.clone(),
                }),
            );

            self.element.set_attribute(attribute);
        }
    }

    // =============================================================================
    // Styles
    // =============================================================================

    pub fn set_styles_at(&mut self, index: usize, styles: Vec<SVGStyle>) {
        if let Some(element) = self.get_child_element_at_mut(index) {
            let mut render_changes: Vec<RenderChange> = vec![];
            let element_id = element.get_id();

            for style in styles {
                // Record the style update as a render change
                render_changes.push(RenderChange::StyleUpdated(StyleUpdated {
                    new_value: style.clone(),
                }));

                element.set_style(style);
            }

            for render_change in render_changes {
                self.register_render_change(element_id, render_change);
            }
        }
    }

    pub fn set_styles(&mut self, styles: Vec<SVGStyle>) {
        for style in styles {
            // Record the style update as a render change
            self.register_render_change(
                self.element.get_id(),
                RenderChange::StyleUpdated(StyleUpdated {
                    new_value: style.clone(),
                }),
            );

            self.element.set_style(style);
        }
    }

    // =============================================================================
    // Children
    // =============================================================================

    pub fn append_child_element_to(
        &mut self,
        index: usize,
        element: SVGElement,
    ) -> Result<usize, String> {
        let next_index = self.get_next_child_index();
        if let Some(target_element) = self.child_elements.get_mut(index) {
            let target_element_id = target_element.get_id();
            target_element.append_child(SVGChildElementIdentifier::InBundleContext(next_index));
            self.register_element_creation_render_change(&element, Some(target_element_id));
            self.child_elements.push(element);
            return Ok(next_index);
        } else {
            return Err(String::from("Invalid parent index"));
        }
    }

    pub fn append_child_element(&mut self, element: SVGElement) -> usize {
        let next_index = self.get_next_child_index();
        self.register_element_creation_render_change(&element, Some(self.element.get_id()));
        self.child_elements.push(element);
        self.element
            .append_child(SVGChildElementIdentifier::InBundleContext(next_index)); // TODO: in bundle context
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

        for id in self.updates_order.drain(..) {
            if let Some(updates) = self.updates.remove(&id) {
                drained_updates.push(RenderUpdateEvent { id, updates });
            }
        }

        return drained_updates;
    }

    fn register_element_creation_render_change(
        &mut self,
        element: &SVGElement,
        maybe_parent_id: Option<u32>,
    ) {
        self.register_render_change(
            element.get_id(),
            RenderChange::ElementCreated(ElementCreated {
                parent_id: maybe_parent_id,
                tag_name: element.get_tag_name().as_str(),
                attributes: element.get_attributes(),
                styles: element.get_styles().clone(),
            }),
        );
    }

    fn register_render_change(&mut self, id: u32, change: RenderChange) {
        if self.updates.entry(id).or_insert_with(Vec::new).is_empty() {
            self.updates_order.push(id);
        }
        self.updates.get_mut(&id).unwrap().push(change);
    }

    // =============================================================================
    // Other
    // =============================================================================

    pub fn append_to_parent(&mut self, parent_id: u32) {
        let element_id = self.element.get_id();
        let mut updated = false;

        // Attempt to set the parent id of the first 'ElementCreated' render change for the element.
        // This ensures the element is correctly attached to its parent during the initial rendering.
        if let Some(updates) = self.updates.get_mut(&element_id) {
            if let Some(update) = updates.first_mut() {
                match update {
                    RenderChange::ElementCreated(element_created) => {
                        element_created.parent_id = Some(parent_id);
                        updated = true;
                    }
                    _ => {}
                }
            }
        }

        if !updated {
            // TODO Append element to parent after intial render
        }
    }

    pub fn to_string(&self, node: &dyn SVGNode, composition: &SVGComposition) -> String {
        self.element.to_string(self, node, composition)
    }
}
