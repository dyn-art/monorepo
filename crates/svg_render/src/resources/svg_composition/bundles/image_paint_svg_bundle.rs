use std::collections::BTreeMap;

use base64::prelude::*;
use bevy_ecs::entity::Entity;
use dyn_composition::{
    modules::node::components::mixins::ImageContent, utils::continuous_id::ContinuousId,
};

use crate::{
    components::SVGImagePaintScaleMode,
    mixin_change::MixinChange,
    resources::{
        changed_entities::{ChangedEntity, ChangedEntityImagePaintType, ChangedEntityType},
        svg_composition::{
            svg_bundle::SVGBundle,
            svg_context::SVGContext,
            svg_element::{
                attributes::{
                    SVGAttribute, SVGHrefVariant, SVGMeasurementUnit, SVGTransformAttribute,
                    SVGUnitsVariant,
                },
                mapper::{map_blend_mode, map_mat3_to_svg_transform},
                styles::{SVGDisplayStyle, SVGStyle},
                SVGElement, SVGTag,
            },
        },
    },
};

#[derive(Debug)]
pub struct ImagePaintSVGBundle {
    entity: Entity,
    variant: ChangedEntityImagePaintType,

    root: SVGElement,
    defs: SVGElement,

    // Paint elements
    paint_pattern: SVGElement,
    paint_clipped_image: SVGElement,
    paint_rect: SVGElement,
}

impl SVGBundle for ImagePaintSVGBundle {
    fn get_entity(&self) -> &Entity {
        &self.entity
    }

    fn get_type(&self) -> ChangedEntityType {
        ChangedEntityType::ImagePaint(self.variant)
    }

    fn update(&mut self, changed_entity: ChangedEntity, _: &mut SVGContext) -> () {
        for change in &changed_entity.changes {
            match change {
                MixinChange::PaintComposition(mixin) => {
                    self.root.set_styles(vec![SVGStyle::Display {
                        display: if mixin.is_visible {
                            SVGDisplayStyle::Block
                        } else {
                            SVGDisplayStyle::None
                        },
                    }]);
                }
                MixinChange::ImagePaint(mixin) => match &mixin.scale_mode {
                    SVGImagePaintScaleMode::Tile {
                        rotation,
                        tile_width,
                        tile_height,
                    } => {
                        self.paint_pattern.set_attributes(vec![
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
                        self.paint_clipped_image.set_attributes(vec![
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
                    SVGImagePaintScaleMode::Crop {
                        transform,
                        image_width,
                        image_height,
                    } => {
                        self.paint_pattern.set_attributes(vec![
                            SVGAttribute::Width {
                                width: *image_width,
                                unit: SVGMeasurementUnit::Pixel,
                            },
                            SVGAttribute::Height {
                                height: *image_height,
                                unit: SVGMeasurementUnit::Pixel,
                            },
                        ]);
                        self.paint_clipped_image.set_attributes(vec![
                            SVGAttribute::Transform {
                                transform: map_mat3_to_svg_transform(transform),
                            },
                            SVGAttribute::Width {
                                width: *image_width,
                                unit: SVGMeasurementUnit::Pixel,
                            },
                            SVGAttribute::Height {
                                height: *image_height,
                                unit: SVGMeasurementUnit::Pixel,
                            },
                        ]);
                    }
                    _ => {}
                },
                MixinChange::ImageContent(mixin) => {
                    self.paint_clipped_image.set_attribute(SVGAttribute::Href {
                        href: match &mixin.content {
                            ImageContent::Binary { content } => SVGHrefVariant::Base64 {
                                content: BASE64_STANDARD.encode(content),
                            },
                            ImageContent::Url { url } => SVGHrefVariant::Url { url: url.clone() },
                        },
                    });
                }
                MixinChange::Dimension(mixin) => {
                    self.paint_rect.set_attributes(vec![
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
                        ChangedEntityImagePaintType::Fill | ChangedEntityImagePaintType::Fit => {
                            self.paint_pattern.set_attributes(vec![
                                SVGAttribute::Width {
                                    width: mixin.width,
                                    unit: SVGMeasurementUnit::Pixel,
                                },
                                SVGAttribute::Height {
                                    height: mixin.height,
                                    unit: SVGMeasurementUnit::Pixel,
                                },
                            ]);
                            self.paint_clipped_image.set_attributes(vec![
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
                MixinChange::Blend(mixin) => {
                    self.root.set_attributes(vec![SVGAttribute::Opacity {
                        opacity: mixin.opacity,
                    }]);
                    self.root.set_styles(vec![SVGStyle::BlendMode {
                        blend_mode: map_blend_mode(&mixin.blend_mode),
                    }]);
                }
                _ => {}
            }
        }
    }

    fn get_child_elements(&self) -> BTreeMap<ContinuousId, &SVGElement> {
        let mut children = BTreeMap::new();
        children.insert(self.defs.get_id(), &self.defs);
        children.insert(self.paint_pattern.get_id(), &self.paint_pattern);
        children.insert(self.paint_clipped_image.get_id(), &self.paint_clipped_image);
        children.insert(self.paint_rect.get_id(), &self.paint_rect);
        return children;
    }

    fn get_child_elements_mut(&mut self) -> BTreeMap<ContinuousId, &mut SVGElement> {
        let mut children = BTreeMap::new();
        children.insert(self.defs.get_id(), &mut self.defs);
        children.insert(self.paint_pattern.get_id(), &mut self.paint_pattern);
        children.insert(
            self.paint_clipped_image.get_id(),
            &mut self.paint_clipped_image,
        );
        children.insert(self.paint_rect.get_id(), &mut self.paint_rect);
        return children;
    }

    fn get_root_element(&self) -> &SVGElement {
        return &self.root;
    }

    fn get_root_element_mut(&mut self) -> &mut SVGElement {
        return &mut self.root;
    }

    fn get_child_entities(&self) -> Vec<Entity> {
        Vec::new()
    }

    fn destroy(&mut self, cx: &mut SVGContext) {
        // Destroy elements associated with the bundle.
        // Removing the root also implicitly removes its child elements.
        cx.destroy_element(self.get_root_element_mut());
    }

    fn to_string(&self, cx: &SVGContext) -> String {
        self.get_root_element().to_string(self, cx)
    }
}

impl ImagePaintSVGBundle {
    pub fn new(entity: Entity, variant: ChangedEntityImagePaintType, cx: &mut SVGContext) -> Self {
        let mut root_element = cx.create_bundle_root_element(SVGTag::Group, entity);
        #[cfg(feature = "tracing")]
        root_element.set_attribute(SVGAttribute::Name {
            name: Self::create_element_name(root_element.get_id(), String::from("root"), false),
        });

        let mut defs_element = cx.create_element(SVGTag::Defs);
        #[cfg(feature = "tracing")]
        defs_element.set_attribute(SVGAttribute::Name {
            name: Self::create_element_name(defs_element.get_id(), String::from("defs"), false),
        });
        root_element.append_child_in_bundle_context(entity, &mut defs_element);

        // Create paint elements

        let mut paint_pattern_element = cx.create_element(SVGTag::Pattern);
        let paint_pattern_id = paint_pattern_element.get_id();
        #[cfg(feature = "tracing")]
        paint_pattern_element.set_attribute(SVGAttribute::Name {
            name: Self::create_element_name(
                paint_pattern_element.get_id(),
                String::from("paint-pattern"),
                true,
            ),
        });
        paint_pattern_element.set_attribute(SVGAttribute::PatternUnits {
            pattern_units: SVGUnitsVariant::UserSpaceOnUse,
        });
        defs_element.append_child_in_bundle_context(entity, &mut paint_pattern_element);

        let mut paint_clipped_image_element = cx.create_element(SVGTag::Image);
        #[cfg(feature = "tracing")]
        paint_clipped_image_element.set_attribute(SVGAttribute::Name {
            name: Self::create_element_name(
                paint_clipped_image_element.get_id(),
                String::from("paint-clipped-image"),
                false,
            ),
        });
        match variant {
            ChangedEntityImagePaintType::Fill => {
                paint_clipped_image_element.set_attribute(SVGAttribute::PreserveAspectRatio {
                    preserve_aspect_ratio: String::from("xMidYMid slice"),
                });
            }
            _ => {}
        }
        paint_pattern_element
            .append_child_in_bundle_context(entity, &mut paint_clipped_image_element);

        let mut paint_rect_element = cx.create_element(SVGTag::Rect);
        #[cfg(feature = "tracing")]
        paint_rect_element.set_attribute(SVGAttribute::Name {
            name: Self::create_element_name(
                paint_rect_element.get_id(),
                String::from("paint-rect"),
                false,
            ),
        });
        paint_rect_element.set_attribute(SVGAttribute::ReferencedFill {
            id: paint_pattern_id,
        });
        root_element.append_child_in_bundle_context(entity, &mut paint_rect_element);

        Self {
            entity,
            variant,
            root: root_element,
            defs: defs_element,

            // Paint elements
            paint_pattern: paint_pattern_element,
            paint_clipped_image: paint_clipped_image_element,
            paint_rect: paint_rect_element,
        }
    }

    #[cfg(feature = "tracing")]
    fn create_element_name(id: ContinuousId, category: String, is_definition: bool) -> String {
        let def_part = if is_definition { "def" } else { "" };
        format!("image-paint_{}_{}{}", category, id, def_part)
    }
}
