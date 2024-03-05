use super::FillSvgBundle;
use crate::{
    resources::svg_context::SvgContextRes,
    svg::{
        svg_bundle::SvgBundle,
        svg_element::{
            attributes::{SvgAttribute, SvgPointerEventsVariants},
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
    /**/ pub fills_wrapper_g: SvgElement,
    /**//**/ pub fill_bundles: SmallVec<[FillSvgBundle; 2]>,
    /**/ pub strokes_wrapper_g: SvgElement,
    /**//**/ pub stroke_bundles: SmallVec<[(); 2]>, // TODO
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
        elements.insert(self.fills_wrapper_g.get_id(), &self.fills_wrapper_g);
        self.fill_bundles.iter().for_each(|fill| {
            let elements_map = fill.get_svg_bundle().get_elements();
            for (_, element) in elements_map {
                elements.insert(element.get_id(), element);
            }
        });
        elements.insert(self.strokes_wrapper_g.get_id(), &self.strokes_wrapper_g);
        // TODO: stroke_bundles
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
        elements.insert(self.fills_wrapper_g.get_id(), &mut self.fills_wrapper_g);
        self.fill_bundles.iter_mut().for_each(|fill| {
            let elements_map = fill.get_svg_bundle_mut().get_elements_mut();
            for (_, element) in elements_map {
                elements.insert(element.get_id(), element);
            }
        });
        elements.insert(self.strokes_wrapper_g.get_id(), &mut self.strokes_wrapper_g);
        // TODO: stroke_bundles
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
        #[cfg(feature = "tracing")]
        root_g_element.set_attribute(SvgAttribute::Name {
            name: Self::create_element_name(root_g_element.get_id(), "root"),
        });

        let mut defs_element = cx.create_element(SvgTag::Defs);
        #[cfg(feature = "tracing")]
        defs_element.set_attribute(SvgAttribute::Name {
            name: Self::create_element_name(defs_element.get_id(), "defs"),
        });
        root_g_element.append_child_in_bundle_context(entity, &mut defs_element);

        let mut children_clip_path_element = cx.create_element(SvgTag::ClipPath);
        #[cfg(feature = "tracing")]
        children_clip_path_element.set_attribute(SvgAttribute::Name {
            name: Self::create_element_name(
                children_clip_path_element.get_id(),
                "children-clip-path",
            ),
        });
        defs_element.append_child_in_bundle_context(entity, &mut children_clip_path_element);

        let mut children_clipped_path_element = cx.create_element(SvgTag::Path);
        #[cfg(feature = "tracing")]
        children_clipped_path_element.set_attribute(SvgAttribute::Name {
            name: Self::create_element_name(
                children_clipped_path_element.get_id(),
                "children-clipped-path",
            ),
        });
        children_clip_path_element
            .append_child_in_bundle_context(entity, &mut children_clipped_path_element);

        let mut click_area_rect_element = cx.create_element(SvgTag::Rect);
        #[cfg(feature = "tracing")]
        click_area_rect_element.set_attributes(vec![
            SvgAttribute::Name {
                name: Self::create_element_name(
                    click_area_rect_element.get_id(),
                    "click-area-rect",
                ),
            },
            SvgAttribute::Fill {
                fill: String::from("rgba(255, 204, 203, 0.5)"),
            },
        ]);
        #[cfg(not(feature = "tracing"))]
        click_area_rect_element.set_attribute(SvgAttribute::Fill {
            fill: String::from("transparent"),
        });
        click_area_rect_element.set_attribute(SvgAttribute::PointerEvents {
            pointer_events: SvgPointerEventsVariants::All,
        });
        root_g_element.append_child_in_bundle_context(entity, &mut click_area_rect_element);

        let mut fills_wrapper_g_element = cx.create_element(SvgTag::Group);
        #[cfg(feature = "tracing")]
        fills_wrapper_g_element.set_attribute(SvgAttribute::Name {
            name: Self::create_element_name(fills_wrapper_g_element.get_id(), "fills"),
        });
        root_g_element.append_child_in_bundle_context(entity, &mut fills_wrapper_g_element);

        let mut strokes_wrapper_g_element = cx.create_element(SvgTag::Group);
        #[cfg(feature = "tracing")]
        strokes_wrapper_g_element.set_attribute(SvgAttribute::Name {
            name: Self::create_element_name(strokes_wrapper_g_element.get_id(), "strokes"),
        });
        root_g_element.append_child_in_bundle_context(entity, &mut strokes_wrapper_g_element);

        let mut children_wrapper_g_element = cx.create_element(SvgTag::Group);
        #[cfg(feature = "tracing")]
        children_wrapper_g_element.set_attribute(SvgAttribute::Name {
            name: Self::create_element_name(children_wrapper_g_element.get_id(), "children"),
        });
        children_wrapper_g_element.set_attribute(SvgAttribute::ClipPath {
            clip_path: children_clip_path_element.get_id(),
        });
        root_g_element.append_child_in_bundle_context(entity, &mut children_wrapper_g_element);

        Self {
            node_entity: entity,

            root_g: root_g_element,
            defs: defs_element,
            children_clip_path: children_clip_path_element,
            children_clipped_path: children_clipped_path_element,
            click_area_rect: click_area_rect_element,
            fills_wrapper_g: fills_wrapper_g_element,
            fill_bundles: SmallVec::new(),
            strokes_wrapper_g: strokes_wrapper_g_element,
            stroke_bundles: SmallVec::new(),
            children_wrapper_g: children_wrapper_g_element,
            child_nodes: SmallVec::new(),
        }
    }

    #[cfg(feature = "tracing")]
    fn create_element_name(id: SvgElementId, category: &str) -> String {
        format!("frame-node_{}_{}", category, id)
    }
}
