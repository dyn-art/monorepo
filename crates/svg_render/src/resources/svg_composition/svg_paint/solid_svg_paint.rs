use bevy_ecs::entity::Entity;
use dyn_composition::core::utils::continuous_id::ContinuousId;

use crate::{
    events::output_event::ElementChangeEvent,
    resources::{
        changed_components::ChangedPaint,
        svg_composition::{
            svg_bundle::{BaseSVGBundle, SVGBundle},
            svg_element::{SVGElement, SVGTag},
            svg_node::ElementReference,
            SVGCompositionRes,
        },
    },
};

use super::SVGPaint;

#[derive(Debug)]
pub struct SolidSVGPaint {
    bundle: BaseSVGBundle,

    // Elements
    paint_rect: ElementReference,
}

impl SVGBundle for SolidSVGPaint {
    fn get_bundle(&self) -> &BaseSVGBundle {
        &self.bundle
    }

    fn get_bundle_mut(&mut self) -> &mut BaseSVGBundle {
        &mut self.bundle
    }

    fn drain_changes(&mut self) -> Vec<ElementChangeEvent> {
        self.bundle.drain_changes()
    }

    fn to_string(&self, composition: &SVGCompositionRes) -> String {
        self.bundle.to_string(composition)
    }
}

impl SVGPaint for SolidSVGPaint {
    fn apply_paint_change(&mut self, changed_paint: &ChangedPaint) {
        // TODO
        // match &changed_paint.paint {
        //     Paint::Solid(paint) => {
        //         let root_element = self.bundle.get_root_mut();
        //         root_element.set_attributes(vec![SVGAttribute::Opacity {
        //             opacity: paint.base_paint.opacity,
        //         }]);
        //         root_element.set_styles(vec![SVGStyle::Display {
        //             display: if paint.base_paint.is_visible {
        //                 SVGDisplayStyle::Block
        //             } else {
        //                 SVGDisplayStyle::None
        //             },
        //         }]);

        //         let paint_rect_element = self.bundle.get_child_mut(self.paint_rect.index).unwrap();
        //         paint_rect_element.set_attributes(vec![SVGAttribute::Fill {
        //             fill: rgb_to_hex(paint.color),
        //         }]);
        //         if let Some(dimension) = &changed_paint.parent_dimension_mixin {
        //             paint_rect_element.set_attributes(vec![
        //                 SVGAttribute::Width {
        //                     width: dimension.width,
        //                     unit: SVGMeasurementUnit::Pixel,
        //                 },
        //                 SVGAttribute::Height {
        //                     height: dimension.height,
        //                     unit: SVGMeasurementUnit::Pixel,
        //                 },
        //             ]);
        //         }
        //     }
        //     _ => {}
        // }
    }
}

impl SolidSVGPaint {
    pub fn new(entity: Entity, id_generator: &mut ContinuousId) -> Self {
        // Create root element
        let mut element = SVGElement::new(SVGTag::Group, id_generator);
        #[cfg(feature = "tracing")]
        element.set_attribute(SVGAttribute::Name {
            name: SolidSVGPaint::create_element_name(element.get_id(), String::from("root"), false),
        });
        let mut bundle = BaseSVGBundle::new(element, entity);

        // Create paint elements
        let mut paint_rect_element = SVGElement::new(SVGTag::Rect, id_generator);
        let paint_rect_element_id = paint_rect_element.get_id();
        #[cfg(feature = "tracing")]
        paint_rect_element.set_attribute(SVGAttribute::Name {
            name: SolidSVGPaint::create_element_name(
                paint_rect_element_id,
                String::from("paint-rect"),
                false,
            ),
        });
        let paint_rect_element_index = bundle.append_child(paint_rect_element);

        Self {
            bundle,
            paint_rect: ElementReference {
                id: paint_rect_element_id,
                index: paint_rect_element_index,
            },
        }
    }

    #[cfg(feature = "tracing")]
    fn create_element_name(id: ContinuousId, category: String, is_definition: bool) -> String {
        let def_part = if is_definition { "def" } else { "" };
        format!("solid-paint_{}_{}{}", category, id, def_part)
    }
}
