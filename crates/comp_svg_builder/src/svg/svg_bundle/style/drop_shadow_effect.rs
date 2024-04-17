// https://observablehq.com/@bumbeishvili/svg-drop-shadows
// https://vanseodesign.com/web-design/svg-filter-primitives-input-output/
// https://www.xanthir.com/b4Yv0
//
// https://codepen.io/mullany/pen/xxPOoX
//
// <filter id="drop-shadow" color-interpolation-filters="sRGB" x="-50%" y="-50%" height="200%" width="200%">
//
// <!-- Take source alpha, offset it by angle/distance and blur it by size -->
// <feOffset id="offset" in="SourceAlpha" dx="3.54" dy="3.54" result="SA-offset"/>
// <feGaussianBlur id="blur" in="SA-offset" stdDeviation="3" result="SA-o-blur"/>
//
// <!-- Apply a contour by using a color curve transform on the alpha and clipping the result to the input -->
//
// <feComponentTransfer in="SA-o-blur" result="SA-o-b-contIN">
//   <feFuncA id="contour" type="table" tableValues="0 1"/>
// </feComponentTransfer>
//
// <feComposite operator="in" in="SA-o-blur" in2="SA-o-b-contIN" result="SA-o-b-cont"/>
//
// <!-- Adjust the spread by multiplying alpha by a constant factor -->
// <feComponentTransfer in="SA-o-b-cont" result="SA-o-b-c-sprd">
//   <feFuncA id="spread-ctrl" type="linear" slope="1"/>
// </feComponentTransfer>
//
// <!-- Adjust color and opacity by adding fixed offsets and an opacity multiplier -->
// <feColorMatrix id="recolor" in="SA-o-b-c-sprd" type="matrix" values="1 0 0 0 0 0 1 0 0 0 0 0 1 0 0 0 0 0 .8 0" result="SA-o-b-c-s-recolor"/>
//
// <!-- Generate a reasonably grainy noise input with baseFrequency between approx .5 to 2.0. And add the noise with k1 and k2 multipliers that sum to 1 -->
// <feTurbulence result="fNoise" type="fractalNoise" numOctaves="6" baseFrequency="1.98"/>
// <feColorMatrix in="fNoise" type="matrix" values="1 0 0 0 0 0 1 0 0 0 0 0 1 0 0 0 0 0 7 -3" result="clipNoise"/>
// <feComposite id="noisemix" operator="arithmetic" in="SA-o-b-c-s-recolor" in2="clipNoise" k1="0" k2="1" result="SA-o-b-c-s-r-mix"/>
//
// <!-- Merge the shadow with the original -->
// <feMerge>
//   <feMergeNode in="SA-o-b-c-s-r-mix"/>
//   <feMergeNode in="SourceGraphic"/>
// </feMerge>
// </filter>

use crate::{
    resources::svg_context::SvgContextRes,
    svg::{
        svg_bundle::SvgBundle,
        svg_element::{
            attributes::{SvgAttribute, SvgAttributeFilter, SvgMeasurementUnit},
            styles::{SvgStyle, SvgStyleColor},
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
    /**//**//**/ pub fe_offset: SvgElement,
    /**//**//**/ pub fe_gaussian_blur: SvgElement,
    /**//**//**/ pub contour_fe_component_transfer: SvgElement,
    /**//**//**//**/ pub contour_fe_func_a: SvgElement,
    /**//**//**/ pub fe_composite: SvgElement,
    /**//**//**/ pub spread_fe_component_transfer: SvgElement,
    /**//**//**//**/ pub spread_fe_func_a: SvgElement,
    /**//**//**/ pub color_fe_color_matrix: SvgElement,
    /**//**//**/ pub noise_fe_turbulence: SvgElement,
    /**//**//**/ pub noise_fe_color_matrix: SvgElement,
    /**//**//**/ pub noise_fe_composite: SvgElement,
    /**//**//**/ pub fe_merge: SvgElement,
    /**//**//**//**/ pub mix_fe_merge_node: SvgElement,
    /**//**//**//**/ pub source_graphic_fe_merge_node: SvgElement,

    /**/ pub shape_path: SvgElement,
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
                .chain(std::iter::once(&self.fe_offset))
                .chain(std::iter::once(&self.fe_gaussian_blur))
                .chain(std::iter::once(&self.contour_fe_component_transfer))
                .chain(std::iter::once(&self.contour_fe_func_a))
                .chain(std::iter::once(&self.fe_composite))
                .chain(std::iter::once(&self.spread_fe_component_transfer))
                .chain(std::iter::once(&self.spread_fe_func_a))
                .chain(std::iter::once(&self.color_fe_color_matrix))
                .chain(std::iter::once(&self.noise_fe_turbulence))
                .chain(std::iter::once(&self.noise_fe_color_matrix))
                .chain(std::iter::once(&self.noise_fe_composite))
                .chain(std::iter::once(&self.fe_merge))
                .chain(std::iter::once(&self.mix_fe_merge_node))
                .chain(std::iter::once(&self.source_graphic_fe_merge_node))
                .chain(std::iter::once(&self.shape_path)),
        )
    }

    fn elements_iter_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item = &'a mut SvgElement> + 'a> {
        Box::new(
            std::iter::once(&mut self.root_g)
                .chain(std::iter::once(&mut self.defs))
                .chain(std::iter::once(&mut self.filter))
                .chain(std::iter::once(&mut self.fe_offset))
                .chain(std::iter::once(&mut self.fe_gaussian_blur))
                .chain(std::iter::once(&mut self.contour_fe_component_transfer))
                .chain(std::iter::once(&mut self.contour_fe_func_a))
                .chain(std::iter::once(&mut self.fe_composite))
                .chain(std::iter::once(&mut self.spread_fe_component_transfer))
                .chain(std::iter::once(&mut self.spread_fe_func_a))
                .chain(std::iter::once(&mut self.color_fe_color_matrix))
                .chain(std::iter::once(&mut self.noise_fe_turbulence))
                .chain(std::iter::once(&mut self.noise_fe_color_matrix))
                .chain(std::iter::once(&mut self.noise_fe_composite))
                .chain(std::iter::once(&mut self.fe_merge))
                .chain(std::iter::once(&mut self.mix_fe_merge_node))
                .chain(std::iter::once(&mut self.source_graphic_fe_merge_node))
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
            SvgAttribute::X {
                x: -50.0,
                unit: SvgMeasurementUnit::Percent,
            },
            SvgAttribute::Y {
                y: -50.0,
                unit: SvgMeasurementUnit::Percent,
            },
            SvgAttribute::Width {
                width: 200.0,
                unit: SvgMeasurementUnit::Percent,
            },
            SvgAttribute::Height {
                height: 200.0,
                unit: SvgMeasurementUnit::Percent,
            },
        ]);
        defs_element.append_child_in_bundle_context(&mut filter_element);

        let mut fe_offset_element = cx.create_element(SvgTag::FeOffset);
        fe_offset_element.set_attributes(vec![
            SvgAttribute::In {
                value: String::from("SourceAlpha"),
            },
            SvgAttribute::DX { dx: 3.54 }, // TODO
            SvgAttribute::DY { dy: 3.54 }, // TODO
            SvgAttribute::Result {
                result: String::from("SA-offset"),
            },
        ]);
        filter_element.append_child_in_bundle_context(&mut fe_offset_element);

        let mut fe_gaussian_blur_element = cx.create_element(SvgTag::FeGaussianBlur);
        fe_gaussian_blur_element.set_attributes(vec![
            SvgAttribute::In {
                value: String::from("SA-offset"),
            },
            SvgAttribute::StdDeviation { std_deviation: 3.0 }, // TODO
            SvgAttribute::Result {
                result: String::from("SA-o-blur"),
            },
        ]);
        filter_element.append_child_in_bundle_context(&mut fe_gaussian_blur_element);

        let mut contour_fe_component_transfer_element =
            cx.create_element(SvgTag::FeComponentTransfer);
        contour_fe_component_transfer_element.set_attributes(vec![
            SvgAttribute::In {
                value: String::from("SA-o-blur"),
            },
            SvgAttribute::Result {
                result: String::from("SA-o-b-contIN"),
            },
        ]);
        filter_element.append_child_in_bundle_context(&mut contour_fe_component_transfer_element);

        let mut contour_fe_func_a_element = cx.create_element(SvgTag::FeFuncA);
        contour_fe_func_a_element.set_attributes(vec![
            SvgAttribute::Type {
                value: String::from("table"),
            },
            SvgAttribute::TableValues {
                table_values: vec![0.0, 1.0], // TODO
            },
        ]);
        contour_fe_component_transfer_element
            .append_child_in_bundle_context(&mut contour_fe_func_a_element);

        let mut fe_composite_element = cx.create_element(SvgTag::FeComposite);
        fe_composite_element.set_attributes(vec![
            SvgAttribute::Operator {
                operator: String::from("in"),
            },
            SvgAttribute::In {
                value: String::from("SA-o-blur"),
            },
            SvgAttribute::In2 {
                value: String::from("SA-o-b-contIN"),
            },
            SvgAttribute::Result {
                result: String::from("SA-o-b-cont"),
            },
        ]);
        filter_element.append_child_in_bundle_context(&mut fe_composite_element);

        let mut spread_fe_component_transfer_element =
            cx.create_element(SvgTag::FeComponentTransfer);
        spread_fe_component_transfer_element.set_attributes(vec![
            SvgAttribute::In {
                value: String::from("SA-o-cont"),
            },
            SvgAttribute::Result {
                result: String::from("SA-o-b-c-sprd"),
            },
        ]);
        filter_element.append_child_in_bundle_context(&mut spread_fe_component_transfer_element);

        let mut spread_fe_func_a_element = cx.create_element(SvgTag::FeFuncA);
        spread_fe_func_a_element.set_attributes(vec![
            SvgAttribute::Type {
                value: String::from("linear"),
            },
            SvgAttribute::Slope {
                slope: 1.0, // TODO
            },
        ]);
        spread_fe_component_transfer_element
            .append_child_in_bundle_context(&mut spread_fe_func_a_element);

        let mut color_fe_color_matrix_element = cx.create_element(SvgTag::FeColorMatrix);
        color_fe_color_matrix_element.set_attributes(vec![
            SvgAttribute::In {
                value: String::from("SA-o-b-c-sprd"),
            },
            SvgAttribute::Type {
                value: String::from("matrix"),
            },
            SvgAttribute::Values {
                values: String::from("1 0 0 0 0 0 1 0 0 0 0 0 1 0 0 0 0 0 0.8 0"), // TODO
            },
            SvgAttribute::Result {
                result: String::from("SA-o-b-c-s-recolor"),
            },
        ]);
        filter_element.append_child_in_bundle_context(&mut color_fe_color_matrix_element);

        let mut noise_fe_turbulence_element = cx.create_element(SvgTag::FeTurbulence);
        noise_fe_turbulence_element.set_attributes(vec![
            SvgAttribute::Result {
                result: String::from("fNoise"),
            },
            SvgAttribute::Type {
                value: String::from("fractalNoise"),
            },
            SvgAttribute::NumOctaves { num_octaves: 6 }, // TODO
            SvgAttribute::BaseFrequency {
                base_frequency: 1.98,
            }, // TODO
        ]);
        filter_element.append_child_in_bundle_context(&mut noise_fe_turbulence_element);

        let mut noise_fe_color_matrix_element = cx.create_element(SvgTag::FeColorMatrix);
        noise_fe_color_matrix_element.set_attributes(vec![
            SvgAttribute::In {
                value: String::from("fNoise"),
            },
            SvgAttribute::Type {
                value: String::from("matrix"),
            },
            SvgAttribute::Values {
                values: String::from("1 0 0 0 0 0 1 0 0 0 0 0 1 0 0 0 0 0 7 -3"), // TODO
            },
            SvgAttribute::Result {
                result: String::from("clipNoise"),
            },
        ]);
        filter_element.append_child_in_bundle_context(&mut noise_fe_color_matrix_element);

        let mut noise_fe_composite_element = cx.create_element(SvgTag::FeComposite);
        noise_fe_composite_element.set_attributes(vec![
            SvgAttribute::Operator {
                operator: String::from("arithmetic"),
            },
            SvgAttribute::In {
                value: String::from("SA-o-b-c-s-recolor"),
            },
            SvgAttribute::In2 {
                value: String::from("clipNoise"),
            },
            SvgAttribute::K1 { k1: 0.0 },
            SvgAttribute::K2 { k2: 1.0 },
            SvgAttribute::Result {
                result: String::from("SA-o-b-c-s-r-mix"),
            },
        ]);
        filter_element.append_child_in_bundle_context(&mut noise_fe_composite_element);

        let mut fe_merge_element = cx.create_element(SvgTag::FeMerge);
        filter_element.append_child_in_bundle_context(&mut fe_merge_element);

        let mut mix_fe_merge_node_element = cx.create_element(SvgTag::FeMergeNode);
        mix_fe_merge_node_element.set_attribute(SvgAttribute::In {
            value: String::from("SA-o-b-c-s-r-mix"),
        });
        fe_merge_element.append_child_in_bundle_context(&mut mix_fe_merge_node_element);

        let mut source_graphic_fe_merge_node_element = cx.create_element(SvgTag::FeMergeNode);
        source_graphic_fe_merge_node_element.set_attribute(SvgAttribute::In {
            value: String::from("SourceGraphic"),
        });
        fe_merge_element.append_child_in_bundle_context(&mut source_graphic_fe_merge_node_element);

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
            fe_offset: fe_offset_element,
            fe_gaussian_blur: fe_gaussian_blur_element,
            contour_fe_component_transfer: contour_fe_component_transfer_element,
            contour_fe_func_a: contour_fe_func_a_element,
            fe_composite: fe_composite_element,
            spread_fe_component_transfer: spread_fe_component_transfer_element,
            spread_fe_func_a: spread_fe_func_a_element,
            color_fe_color_matrix: color_fe_color_matrix_element,
            noise_fe_turbulence: noise_fe_turbulence_element,
            noise_fe_color_matrix: noise_fe_color_matrix_element,
            noise_fe_composite: noise_fe_composite_element,
            fe_merge: fe_merge_element,
            mix_fe_merge_node: mix_fe_merge_node_element,
            source_graphic_fe_merge_node: source_graphic_fe_merge_node_element,
            shape_path: shape_path_element,
        }
    }
}
