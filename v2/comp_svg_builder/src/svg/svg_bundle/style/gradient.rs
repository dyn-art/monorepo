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
use dyn_comp_common::common::GradientVariant;
use smallvec::SmallVec;

#[derive(Debug, Clone)]
pub struct GradientStyleSvgBundle {
    pub entity: Entity,

    pub root_g: SvgElement,
    /**/ pub defs: SvgElement,
    /**//**/ pub gradient: SvgElement,
    /**//**/ pub gradient_stops: SmallVec<[SvgElement; 2]>,
    /**/ pub shape_path: SvgElement,
}

impl SvgBundle for GradientStyleSvgBundle {
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
                .chain(std::iter::once(&self.gradient))
                .chain(self.gradient_stops.iter())
                .chain(std::iter::once(&self.shape_path)),
        )
    }

    fn elements_iter_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item = &'a mut SvgElement> + 'a> {
        Box::new(
            std::iter::once(&mut self.root_g)
                .chain(std::iter::once(&mut self.defs))
                .chain(std::iter::once(&mut self.gradient))
                .chain(self.gradient_stops.iter_mut())
                .chain(std::iter::once(&mut self.shape_path)),
        )
    }
}

impl GradientStyleSvgBundle {
    pub fn new(entity: Entity, gradient_variant: GradientVariant, cx: &mut SvgContextRes) -> Self {
        log::info!("[SolidPaintSvgBundle::new] {:?}", entity);

        let mut root_g_element = cx.create_bundle_root_element(SvgTag::Group, entity);

        let mut defs_element = cx.create_element(SvgTag::Defs);
        root_g_element.append_child_in_bundle_context(&mut defs_element);

        let mut gradient_element = cx.create_element(match gradient_variant {
            GradientVariant::Linear { .. } => SvgTag::LinearGradient,
            GradientVariant::Radial { .. } => SvgTag::RadialGradient,
        });
        gradient_element.set_attribute(SvgAttribute::GradientUnits {
            gradient_units: SvgUnits::UserSpaceOnUse,
        });
        defs_element.append_child_in_bundle_context(&mut gradient_element);

        let mut shape_path_element = cx.create_element(SvgTag::Path);
        shape_path_element.set_style(SvgStyle::Fill {
            fill: SvgStyleColor::Reference {
                id: gradient_element.get_id(),
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
            gradient_element.set_attribute(SvgAttribute::Class {
                class: Self::create_element_name(gradient_element.get_id(), "gradient"),
            });
            shape_path_element.set_attribute(SvgAttribute::Class {
                class: Self::create_element_name(shape_path_element.get_id(), "shape-path"),
            });
        }

        Self {
            entity,

            root_g: root_g_element,
            defs: defs_element,
            gradient: gradient_element,
            gradient_stops: SmallVec::new(),
            shape_path: shape_path_element,
        }
    }

    #[cfg(feature = "tracing")]
    #[inline]
    fn create_element_name(id: crate::svg::svg_element::SvgElementId, category: &str) -> String {
        format!("gradient-fill_{}_{}", category, id)
    }
}
