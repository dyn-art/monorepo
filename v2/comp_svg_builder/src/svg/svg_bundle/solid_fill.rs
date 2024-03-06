use super::SvgBundle;
use crate::{
    resources::svg_context::SvgContextRes,
    svg::svg_element::{SvgElement, SvgElementId, SvgTag},
};
use bevy_ecs::entity::Entity;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct SolidFillSvgBundle {
    pub paint_entity: Entity,

    pub root_g: SvgElement,
    /**/ pub defs: SvgElement,
    /**/ pub shape_path: SvgElement,
}

impl SvgBundle for SolidFillSvgBundle {
    fn get_root_element(&self) -> &SvgElement {
        &self.root_g
    }

    fn get_root_element_mut(&mut self) -> &mut SvgElement {
        &mut self.root_g
    }

    fn get_elements(&self) -> BTreeMap<SvgElementId, &SvgElement> {
        let mut elements = BTreeMap::new();

        elements.insert(self.root_g.get_id(), &self.root_g);
        elements.insert(self.defs.get_id(), &self.defs);
        elements.insert(self.shape_path.get_id(), &self.shape_path);

        return elements;
    }

    fn get_elements_mut(&mut self) -> BTreeMap<SvgElementId, &mut SvgElement> {
        let mut elements = BTreeMap::new();

        elements.insert(self.root_g.get_id(), &mut self.root_g);
        elements.insert(self.defs.get_id(), &mut self.defs);
        elements.insert(self.shape_path.get_id(), &mut self.shape_path);

        return elements;
    }
}

impl SolidFillSvgBundle {
    pub fn new(entity: Entity, cx: &mut SvgContextRes) -> Self {
        log::info!("[SolidPaintSvgBundle::new] {:?}", entity);

        let mut root_g_element = cx.create_bundle_root_element(SvgTag::Group, entity);

        let mut defs_element = cx.create_element(SvgTag::Defs);
        root_g_element.append_child_in_bundle_context(entity, &mut defs_element);

        let mut shape_path_element = cx.create_element(SvgTag::Path);
        root_g_element.append_child_in_bundle_context(entity, &mut shape_path_element);

        #[cfg(feature = "tracing")]
        {
            use crate::svg::svg_element::attributes::SvgAttribute;

            root_g_element.set_attribute(SvgAttribute::Class {
                class: Self::create_element_name(root_g_element.get_id(), "root"),
            });
            defs_element.set_attribute(SvgAttribute::Class {
                class: Self::create_element_name(defs_element.get_id(), "defs"),
            });
            shape_path_element.set_attribute(SvgAttribute::Class {
                class: Self::create_element_name(shape_path_element.get_id(), "shape-path"),
            });
        }

        Self {
            paint_entity: entity,

            root_g: root_g_element,
            defs: defs_element,
            shape_path: shape_path_element,
        }
    }

    #[cfg(feature = "tracing")]
    fn create_element_name(id: SvgElementId, category: &str) -> String {
        format!("solid-paint_{}_{}", category, id)
    }
}
