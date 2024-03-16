use crate::{
    resources::svg_context::SvgContextRes,
    svg::{
        svg_bundle::SvgBundle,
        svg_element::{
            attributes::{SvgAttribute, SvgUnits},
            styles::{SvgStyle, SvgStyleColor},
            SvgElement, SvgTag,
        },
    },
};
use bevy_ecs::entity::Entity;
use dyn_comp_common::common::ImageScaleMode;

#[derive(Debug, Clone)]
pub struct ImageFillStyleSvgBundle {
    pub entity: Entity,
    pub variant: ImageFillStyleVariant,

    pub root_g: SvgElement,
    /**/ pub defs: SvgElement,
    /**//**/ pub pattern: SvgElement,
    /**//**/ pub image: SvgElement,
    /**/ pub shape_path: SvgElement,
}

impl SvgBundle for ImageFillStyleSvgBundle {
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
                .chain(std::iter::once(&self.defs))
                .chain(std::iter::once(&self.pattern))
                .chain(std::iter::once(&self.image))
                .chain(std::iter::once(&self.shape_path)),
        )
    }

    fn elements_iter_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item = &'a mut SvgElement> + 'a> {
        Box::new(
            std::iter::once(&mut self.root_g)
                .chain(std::iter::once(&mut self.defs))
                .chain(std::iter::once(&mut self.pattern))
                .chain(std::iter::once(&mut self.image))
                .chain(std::iter::once(&mut self.shape_path)),
        )
    }
}

impl ImageFillStyleSvgBundle {
    pub fn new(entity: Entity, scale_mode: ImageScaleMode, cx: &mut SvgContextRes) -> Self {
        log::info!("[ImageFillStyleSvgBundle::new] {:?}", entity);

        let mut root_g_element = cx.create_bundle_root_element(SvgTag::Group, entity);

        let mut defs_element = cx.create_element(SvgTag::Defs);
        root_g_element.append_child_in_bundle_context(&mut defs_element);

        let mut pattern_element = cx.create_element(SvgTag::Pattern);
        pattern_element.set_attribute(SvgAttribute::PatternUnits {
            pattern_units: SvgUnits::UserSpaceOnUse,
        });
        defs_element.append_child_in_bundle_context(&mut pattern_element);

        let mut image_element = cx.create_element(SvgTag::Image);
        match scale_mode {
            ImageScaleMode::Fill => {
                image_element.set_attribute(SvgAttribute::PreserveAspectRatio {
                    preserve_aspect_ratio: String::from("xMidYMid slice"),
                });
            }
            _ => {}
        }
        pattern_element.append_child_in_bundle_context(&mut image_element);

        let mut shape_path_element = cx.create_element(SvgTag::Path);
        shape_path_element.set_style(SvgStyle::Fill {
            fill: SvgStyleColor::Reference {
                id: pattern_element.get_id(),
            },
        });
        root_g_element.append_child_in_bundle_context(&mut shape_path_element);

        #[cfg(feature = "tracing")]
        {
            use crate::svg::svg_element::attributes::SvgAttribute;

            root_g_element.set_attribute(SvgAttribute::Class {
                class: Self::create_element_name(
                    root_g_element.get_id(),
                    &format!("root-{:?}", entity),
                ),
            });
            defs_element.set_attribute(SvgAttribute::Class {
                class: Self::create_element_name(defs_element.get_id(), "defs"),
            });
            pattern_element.set_attribute(SvgAttribute::Class {
                class: Self::create_element_name(pattern_element.get_id(), "pattern"),
            });
            image_element.set_attribute(SvgAttribute::Class {
                class: Self::create_element_name(image_element.get_id(), "image"),
            });
            shape_path_element.set_attribute(SvgAttribute::Class {
                class: Self::create_element_name(shape_path_element.get_id(), "shape-path"),
            });
        }

        Self {
            entity,
            variant: match scale_mode {
                ImageScaleMode::Fill => ImageFillStyleVariant::Fill,
                ImageScaleMode::Fit => ImageFillStyleVariant::Fit,
                ImageScaleMode::Crop { .. } => ImageFillStyleVariant::Crop,
                ImageScaleMode::Tile { .. } => ImageFillStyleVariant::Tile,
            },

            root_g: root_g_element,
            defs: defs_element,
            pattern: pattern_element,
            image: image_element,
            shape_path: shape_path_element,
        }
    }

    #[cfg(feature = "tracing")]
    #[inline]
    fn create_element_name(id: crate::svg::svg_element::SvgElementId, category: &str) -> String {
        format!("image-fill_{}_{}", category, id)
    }
}

#[derive(Debug, Clone)]
pub enum ImageFillStyleVariant {
    Fill,
    Fit,
    Crop,
    Tile,
}
