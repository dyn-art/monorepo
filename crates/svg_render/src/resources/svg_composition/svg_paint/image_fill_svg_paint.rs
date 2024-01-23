use bevy_ecs::entity::Entity;
use dyn_composition::core::utils::continuous_id::ContinuousId;

use crate::{
    events::output_event::ElementChangeEvent,
    resources::{
        changed_components::ChangedPaint,
        svg_composition::{
            svg_bundle::{BaseSVGBundle, SVGBundle},
            svg_element::{
                attributes::{PatternUnit, SVGAttribute, SVGTransformAttribute},
                SVGElement, SVGTag,
            },
            svg_node::ElementReference,
            SVGCompositionRes,
        },
    },
};

use super::SVGPaint;

#[derive(Debug)]
pub struct ImageFillSVGPaint {
    bundle: BaseSVGBundle,

    defs: ElementReference,

    // Paint elements
    paint_pattern: ElementReference,
    paint_clipped_image: ElementReference,
    paint_rect: ElementReference,
}

impl SVGBundle for ImageFillSVGPaint {
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

impl SVGPaint for ImageFillSVGPaint {
    fn apply_paint_change(&mut self, changed_paint: &ChangedPaint) {
        // TODO
        // match &changed_paint.paint {
        //     Paint::Image(paint) => {
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

        //         let paint_clipped_image_element = self
        //             .bundle
        //             .get_child_mut(self.paint_clipped_image.index)
        //             .unwrap();
        //         paint_clipped_image_element.set_attribute(SVGAttribute::Href {
        //             href: match &paint.content {
        //                 ImageContent::Binary { content } => HrefVariant::Binary {
        //                     content: content.clone(),
        //                 },
        //                 ImageContent::Url { url } => HrefVariant::Url { url: url.clone() },
        //             },
        //         });

        //         if let Some(dimension) = &changed_paint.parent_dimension_mixin {
        //             let paint_rect_element =
        //                 self.bundle.get_child_mut(self.paint_rect.index).unwrap();
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

        //             let paint_pattern_element =
        //                 self.bundle.get_child_mut(self.paint_pattern.index).unwrap();
        //             paint_pattern_element.set_attributes(vec![
        //                 SVGAttribute::Width {
        //                     width: dimension.width,
        //                     unit: SVGMeasurementUnit::Pixel,
        //                 },
        //                 SVGAttribute::Height {
        //                     height: dimension.height,
        //                     unit: SVGMeasurementUnit::Pixel,
        //                 },
        //             ]);

        //             let paint_clipped_image_element = self
        //                 .bundle
        //                 .get_child_mut(self.paint_clipped_image.index)
        //                 .unwrap();
        //             paint_clipped_image_element.set_attributes(vec![
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

        //         match paint.scale_mode {
        //             ImagePaintScaleMode::Fill { rotation } => {
        //                 // TODO
        //             }
        //             _ => {}
        //         }
        //     }
        //     _ => {}
        // }
    }
}

impl ImageFillSVGPaint {
    pub fn new(entity: Entity, id_generator: &mut ContinuousId) -> Self {
        // Create root element
        let mut element = SVGElement::new(SVGTag::Group, id_generator);
        #[cfg(feature = "tracing")]
        element.set_attribute(SVGAttribute::Name {
            name: ShapeSVGNode::create_element_name(element.get_id(), String::from("root"), false),
        });
        let mut bundle = BaseSVGBundle::new(element, entity);

        let mut defs_element = SVGElement::new(SVGTag::Defs, id_generator);
        let defs_id = defs_element.get_id();
        #[cfg(feature = "tracing")]
        defs_element.set_attribute(SVGAttribute::Name {
            name: ShapeSVGNode::create_element_name(defs_id, String::from("defs"), false),
        });
        let defs_index = bundle.append_child(defs_element);

        // Create paint elements
        let mut paint_pattern_element = SVGElement::new(SVGTag::Pattern, id_generator);
        let paint_pattern_id = paint_pattern_element.get_id();
        #[cfg(feature = "tracing")]
        paint_pattern_element.set_attribute(SVGAttribute::Name {
            name: ShapeSVGNode::create_element_name(
                paint_pattern_id,
                String::from("paint-pattern"),
                true,
            ),
        });
        paint_pattern_element.set_attribute(SVGAttribute::PatternUnits {
            unit: PatternUnit::UserSpaceOnUse,
        });
        let paint_pattern_index = bundle
            .append_child_to(defs_index, paint_pattern_element)
            .unwrap();

        let mut paint_clipped_image_element = SVGElement::new(SVGTag::Image, id_generator);
        let paint_clipped_image_id = paint_clipped_image_element.get_id();
        #[cfg(feature = "tracing")]
        paint_clipped_image_element.set_attribute(SVGAttribute::Name {
            name: ShapeSVGNode::create_element_name(
                paint_clipped_image_id,
                String::from("paint-clipped-image"),
                false,
            ),
        });
        paint_clipped_image_element.set_attribute(SVGAttribute::Transform {
            transform: SVGTransformAttribute::Matrix {
                a: 0.0,
                b: 0.0,
                c: 0.0,
                d: 0.0,
                tx: 0.0,
                ty: 0.0,
            },
        });
        let paint_clipped_image_index = bundle
            .append_child_to(paint_pattern_index, paint_clipped_image_element)
            .unwrap();

        let mut paint_rect_element = SVGElement::new(SVGTag::Rect, id_generator);
        let paint_rect_id = paint_rect_element.get_id();
        #[cfg(feature = "tracing")]
        paint_rect_element.set_attribute(SVGAttribute::Name {
            name: ShapeSVGNode::create_element_name(
                paint_rect_id,
                String::from("paint-rect"),
                false,
            ),
        });
        paint_rect_element.set_attribute(SVGAttribute::ReferencedFill {
            id: paint_pattern_id,
        });
        let paint_rect_index = bundle.append_child(paint_rect_element);

        Self {
            bundle,
            defs: ElementReference {
                id: defs_id,
                index: defs_index,
            },

            // Paint element references
            paint_pattern: ElementReference {
                id: paint_pattern_id,
                index: paint_pattern_index,
            },
            paint_clipped_image: ElementReference {
                id: paint_clipped_image_id,
                index: paint_clipped_image_index,
            },
            paint_rect: ElementReference {
                id: paint_rect_id,
                index: paint_rect_index,
            },
        }
    }

    #[cfg(feature = "tracing")]
    fn create_element_name(id: ContinuousId, category: String, is_definition: bool) -> String {
        let def_part = if is_definition { "def" } else { "" };
        format!("image-fill_{}_{}{}", category, id, def_part)
    }
}
