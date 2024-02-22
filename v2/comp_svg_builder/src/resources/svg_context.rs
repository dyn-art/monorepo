use bevy_ecs::{entity::Entity, system::Resource};

use crate::svg::svg_element::{SvgElement, SvgElementId};

#[derive(Resource, Debug)]
pub struct SvgContextRes {
    id_generator: SvgElementId,
}

impl Default for SvgContextRes {
    fn default() -> Self {
        Self {
            id_generator: SvgElementId::ZERO,
        }
    }
}

impl SvgContextRes {
    pub fn create_element(&mut self, tag: &'static str) -> SvgElement {
        let mut svg_element = SvgElement::new(tag, self.id_generator.next_id());
        #[cfg(feature = "output_events")]
        svg_element.init(None);
        return svg_element;
    }

    pub fn create_bundle_root_element(&mut self, tag: &'static str, entity: Entity) -> SvgElement {
        let mut svg_element = SvgElement::new(tag, self.id_generator.next_id());
        #[cfg(feature = "output_events")]
        svg_element.init(Some(entity));
        return svg_element;
    }
}
