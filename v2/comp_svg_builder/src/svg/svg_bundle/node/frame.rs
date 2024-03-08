use crate::{
    resources::svg_context::SvgContextRes,
    svg::{
        svg_bundle::SvgBundle,
        svg_element::{
            attributes::SvgAttribute,
            styles::{SvgPointerEventsStyle, SvgStyle},
            SvgElement, SvgElementId, SvgTag,
        },
    },
};
use bevy_ecs::entity::Entity;
use smallvec::SmallVec;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct FrameNodeSvgBundle {
    pub node_entity: Entity,

    pub root_g: SvgElement,
    /**/ pub defs: SvgElement,
    /**//**/ pub children_clip_path: SvgElement,
    /**//**//**/ pub children_clipped_path: SvgElement,
    /**/ pub click_area_rect: SvgElement,
    /**/ pub styles_wrapper_g: SvgElement,
    /**//**/ pub styles: SmallVec<[Entity; 2]>,
    /**/ pub children_wrapper_g: SvgElement,
    /**//**/ pub child_nodes: SmallVec<[Entity; 2]>,
}

impl SvgBundle for FrameNodeSvgBundle {
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
        elements.insert(self.children_clip_path.get_id(), &self.children_clip_path);
        elements.insert(
            self.children_clipped_path.get_id(),
            &self.children_clipped_path,
        );
        elements.insert(self.click_area_rect.get_id(), &self.click_area_rect);
        elements.insert(self.styles_wrapper_g.get_id(), &self.styles_wrapper_g);
        elements.insert(self.children_wrapper_g.get_id(), &self.children_wrapper_g);

        return elements;
    }

    fn get_elements_mut(&mut self) -> BTreeMap<SvgElementId, &mut SvgElement> {
        let mut elements = BTreeMap::new();

        elements.insert(self.root_g.get_id(), &mut self.root_g);
        elements.insert(self.defs.get_id(), &mut self.defs);
        elements.insert(
            self.children_clip_path.get_id(),
            &mut self.children_clip_path,
        );
        elements.insert(
            self.children_clipped_path.get_id(),
            &mut self.children_clipped_path,
        );
        elements.insert(self.click_area_rect.get_id(), &mut self.click_area_rect);
        elements.insert(self.styles_wrapper_g.get_id(), &mut self.styles_wrapper_g);
        elements.insert(
            self.children_wrapper_g.get_id(),
            &mut self.children_wrapper_g,
        );

        return elements;
    }
}

impl FrameNodeSvgBundle {
    pub fn new(entity: Entity, cx: &mut SvgContextRes) -> Self {
        log::info!("[FrameNodeSvgBundle::new] {:?}", entity);

        let mut root_g_element = cx.create_bundle_root_element(SvgTag::Group, entity);

        let mut defs_element = cx.create_element(SvgTag::Defs);
        root_g_element.append_child_in_bundle_context(&mut defs_element);

        let mut children_clip_path_element = cx.create_element(SvgTag::ClipPath);
        defs_element.append_child_in_bundle_context(&mut children_clip_path_element);

        let mut children_clipped_path_element = cx.create_element(SvgTag::Path);
        children_clip_path_element
            .append_child_in_bundle_context(&mut children_clipped_path_element);

        let mut click_area_rect_element = cx.create_element(SvgTag::Rect);
        click_area_rect_element.set_style(SvgStyle::PointerEvents {
            pointer_events: SvgPointerEventsStyle::All,
        });
        root_g_element.append_child_in_bundle_context(&mut click_area_rect_element);

        let mut styles_wrapper_g_element = cx.create_element(SvgTag::Group);
        root_g_element.append_child_in_bundle_context(&mut styles_wrapper_g_element);

        let mut children_wrapper_g_element = cx.create_element(SvgTag::Group);
        children_wrapper_g_element.set_attribute(SvgAttribute::ClipPath {
            clip_path: children_clip_path_element.get_id(),
        });
        root_g_element.append_child_in_bundle_context(&mut children_wrapper_g_element);

        #[cfg(feature = "tracing")]
        {
            use crate::svg::svg_element::styles::SvgFillStyle;

            root_g_element.set_attribute(SvgAttribute::Class {
                class: Self::create_element_name(root_g_element.get_id(), "root"),
            });
            defs_element.set_attribute(SvgAttribute::Class {
                class: Self::create_element_name(defs_element.get_id(), "defs"),
            });
            children_clip_path_element.set_attribute(SvgAttribute::Class {
                class: Self::create_element_name(
                    children_clip_path_element.get_id(),
                    "children-clip-path",
                ),
            });
            children_clipped_path_element.set_attribute(SvgAttribute::Class {
                class: Self::create_element_name(
                    children_clipped_path_element.get_id(),
                    "children-clipped-path",
                ),
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
            styles_wrapper_g_element.set_attribute(SvgAttribute::Class {
                class: Self::create_element_name(styles_wrapper_g_element.get_id(), "styles"),
            });
            children_wrapper_g_element.set_attribute(SvgAttribute::Class {
                class: Self::create_element_name(children_wrapper_g_element.get_id(), "children"),
            });
        }

        Self {
            node_entity: entity,

            root_g: root_g_element,
            defs: defs_element,
            children_clip_path: children_clip_path_element,
            children_clipped_path: children_clipped_path_element,
            click_area_rect: click_area_rect_element,
            styles_wrapper_g: styles_wrapper_g_element,
            styles: SmallVec::new(),
            children_wrapper_g: children_wrapper_g_element,
            child_nodes: SmallVec::new(),
        }
    }

    #[cfg(feature = "tracing")]
    fn create_element_name(id: SvgElementId, category: &str) -> String {
        format!("frame-node_{}_{}", category, id)
    }
}
