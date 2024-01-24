use bevy_ecs::entity::Entity;
use dyn_composition::core::{
    modules::node::components::{
        mixins::ImageContent,
        types::{ImageFillFitPaintTransform, ImagePaintScaleMode, ImageTilePaintTransform},
    },
    utils::continuous_id::ContinuousId,
};

use crate::{
    events::output_event::ElementChangeEvent,
    mixin_change::PaintMixinChange,
    resources::{
        changed_components::ChangedPaint,
        svg_composition::{
            svg_bundle::{BaseSVGBundle, SVGBundle},
            svg_element::{
                attributes::{
                    SVGAttribute, SVGHrefVariant, SVGMeasurementUnit, SVGPatternUnitsVariant,
                    SVGTransformAttribute,
                },
                helper::mat3_to_svg_transform,
                mapper::map_blend_mode,
                styles::{SVGDisplayStyle, SVGStyle},
                SVGElement, SVGTag,
            },
            svg_node::ElementReference,
            SVGCompositionRes,
        },
    },
};
use base64::prelude::*;

use super::SVGPaint;

#[derive(Debug)]
pub struct ImageSVGPaint {
    bundle: BaseSVGBundle,
    variant: ImageSVGPaintVariant,

    defs: ElementReference,

    // Paint elements
    paint_pattern: ElementReference,
    paint_clipped_image: ElementReference,
    paint_rect: ElementReference,
}

#[derive(Debug)]
enum ImageSVGPaintVariant {
    Fill,
    Fit,
    Crop,
    Tile,
}

impl SVGBundle for ImageSVGPaint {
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

impl SVGPaint for ImageSVGPaint {
    fn apply_paint_change(&mut self, changed_paint: &ChangedPaint) {
        for change in &changed_paint.changes {
            match change {
                PaintMixinChange::ImagePaint(mixin) => match &mixin.scale_mode {
                    ImagePaintScaleMode::Fill { transform }
                    | ImagePaintScaleMode::Fit { transform } => match transform {
                        ImageFillFitPaintTransform::Render { transform } => {
                            self.bundle
                                .get_child_mut(self.paint_clipped_image.index)
                                .unwrap()
                                .set_attribute(SVGAttribute::Transform {
                                    transform: mat3_to_svg_transform(transform),
                                });
                        }
                        _ => {}
                    },
                    ImagePaintScaleMode::Tile { transform } => match transform {
                        ImageTilePaintTransform::Render {
                            rotation,
                            tile_width,
                            tile_height,
                        } => {
                            self.bundle
                                .get_child_mut(self.paint_pattern.index)
                                .unwrap()
                                .set_attributes(vec![
                                    SVGAttribute::PatternTransform {
                                        transform: SVGTransformAttribute::Rotate {
                                            rotation: *rotation,
                                        },
                                    },
                                    SVGAttribute::Width {
                                        width: *tile_width,
                                        unit: SVGMeasurementUnit::Pixel,
                                    },
                                    SVGAttribute::Height {
                                        height: *tile_height,
                                        unit: SVGMeasurementUnit::Pixel,
                                    },
                                ]);
                            self.bundle
                                .get_child_mut(self.paint_clipped_image.index)
                                .unwrap()
                                .set_attributes(vec![
                                    SVGAttribute::Width {
                                        width: *tile_width,
                                        unit: SVGMeasurementUnit::Pixel,
                                    },
                                    SVGAttribute::Height {
                                        height: *tile_height,
                                        unit: SVGMeasurementUnit::Pixel,
                                    },
                                ]);
                        }
                        _ => {}
                    },
                    ImagePaintScaleMode::Crop { transform } => {
                        self.bundle
                            .get_child_mut(self.paint_clipped_image.index)
                            .unwrap()
                            .set_attribute(SVGAttribute::Transform {
                                transform: mat3_to_svg_transform(transform),
                            });
                    }
                    _ => {}
                },
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

                    match self.variant {
                        ImageSVGPaintVariant::Fill
                        | ImageSVGPaintVariant::Fit
                        | ImageSVGPaintVariant::Crop => {
                            self.bundle
                                .get_child_mut(self.paint_pattern.index)
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
                            self.bundle
                                .get_child_mut(self.paint_clipped_image.index)
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
                        _ => {}
                    }
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
                PaintMixinChange::ImageContent(mixin) => {
                    self.bundle
                        .get_child_mut(self.paint_clipped_image.index)
                        .unwrap()
                        .set_attribute(SVGAttribute::Href {
                            href: match &mixin.content {
                                ImageContent::Binary { content } => SVGHrefVariant::Base64 {
                                    content: BASE64_STANDARD.encode(content),
                                },
                                ImageContent::Url { url } => {
                                    SVGHrefVariant::Url { url: url.clone() }
                                }
                            },
                        });
                }
                _ => {}
            }
        }
    }
}

impl ImageSVGPaint {
    pub fn new(
        entity: Entity,
        id_generator: &mut ContinuousId,
        scale_mode: &ImagePaintScaleMode,
    ) -> Self {
        // Create root element
        let mut element = SVGElement::new(SVGTag::Group, id_generator);
        #[cfg(feature = "tracing")]
        element.set_attribute(SVGAttribute::Name {
            name: ImageSVGPaint::create_element_name(element.get_id(), String::from("root"), false),
        });
        let mut bundle = BaseSVGBundle::new(element, entity);

        let mut defs_element = SVGElement::new(SVGTag::Defs, id_generator);
        let defs_id = defs_element.get_id();
        #[cfg(feature = "tracing")]
        defs_element.set_attribute(SVGAttribute::Name {
            name: ImageSVGPaint::create_element_name(defs_id, String::from("defs"), false),
        });
        let defs_index = bundle.append_child(defs_element);

        // Create paint elements
        let mut paint_pattern_element = SVGElement::new(SVGTag::Pattern, id_generator);
        let paint_pattern_id = paint_pattern_element.get_id();
        #[cfg(feature = "tracing")]
        paint_pattern_element.set_attribute(SVGAttribute::Name {
            name: ImageSVGPaint::create_element_name(
                paint_pattern_id,
                String::from("paint-pattern"),
                true,
            ),
        });
        paint_pattern_element.set_attribute(SVGAttribute::PatternUnits {
            pattern_units: SVGPatternUnitsVariant::UserSpaceOnUse,
        });
        let paint_pattern_index = bundle
            .append_child_to(defs_index, paint_pattern_element)
            .unwrap();

        let mut paint_clipped_image_element = SVGElement::new(SVGTag::Image, id_generator);
        let paint_clipped_image_id = paint_clipped_image_element.get_id();
        #[cfg(feature = "tracing")]
        paint_clipped_image_element.set_attribute(SVGAttribute::Name {
            name: ImageSVGPaint::create_element_name(
                paint_clipped_image_id,
                String::from("paint-clipped-image"),
                false,
            ),
        });
        match scale_mode {
            ImagePaintScaleMode::Fill { .. } => {
                paint_clipped_image_element.set_attribute(SVGAttribute::PreserveAspectRatio {
                    preserve_aspect_ratio: String::from("xMidYMid slice"),
                });
            }
            _ => {}
        }
        let paint_clipped_image_index = bundle
            .append_child_to(paint_pattern_index, paint_clipped_image_element)
            .unwrap();

        let mut paint_rect_element = SVGElement::new(SVGTag::Rect, id_generator);
        let paint_rect_id = paint_rect_element.get_id();
        #[cfg(feature = "tracing")]
        paint_rect_element.set_attribute(SVGAttribute::Name {
            name: ImageSVGPaint::create_element_name(
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
            variant: match scale_mode {
                ImagePaintScaleMode::Fill { .. } => ImageSVGPaintVariant::Fill,
                ImagePaintScaleMode::Fit { .. } => ImageSVGPaintVariant::Fit,
                ImagePaintScaleMode::Crop { .. } => ImageSVGPaintVariant::Crop,
                ImagePaintScaleMode::Tile { .. } => ImageSVGPaintVariant::Tile,
            },
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
