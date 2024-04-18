// Resources:
// https://observablehq.com/@bumbeishvili/svg-drop-shadows
// https://vanseodesign.com/web-design/svg-filter-primitives-input-output/
// https://www.xanthir.com/b4Yv0
// https://codepen.io/mullany/pen/xxPOoX
// https://tympanus.net/codrops/2019/01/15/svg-filters-101/

use crate::{
    resources::svg_context::SvgContextRes,
    svg::{
        svg_bundle::SvgBundle,
        svg_element::{
            attributes::{
                ColorMatrix, SvgAttribute, SvgAttributeFilter, SvgAttributeIn, SvgAttributeMode,
                SvgAttributeOperator, SvgAttributeType, SvgAttributeValues, SvgUnits,
            },
            SvgElement, SvgTag,
        },
    },
};
use bevy_ecs::entity::Entity;

#[derive(Debug, Clone)]
pub struct DropShadowEffectStyleSvgBundle {
    pub entity: Entity,

    pub root_g: SvgElement,
    /**/ pub defs: SvgElement,
    /**//**/ pub filter: SvgElement,
    /**//**//**/ pub fe_flood: SvgElement,
    /**//**//**/ pub source_alpha_fe_color_matrix: SvgElement,
    /**//**//**/ pub source_alpha_fe_morphology: SvgElement,
    /**//**//**/ pub fe_offset: SvgElement,
    /**//**//**/ pub fe_gaussian_blur: SvgElement,
    /**//**//**/ pub fe_color_matrix: SvgElement,
    /**//**//**/ pub shadow_fe_blend: SvgElement,
    /**//**//**/ pub final_graphic_fe_blend: SvgElement,
    pub shape_path: SvgElement,
}

impl SvgBundle for DropShadowEffectStyleSvgBundle {
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
                .chain(std::iter::once(&self.filter))
                .chain(std::iter::once(&self.fe_flood))
                .chain(std::iter::once(&self.source_alpha_fe_color_matrix))
                .chain(std::iter::once(&self.source_alpha_fe_morphology))
                .chain(std::iter::once(&self.fe_offset))
                .chain(std::iter::once(&self.fe_gaussian_blur))
                .chain(std::iter::once(&self.fe_color_matrix))
                .chain(std::iter::once(&self.shadow_fe_blend))
                .chain(std::iter::once(&self.final_graphic_fe_blend))
                .chain(std::iter::once(&self.shape_path)),
        )
    }

    fn elements_iter_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item = &'a mut SvgElement> + 'a> {
        Box::new(
            std::iter::once(&mut self.root_g)
                .chain(std::iter::once(&mut self.defs))
                .chain(std::iter::once(&mut self.filter))
                .chain(std::iter::once(&mut self.fe_flood))
                .chain(std::iter::once(&mut self.source_alpha_fe_color_matrix))
                .chain(std::iter::once(&mut self.source_alpha_fe_morphology))
                .chain(std::iter::once(&mut self.fe_offset))
                .chain(std::iter::once(&mut self.fe_gaussian_blur))
                .chain(std::iter::once(&mut self.fe_color_matrix))
                .chain(std::iter::once(&mut self.shadow_fe_blend))
                .chain(std::iter::once(&mut self.final_graphic_fe_blend))
                .chain(std::iter::once(&mut self.shape_path)),
        )
    }
}

impl DropShadowEffectStyleSvgBundle {
    pub fn new(entity: Entity, cx: &mut SvgContextRes) -> Self {
        log::info!("[DropShadowEffectStyleSvgBundle::new] {:?}", entity);

        let mut root_g_element = cx.create_bundle_root_element(SvgTag::Group, entity);

        let mut defs_element = cx.create_element(SvgTag::Defs);
        root_g_element.append_child_in_bundle_context(&mut defs_element);

        let mut filter_element = cx.create_element(SvgTag::Filter);
        filter_element.set_attributes(vec![
            SvgAttribute::ColorInterpolationFilters {
                color_interpolation_filters: String::from("sRGB"),
            },
            SvgAttribute::FilterUnits {
                filter_units: SvgUnits::ObjectBoundingBox,
            },
        ]);
        defs_element.append_child_in_bundle_context(&mut filter_element);

        let mut fe_flood_element = cx.create_element(SvgTag::FeFlood);
        fe_flood_element.set_attributes(vec![
            SvgAttribute::FloodOpacity { flood_opacity: 0.0 },
            SvgAttribute::Result {
                result: String::from("backgroundFix"),
            },
        ]);
        filter_element.append_child_in_bundle_context(&mut fe_flood_element);

        let mut source_alpha_fe_color_matrix_element = cx.create_element(SvgTag::FeColorMatrix);
        source_alpha_fe_color_matrix_element.set_attributes(vec![
            SvgAttribute::In {
                value: SvgAttributeIn::SourceAlpha,
            },
            SvgAttribute::Type {
                value: SvgAttributeType::Matrix,
            },
            SvgAttribute::Values {
                values: SvgAttributeValues::ColorMatrix(ColorMatrix::from_rgba(0, 0, 0, 127.0)),
            },
        ]);
        filter_element.append_child_in_bundle_context(&mut source_alpha_fe_color_matrix_element);

        let mut source_alpha_fe_morphology_element = cx.create_element(SvgTag::FeMorphology);
        source_alpha_fe_morphology_element.set_attributes(vec![
            // SvgAttribute::Radius { radius: 20.0 },
            SvgAttribute::Operator {
                operator: SvgAttributeOperator::Dilate,
            },
            SvgAttribute::In {
                value: SvgAttributeIn::SourceAlpha,
            },
            SvgAttribute::Result {
                result: String::from("shadow"),
            },
        ]);
        filter_element.append_child_in_bundle_context(&mut source_alpha_fe_morphology_element);

        let mut fe_offset_element = cx.create_element(SvgTag::FeOffset);
        filter_element.append_child_in_bundle_context(&mut fe_offset_element);

        let mut fe_gaussian_blur_element = cx.create_element(SvgTag::FeGaussianBlur);
        filter_element.append_child_in_bundle_context(&mut fe_gaussian_blur_element);

        let mut fe_color_matrix_element = cx.create_element(SvgTag::FeColorMatrix);
        fe_color_matrix_element.set_attribute(SvgAttribute::Type {
            value: SvgAttributeType::Matrix,
        });
        filter_element.append_child_in_bundle_context(&mut fe_color_matrix_element);

        let mut shadow_fe_blend_element = cx.create_element(SvgTag::FeBlend);
        shadow_fe_blend_element.set_attributes(vec![
            SvgAttribute::Mode {
                mode: SvgAttributeMode::Normal,
            },
            SvgAttribute::In2 {
                value: SvgAttributeIn::Other(String::from("backgroundFix")),
            },
            SvgAttribute::Result {
                result: String::from("shadow"),
            },
        ]);
        filter_element.append_child_in_bundle_context(&mut shadow_fe_blend_element);

        let mut final_graphic_fe_blend_element = cx.create_element(SvgTag::FeBlend);
        final_graphic_fe_blend_element.set_attributes(vec![
            SvgAttribute::Mode {
                mode: SvgAttributeMode::Normal,
            },
            SvgAttribute::In {
                value: SvgAttributeIn::SourceGraphic,
            },
            SvgAttribute::In2 {
                value: SvgAttributeIn::Other(String::from("shadow")),
            },
            SvgAttribute::Result {
                result: String::from("finalGraphic"),
            },
        ]);
        filter_element.append_child_in_bundle_context(&mut final_graphic_fe_blend_element);

        let mut shape_path_element = cx.create_element(SvgTag::Path);
        shape_path_element.set_attribute(SvgAttribute::Filter {
            filter: SvgAttributeFilter::Reference {
                id: filter_element.get_id(),
            },
        });
        root_g_element.append_child_in_bundle_context(&mut shape_path_element);

        Self {
            entity,
            root_g: root_g_element,
            defs: defs_element,
            filter: filter_element,
            fe_flood: fe_flood_element,
            source_alpha_fe_color_matrix: source_alpha_fe_color_matrix_element,
            source_alpha_fe_morphology: source_alpha_fe_morphology_element,
            fe_offset: fe_offset_element,
            fe_gaussian_blur: fe_gaussian_blur_element,
            fe_color_matrix: fe_color_matrix_element,
            shadow_fe_blend: shadow_fe_blend_element,
            final_graphic_fe_blend: final_graphic_fe_blend_element,
            shape_path: shape_path_element,
        }
    }
}
