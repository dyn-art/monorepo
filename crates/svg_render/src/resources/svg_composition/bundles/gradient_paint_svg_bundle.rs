use std::collections::BTreeMap;

use bevy_ecs::entity::Entity;
use dyn_composition::{
    modules::node::components::types::{
        GradientPaintVariant, LinearGradientPaintTransform, RadialGradientPaintTransform,
    },
    utils::continuous_id::ContinuousId,
};

use crate::{
    mixin_change::MixinChange,
    resources::{
        changed_entities::{ChangedEntity, ChangedEntityGradientPaintType, ChangedEntityType},
        svg_composition::{
            svg_bundle::SVGBundle,
            svg_context::SVGContext,
            svg_element::{
                attributes::{SVGAttribute, SVGMeasurementUnit, SVGUnitsVariant},
                mapper::map_blend_mode,
                styles::{SVGDisplayStyle, SVGStyle},
                SVGElement, SVGTag,
            },
        },
    },
};

#[derive(Debug)]
pub struct GradientPaintSVGBundle {
    entity: Entity,
    variant: ChangedEntityGradientPaintType,

    root: SVGElement,
    defs: SVGElement,

    // Paint elements
    paint_gradient: SVGElement,
    paint_gradient_stops: Vec<SVGElement>,
    paint_rect: SVGElement,
}

impl SVGBundle for GradientPaintSVGBundle {
    fn get_entity(&self) -> &Entity {
        &self.entity
    }

    fn get_type(&self) -> ChangedEntityType {
        ChangedEntityType::GradientPaint(self.variant)
    }

    fn update(&mut self, changed_entity: ChangedEntity, cx: &mut SVGContext) -> () {
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
                // TODO: Remove this Basic & Internal stuff and instead create a separate mixin for the Internal Mixin
                MixinChange::GradientPaint(mixin) => {
                    match &mixin.variant {
                        GradientPaintVariant::Linear { transform } => match transform {
                            LinearGradientPaintTransform::Internal { start, end, .. } => {
                                self.paint_gradient.set_attributes(vec![
                                    SVGAttribute::X1 { x1: start.x },
                                    SVGAttribute::Y1 { y1: start.y },
                                    SVGAttribute::X2 { x2: end.x },
                                    SVGAttribute::Y2 { y2: end.y },
                                ]);
                            }
                            _ => {}
                        },
                        GradientPaintVariant::Radial { transform } => match transform {
                            RadialGradientPaintTransform::Internal {
                                center,
                                radius,
                                rotation,
                            } => {
                                // TODO
                            }
                            _ => {}
                        },
                    }
                }
                MixinChange::GradientStopsMixin(mixin) => {
                    // Remove old gradient stop elements
                    self.paint_gradient.clear_children();
                    self.paint_gradient_stops
                        .drain(..)
                        .for_each(|mut paint_gradient_stop| {
                            cx.destroy_element(&mut paint_gradient_stop);
                        });

                    // Add new gradient stop elements
                    for gradient_stop in &mixin.gradient_stops {
                        let mut gradient_stop_element = cx.create_element(SVGTag::Stop);
                        gradient_stop_element.set_attributes(vec![
                            SVGAttribute::Offset {
                                offset: gradient_stop.position,
                            },
                            SVGAttribute::StopColor {
                                stop_color: rgb_to_hex(gradient_stop.color),
                            },
                        ]);
                        self.paint_gradient.append_child_in_bundle_context(
                            self.entity,
                            &mut gradient_stop_element,
                        );
                        self.paint_gradient_stops.push(gradient_stop_element);
                    }
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
        children.insert(self.paint_gradient.get_id(), &self.paint_gradient);
        children.insert(self.paint_rect.get_id(), &self.paint_rect);
        self.paint_gradient_stops
            .iter()
            .for_each(|paint_gradient_stop| {
                children.insert(paint_gradient_stop.get_id(), &paint_gradient_stop);
            });
        return children;
    }

    fn get_child_elements_mut(&mut self) -> BTreeMap<ContinuousId, &mut SVGElement> {
        let mut children = BTreeMap::new();
        children.insert(self.defs.get_id(), &mut self.defs);
        children.insert(self.paint_gradient.get_id(), &mut self.paint_gradient);
        children.insert(self.paint_rect.get_id(), &mut self.paint_rect);
        self.paint_gradient_stops
            .iter_mut()
            .for_each(|paint_gradient_stop| {
                children.insert(paint_gradient_stop.get_id(), paint_gradient_stop);
            });
        return children;
    }

    fn get_root_element(&self) -> &SVGElement {
        return &self.root;
    }

    fn get_root_element_mut(&mut self) -> &mut SVGElement {
        return &mut self.root;
    }

    fn to_string(&self, cx: &SVGContext) -> String {
        self.get_root_element().to_string(self, cx)
    }
}

impl GradientPaintSVGBundle {
    pub fn new(
        entity: Entity,
        variant: ChangedEntityGradientPaintType,
        cx: &mut SVGContext,
    ) -> Self {
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

        let mut paint_gradient_element = cx.create_element(match variant {
            ChangedEntityGradientPaintType::Linear => SVGTag::LinearGradient,
            ChangedEntityGradientPaintType::Radial => SVGTag::RadialGradient,
        });
        let paint_gradient_id = paint_gradient_element.get_id();
        #[cfg(feature = "tracing")]
        paint_gradient_element.set_attribute(SVGAttribute::Name {
            name: Self::create_element_name(
                paint_gradient_element.get_id(),
                String::from("paint-gradient"),
                false,
            ),
        });
        paint_gradient_element.set_attribute(SVGAttribute::GradientUnits {
            gradient_units: SVGUnitsVariant::UserSpaceOnUse,
        });
        defs_element.append_child_in_bundle_context(entity, &mut paint_gradient_element);

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
            id: paint_gradient_id,
        });
        root_element.append_child_in_bundle_context(entity, &mut paint_rect_element);

        Self {
            entity,
            variant,
            root: root_element,
            defs: defs_element,

            // Paint elements
            paint_gradient: paint_gradient_element,
            paint_gradient_stops: Vec::new(),
            paint_rect: paint_rect_element,
        }
    }

    #[cfg(feature = "tracing")]
    fn create_element_name(id: ContinuousId, category: String, is_definition: bool) -> String {
        let def_part = if is_definition { "def" } else { "" };
        format!("gradient-paint_{}_{}{}", category, id, def_part)
    }
}

fn rgb_to_hex(rgb: (u8, u8, u8)) -> String {
    format!("#{:02X}{:02X}{:02X}", rgb.0, rgb.1, rgb.2)
}
