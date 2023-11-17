use bevy_utils::HashMap;

use crate::core::{
    events::output_event::RenderUpdateEvent,
    modules::svg_render::resources::svg_composition::{
        svg_element::{
            attributes::SVGAttribute,
            events::{AttributeUpdated, ElementCreated, RenderChange, StyleUpdated},
            styles::SVGStyle,
            SVGChildElementIdentifier, SVGElement,
        },
        SVGComposition,
    },
};

use super::ElementReference;

/// Defines an ECS node in the SVG structure.
/// Consists of multiple SVGElements.
#[derive(Debug)]
pub struct BaseSVGNode {
    // Unique identifier of the SVGNode
    id: u32,
    // The primary SVG element associated with this node
    element: SVGElement,
    // Children that are directly related to this node's context.
    // Using a Vector for child_elements as:
    // - The size is known at compile time, minimizing dynamic changes.
    // - Offers efficient O(1) access by index, suitable for our use case.
    // - More memory-efficient and simpler than a HashMap for fixed-size collections.
    child_elements: Vec<SVGElement>,
    // Maps element ids to a list of render changes.
    // Group here by element id to avoid grouping or frequent lookups of elements on the JS site.
    updates: HashMap<u32, Vec<RenderChange>>,
    updates_order: Vec<u32>,
}

impl BaseSVGNode {
    pub fn new(element: SVGElement, maybe_parent_element_id: Option<&ElementReference>) -> Self {
        let element_id = element.get_id();
        let initial_updates = HashMap::from([(
            element_id,
            vec![RenderChange::ElementCreated(ElementCreated {
                parent_id: maybe_parent_element_id.map(|reference| reference.id),
                tag_name: element.get_tag_name().as_str(),
                attributes: element.get_attributes().clone(),
                styles: element.get_styles().clone(),
            })],
        )]);

        return BaseSVGNode {
            id: rand::random(),
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
            target_element.append_child(SVGChildElementIdentifier::InContext(next_index));
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
            .append_child(SVGChildElementIdentifier::InContext(next_index));
        return next_index;
    }

    #[inline]
    pub fn get_next_child_index(&self) -> usize {
        self.child_elements.len()
    }

    // =============================================================================
    // Other
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

    pub fn to_string(&self, composition: &SVGComposition) -> String {
        self.element.to_string(&self, composition)
    }

    // =============================================================================
    // Helper
    // =============================================================================

    fn register_element_creation_render_change(
        &mut self,
        element: &SVGElement,
        parent_id: Option<u32>,
    ) {
        self.register_render_change(
            element.get_id(),
            RenderChange::ElementCreated(ElementCreated {
                parent_id,
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
}
