use dyn_composition::core::modules::node::components::mixins::{Paint, SolidPaint};

use crate::core::{
    events::output_event::RenderUpdateEvent,
    modules::svg_render::resources::svg_composition::{
        svg_bundle::{BaseSVGBundle, SVGBundle},
        svg_element::{
            attributes::{SVGAttribute, SVGMeasurementUnit},
            styles::{SVGDisplayStyle, SVGStyle},
            SVGElement, SVGTag,
        },
        svg_node::{ElementReference, SVGNode},
        SVGComposition,
    },
};

use super::SVGPaint;

// TODO:
// - Extract Paint as separate Entity
// - Update FillMixin to only hold Entity ids to Paints
// - Update DTIF load logic ("simply" go through fill mixin paints array
//   and lookup corresponding Paint in paints hashmap of DTIF)
// - Add system to detect Paint entity changes
// - Create paint mixins to change from JS site
// - Update fill paints array change logic in SVGComposition
// ..

#[derive(Debug)]
pub struct SolidSVGPaint {
    bundle: BaseSVGBundle,

    // Elements
    paint_shape: ElementReference,
}

impl SVGBundle for SolidSVGPaint {
    fn get_bundle(&self) -> &BaseSVGBundle {
        &self.bundle
    }

    fn get_bundle_mut(&mut self) -> &mut BaseSVGBundle {
        &mut self.bundle
    }
}

impl SVGPaint for SolidSVGPaint {
    fn apply_paint_change(&mut self, paint: &Paint) {
        match paint {
            Paint::Solid(paint) => {
                let paint_shape_index = self.paint_shape.index;
                self.bundle.set_attributes(vec![SVGAttribute::Opacity {
                    opacity: paint.opacity,
                }]);
                self.bundle.set_styles(vec![SVGStyle::Display {
                    display: if paint.is_visible {
                        SVGDisplayStyle::Block
                    } else {
                        SVGDisplayStyle::None
                    },
                }]);
                self.bundle.set_attributes_at(
                    paint_shape_index,
                    vec![
                        SVGAttribute::Fill {
                            fill: String::from("red"), // TODO
                        },
                        SVGAttribute::Width {
                            width: 100,
                            unit: SVGMeasurementUnit::Percent,
                        },
                        SVGAttribute::Height {
                            height: 100,
                            unit: SVGMeasurementUnit::Percent,
                        },
                    ],
                );
            }
        }
    }

    fn drain_updates(&mut self) -> Vec<RenderUpdateEvent> {
        self.bundle.drain_updates()
    }

    fn to_string(&self, node: &dyn SVGNode, composition: &SVGComposition) -> String {
        self.bundle.to_string(node, composition)
    }
}

impl SolidSVGPaint {
    pub fn new() -> Self {
        // Create root element and apply it to the solid SVG paint
        let mut element = SVGElement::new(SVGTag::Group);
        #[cfg(feature = "trace")]
        element.set_attribute(SVGAttribute::Name {
            name: SolidSVGPaint::create_element_name(element.get_id(), String::from("root"), false),
        });
        let mut bundle = BaseSVGBundle::new(element);

        // Create paint elements
        let mut paint_shape_element = SVGElement::new(SVGTag::Rect);
        let paint_shape_element_id = paint_shape_element.get_id();
        #[cfg(feature = "trace")]
        paint_shape_element.set_attribute(SVGAttribute::Name {
            name: SolidSVGPaint::create_element_name(
                paint_shape_element_id,
                String::from("shape"),
                false,
            ),
        });
        let paint_shape_element_index = bundle.append_child_element(paint_shape_element);

        Self {
            bundle,
            paint_shape: ElementReference {
                id: paint_shape_element_id,
                index: paint_shape_element_index,
            },
        }
    }

    #[cfg(feature = "trace")]
    fn create_element_name(id: u32, category: String, is_definition: bool) -> String {
        let def_part = if is_definition { "def" } else { "" };
        format!("solid-paint_{}_{}{}", category, id, def_part)
    }
}