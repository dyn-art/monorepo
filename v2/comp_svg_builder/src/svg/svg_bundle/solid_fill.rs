use std::collections::BTreeMap;

use bevy_ecs::entity::Entity;

use crate::{
    resources::svg_context::SvgContextRes,
    svg::svg_element::{attributes::SvgAttribute, SvgElement, SvgElementId, SvgTag},
};

use super::SvgBundle;

#[derive(Debug, Clone)]
pub struct SolidFillSvgBundle {
    pub paint_entity: Entity,

    pub root: SvgElement,
    pub paint_rect: SvgElement,
}

impl SvgBundle for SolidFillSvgBundle {
    fn get_root_element(&self) -> &SvgElement {
        &self.root
    }

    fn get_root_element_mut(&mut self) -> &mut SvgElement {
        &mut self.root
    }

    fn get_child_elements(&self) -> BTreeMap<SvgElementId, &SvgElement> {
        let mut children = BTreeMap::new();

        children.insert(self.paint_rect.get_id(), &self.paint_rect);

        return children;
    }

    fn get_child_elements_mut(&mut self) -> BTreeMap<SvgElementId, &mut SvgElement> {
        let mut children = BTreeMap::new();

        children.insert(self.paint_rect.get_id(), &mut self.paint_rect);

        return children;
    }
}

impl SolidFillSvgBundle {
    pub fn new(entity: Entity, cx: &mut SvgContextRes) -> Self {
        log::info!("[SolidPaintSvgBundle::new] {:?}", entity);

        let mut root_element = cx.create_bundle_root_element(SvgTag::Group, entity);
        #[cfg(feature = "tracing")]
        root_element.set_attribute(SvgAttribute::Name {
            name: Self::create_element_name(root_element.get_id(), "root", false),
        });

        let mut paint_rect_element = cx.create_element(SvgTag::Rect);
        #[cfg(feature = "tracing")]
        paint_rect_element.set_attribute(SvgAttribute::Name {
            name: Self::create_element_name(paint_rect_element.get_id(), "paint-rect", false),
        });
        root_element.append_child_in_bundle_context(entity, &mut paint_rect_element);

        Self {
            paint_entity: entity,

            root: root_element,
            paint_rect: paint_rect_element,
        }
    }

    #[cfg(feature = "tracing")]
    fn create_element_name(id: SvgElementId, category: &str, is_definition: bool) -> String {
        let def_part = if is_definition { "_def" } else { "" };
        format!("solid-paint_{}_{}{}", category, id, def_part)
    }
}
