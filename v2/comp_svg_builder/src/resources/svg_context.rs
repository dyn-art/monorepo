use bevy_ecs::{entity::Entity, system::Resource};

use crate::svg::svg_element::{SVGElement, SVGElementId};

#[derive(Resource, Debug)]
pub struct SVGContextRes {
    id_generator: SVGElementId,
}

impl Default for SVGContextRes {
    fn default() -> Self {
        Self {
            id_generator: SVGElementId::ZERO,
        }
    }
}

impl SVGContextRes {
    pub fn create_element(&mut self, tag: &'static str) -> SVGElement {
        let mut svg_element = SVGElement::new(tag, self.id_generator.next_id());
        #[cfg(feature = "output_events")]
        svg_element.init(None);
        return svg_element;
    }

    pub fn create_bundle_root_element(&mut self, tag: &'static str, entity: Entity) -> SVGElement {
        let mut svg_element = SVGElement::new(tag, self.id_generator.next_id());
        #[cfg(feature = "output_events")]
        svg_element.init(Some(entity));
        return svg_element;
    }
}
