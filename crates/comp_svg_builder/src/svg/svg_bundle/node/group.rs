use crate::{
    resources::svg_context::SvgContextRes,
    svg::{
        svg_bundle::SvgBundle,
        svg_element::{
            styles::{SvgPointerEventsStyle, SvgStyle, SvgStyleColor},
            SvgElement, SvgTag,
        },
    },
};
use bevy_ecs::entity::Entity;
use smallvec::SmallVec;

#[derive(Debug, Clone)]
pub struct GroupNodeSvgBundle {
    pub entity: Entity,

    pub root_g: SvgElement,
    /**/ pub click_area_rect: SvgElement,
    /**/ pub children_wrapper_g: SvgElement,
    /**//**/ pub child_node_entities: SmallVec<[Entity; 2]>,
}

impl SvgBundle for GroupNodeSvgBundle {
    fn get_entity(&self) -> &Entity {
        &self.entity
    }

    fn get_root_element(&self) -> &SvgElement {
        &self.root_g
    }

    fn get_root_element_mut(&mut self) -> &mut SvgElement {
        &mut self.root_g
    }

    fn elements_iter<'a>(&'a self) -> Box<dyn Iterator<Item = &'a SvgElement> + 'a> {
        Box::new(
            std::iter::once(&self.root_g)
                .chain(std::iter::once(&self.click_area_rect))
                .chain(std::iter::once(&self.children_wrapper_g)),
        )
    }

    fn elements_iter_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item = &'a mut SvgElement> + 'a> {
        Box::new(
            std::iter::once(&mut self.root_g)
                .chain(std::iter::once(&mut self.click_area_rect))
                .chain(std::iter::once(&mut self.children_wrapper_g)),
        )
    }
}

impl GroupNodeSvgBundle {
    pub fn new(entity: Entity, cx: &mut SvgContextRes) -> Self {
        log::info!("[GroupNodeSvgBundle::new] {:?}", entity);

        let mut root_g_element = cx.create_bundle_root_element(SvgTag::Group, entity);

        let mut click_area_rect_element = cx.create_element(SvgTag::Rect);
        click_area_rect_element.set_styles(vec![
            SvgStyle::PointerEvents {
                pointer_events: SvgPointerEventsStyle::All,
            },
            SvgStyle::Fill {
                fill: SvgStyleColor::None,
            },
        ]);
        root_g_element.append_child_in_bundle_context(&mut click_area_rect_element);

        let mut children_wrapper_g_element = cx.create_element(SvgTag::Group);
        root_g_element.append_child_in_bundle_context(&mut children_wrapper_g_element);

        #[cfg(feature = "tracing")]
        {
            use crate::svg::svg_element::attributes::SvgAttribute;

            root_g_element.set_attribute(SvgAttribute::Class {
                class: Self::create_element_name(
                    root_g_element.get_id(),
                    &format!("root-{:?}", entity),
                ),
            });
            click_area_rect_element.set_attribute(SvgAttribute::Class {
                class: Self::create_element_name(
                    click_area_rect_element.get_id(),
                    "click-area-rect",
                ),
            });
            click_area_rect_element.set_style(SvgStyle::Fill {
                fill: SvgStyleColor::RGBA {
                    red: 255,
                    green: 204,
                    blue: 203,
                    alpha: 0.5,
                },
            });
            children_wrapper_g_element.set_attribute(SvgAttribute::Class {
                class: Self::create_element_name(children_wrapper_g_element.get_id(), "children"),
            });
        }

        Self {
            entity,

            root_g: root_g_element,
            click_area_rect: click_area_rect_element,
            children_wrapper_g: children_wrapper_g_element,
            child_node_entities: SmallVec::new(),
        }
    }

    #[cfg(feature = "tracing")]
    #[inline]
    fn create_element_name(id: crate::svg::svg_element::SvgElementId, category: &str) -> String {
        format!("group-node_{}_{}", category, id)
    }
}
