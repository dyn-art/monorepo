use super::FillSvgBundle;
use crate::{
    resources::svg_context::SvgContextRes,
    svg::{
        svg_bundle::SvgBundle,
        svg_element::{
            styles::{SvgPointerEventsStyle, SvgStyle},
            SvgElement, SvgElementId, SvgTag,
        },
    },
};
use bevy_ecs::entity::Entity;
use smallvec::SmallVec;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct ShapeNodeSvgBundle {
    pub node_entity: Entity,

    pub root_g: SvgElement,
    /**/ pub defs: SvgElement,
    /**/ pub click_area_rect: SvgElement,
    /**/ pub fills_wrapper_g: SvgElement,
    /**//**/ pub fill_bundles: SmallVec<[FillSvgBundle; 2]>,
    /**/ pub strokes_wrapper_g: SvgElement,
    /**//**/ pub stroke_bundles: SmallVec<[(); 2]>, // TODO
}

impl SvgBundle for ShapeNodeSvgBundle {
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
        elements.insert(self.click_area_rect.get_id(), &self.click_area_rect);
        elements.insert(self.fills_wrapper_g.get_id(), &self.fills_wrapper_g);
        self.fill_bundles.iter().for_each(|fill| {
            let elements_map = fill.get_svg_bundle().get_elements();
            for (_, element) in elements_map {
                elements.insert(element.get_id(), element);
            }
        });
        elements.insert(self.strokes_wrapper_g.get_id(), &self.strokes_wrapper_g);
        // TODO: stroke_bundles

        return elements;
    }

    fn get_elements_mut(&mut self) -> BTreeMap<SvgElementId, &mut SvgElement> {
        let mut elements = BTreeMap::new();

        elements.insert(self.root_g.get_id(), &mut self.root_g);
        elements.insert(self.defs.get_id(), &mut self.defs);
        elements.insert(self.click_area_rect.get_id(), &mut self.click_area_rect);
        elements.insert(self.fills_wrapper_g.get_id(), &mut self.fills_wrapper_g);
        self.fill_bundles.iter_mut().for_each(|fill| {
            let elements_map = fill.get_svg_bundle_mut().get_elements_mut();
            for (_, element) in elements_map {
                elements.insert(element.get_id(), element);
            }
        });
        elements.insert(self.strokes_wrapper_g.get_id(), &mut self.strokes_wrapper_g);
        // TODO: stroke_bundles

        return elements;
    }
}

impl ShapeNodeSvgBundle {
    pub fn new(entity: Entity, cx: &mut SvgContextRes) -> Self {
        log::info!("[ShapeNodeSvgBundle::new] {:?}", entity);

        let mut root_g_element = cx.create_bundle_root_element(SvgTag::Group, entity);

        let mut defs_element = cx.create_element(SvgTag::Defs);
        root_g_element.append_child_in_bundle_context(entity, &mut defs_element);

        let mut click_area_rect_element = cx.create_element(SvgTag::Rect);
        click_area_rect_element.set_style(SvgStyle::PointerEvents {
            pointer_events: SvgPointerEventsStyle::All,
        });
        root_g_element.append_child_in_bundle_context(entity, &mut click_area_rect_element);

        let mut fills_wrapper_g_element = cx.create_element(SvgTag::Group);
        root_g_element.append_child_in_bundle_context(entity, &mut fills_wrapper_g_element);

        let mut strokes_wrapper_g_element = cx.create_element(SvgTag::Group);
        root_g_element.append_child_in_bundle_context(entity, &mut strokes_wrapper_g_element);

        #[cfg(feature = "tracing")]
        {
            use crate::svg::svg_element::attributes::SvgAttribute;
            use crate::svg::svg_element::styles::SvgFillStyle;

            root_g_element.set_attribute(SvgAttribute::Class {
                class: Self::create_element_name(root_g_element.get_id(), "root"),
            });
            defs_element.set_attribute(SvgAttribute::Class {
                class: Self::create_element_name(defs_element.get_id(), "defs"),
            });
            click_area_rect_element.set_attribute(SvgAttribute::Class {
                class: Self::create_element_name(
                    click_area_rect_element.get_id(),
                    "click-area-rect",
                ),
            });
            click_area_rect_element.set_style(SvgStyle::Fill {
                fill: SvgFillStyle::RGBA {
                    red: 255,
                    green: 204,
                    blue: 203,
                    alpha: 0.5,
                },
            });
            fills_wrapper_g_element.set_attribute(SvgAttribute::Class {
                class: Self::create_element_name(fills_wrapper_g_element.get_id(), "fills"),
            });
            strokes_wrapper_g_element.set_attribute(SvgAttribute::Class {
                class: Self::create_element_name(strokes_wrapper_g_element.get_id(), "strokes"),
            });
        }

        Self {
            node_entity: entity,

            root_g: root_g_element,
            defs: defs_element,
            click_area_rect: click_area_rect_element,
            fills_wrapper_g: fills_wrapper_g_element,
            fill_bundles: SmallVec::new(),
            strokes_wrapper_g: strokes_wrapper_g_element,
            stroke_bundles: SmallVec::new(),
        }
    }

    #[cfg(feature = "tracing")]
    fn create_element_name(id: SvgElementId, category: &str) -> String {
        format!("shape-node_{}_{}", category, id)
    }
}
