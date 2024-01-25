use bevy_ecs::entity::Entity;
use dyn_composition::core::{
    modules::node::components::types::GradientVariant, utils::continuous_id::ContinuousId,
};

use crate::{
    events::output_event::ElementChangeEvent,
    mixin_change::PaintMixinChange,
    resources::{
        changed_components::ChangedPaint,
        svg_composition::{
            svg_bundle::{BaseSVGBundle, SVGBundle},
            svg_element::{
                attributes::{SVGAttribute, SVGMeasurementUnit},
                mapper::map_blend_mode,
                styles::{SVGDisplayStyle, SVGStyle},
                SVGElement, SVGTag,
            },
            svg_node::ElementReference,
            SVGCompositionRes,
        },
    },
};

use super::SVGPaint;

#[derive(Debug)]
pub struct GradientSVGPaint {
    bundle: BaseSVGBundle,
    variant: GradientSVGPaintVariant,

    defs: ElementReference,

    // Paint elements
    paint_gradient: ElementReference,
    paint_gradient_stops: Vec<ElementReference>,
    paint_rect: ElementReference,
}

#[derive(Debug)]
enum GradientSVGPaintVariant {
    Linear,
    Radial,
    Unsupported,
}

impl SVGBundle for GradientSVGPaint {
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

impl SVGPaint for GradientSVGPaint {
    fn apply_paint_change(&mut self, changed_paint: &ChangedPaint) {
        for change in &changed_paint.changes {
            match change {
                PaintMixinChange::GradientPaint(mixin) => {
                    // Clear children
                    self.bundle
                        .get_child_mut(self.paint_gradient.index)
                        .unwrap()
                        .clear_children();

                    // TODO: Add new stops
                    for gradient_stop in &mixin.gradient_stops {
                        self.bundle
                            .append_child_to(self.paint_gradient.index, todo!());
                    }
                }
                PaintMixinChange::PaintComposition(mixin) => {
                    self.bundle
                        .get_root_mut()
                        .set_styles(vec![SVGStyle::Display {
                            display: if mixin.is_visible {
                                SVGDisplayStyle::Block
                            } else {
                                SVGDisplayStyle::None
                            },
                        }]);
                }
                PaintMixinChange::Dimension(mixin) => {
                    self.bundle
                        .get_child_mut(self.paint_rect.index)
                        .unwrap()
                        .set_attributes(vec![
                            SVGAttribute::Width {
                                width: mixin.width,
                                unit: SVGMeasurementUnit::Pixel,
                            },
                            SVGAttribute::Height {
                                height: mixin.height,
                                unit: SVGMeasurementUnit::Pixel,
                            },
                        ]);
                }
                PaintMixinChange::Blend(mixin) => {
                    let root_element = self.bundle.get_root_mut();
                    root_element.set_attributes(vec![SVGAttribute::Opacity {
                        opacity: mixin.opacity,
                    }]);
                    root_element.set_styles(vec![SVGStyle::BlendMode {
                        blend_mode: map_blend_mode(&mixin.blend_mode),
                    }]);
                }
                _ => {}
            }
        }
    }
}

impl GradientSVGPaint {
    pub fn new(entity: Entity, id_generator: &mut ContinuousId, variant: &GradientVariant) -> Self {
        // Create root element
        let mut element = SVGElement::new(SVGTag::Group, id_generator);
        #[cfg(feature = "tracing")]
        element.set_attribute(SVGAttribute::Name {
            name: GradientSVGPaint::create_element_name(
                element.get_id(),
                String::from("root"),
                false,
            ),
        });
        let mut bundle = BaseSVGBundle::new(element, entity);

        let mut defs_element = SVGElement::new(SVGTag::Defs, id_generator);
        let defs_id = defs_element.get_id();
        #[cfg(feature = "tracing")]
        defs_element.set_attribute(SVGAttribute::Name {
            name: GradientSVGPaint::create_element_name(defs_id, String::from("defs"), false),
        });
        let defs_index = bundle.append_child(defs_element);

        // Create paint elements
        let mut paint_gradient_element = SVGElement::new(SVGTag::Image, id_generator);
        let paint_gradient_id = paint_gradient_element.get_id();
        #[cfg(feature = "tracing")]
        paint_gradient_element.set_attribute(SVGAttribute::Name {
            name: GradientSVGPaint::create_element_name(
                paint_gradient_id,
                String::from("paint-clipped-image"),
                false,
            ),
        });
        let paint_gradient_index = bundle
            .append_child_to(defs_index, paint_gradient_element)
            .unwrap();

        // TODO: Add stops here or in update?

        let mut paint_rect_element = SVGElement::new(SVGTag::Rect, id_generator);
        let paint_rect_id = paint_rect_element.get_id();
        #[cfg(feature = "tracing")]
        paint_rect_element.set_attribute(SVGAttribute::Name {
            name: GradientSVGPaint::create_element_name(
                paint_rect_id,
                String::from("paint-rect"),
                false,
            ),
        });
        paint_rect_element.set_attribute(SVGAttribute::ReferencedFill {
            id: paint_gradient_id,
        });
        let paint_rect_index = bundle.append_child(paint_rect_element);

        Self {
            bundle,
            variant: match variant {
                GradientVariant::Linear { .. } => GradientSVGPaintVariant::Linear,
                GradientVariant::Radial { .. } => GradientSVGPaintVariant::Radial,
                _ => GradientSVGPaintVariant::Unsupported,
            },
            defs: ElementReference {
                // id: defs_id,
                index: defs_index,
            },

            // Paint element references
            paint_gradient: ElementReference {
                // id: paint_gradient_id,
                index: paint_gradient_index,
            },
            paint_gradient_stops: Vec::new(),
            paint_rect: ElementReference {
                // id: paint_rect_id,
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
