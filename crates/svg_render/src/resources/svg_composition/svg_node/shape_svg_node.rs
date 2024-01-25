use bevy_ecs::entity::Entity;
use dyn_composition::core::utils::continuous_id::ContinuousId;

use crate::{
    events::output_event::ElementChangeEvent,
    mixin_change::NodeMixinChange,
    resources::{
        changed_components::ChangedNode,
        svg_composition::{
            svg_bundle::{BaseSVGBundle, SVGBundle},
            svg_element::{
                attributes::{SVGAttribute, SVGMeasurementUnit},
                helper::{construct_svg_path, mat3_to_svg_transform},
                mapper::map_blend_mode,
                styles::{SVGDisplayStyle, SVGStyle},
                SVGElement, SVGTag,
            },
            svg_node::ElementReference,
            SVGCompositionRes,
        },
    },
};

use super::SVGNode;

#[derive(Debug)]
pub struct ShapeSVGNode {
    bundle: BaseSVGBundle,

    defs: ElementReference,

    // Fill elements
    fill_clip_path: ElementReference,
    fill_clipped_path: ElementReference,
    fill_wrapper_g: ElementReference,

    // Click area elements
    click_area_rect: ElementReference,
}

impl SVGBundle for ShapeSVGNode {
    fn get_bundle(&self) -> &BaseSVGBundle {
        &self.bundle
    }

    fn get_bundle_mut(&mut self) -> &mut BaseSVGBundle {
        &mut self.bundle
    }

    fn drain_changes(&mut self) -> Vec<ElementChangeEvent> {
        self.get_bundle_mut().drain_changes()
    }

    fn to_string(&self, composition: &SVGCompositionRes) -> String {
        self.bundle.to_string(composition)
    }
}

impl SVGNode for ShapeSVGNode {
    fn apply_node_change(&mut self, changed_node: &ChangedNode, _: &mut ContinuousId) {
        for change in &changed_node.changes {
            match change {
                NodeMixinChange::NodeComposition(mixin) => {
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
                NodeMixinChange::Dimension(mixin) => {
                    self.bundle.get_root_mut().set_attributes(vec![
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
                        .get_child_element_mut(self.click_area_rect.index)
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
                NodeMixinChange::RelativeTransform(mixin) => {
                    self.bundle.get_root_mut().set_attributes(vec![
                        (SVGAttribute::Transform {
                            transform: mat3_to_svg_transform(&mixin.relative_transform.0),
                        }),
                    ]);
                }
                NodeMixinChange::Path(mixin) => self
                    .bundle
                    .get_child_element_mut(self.fill_clipped_path.index)
                    .unwrap()
                    .set_attributes(vec![SVGAttribute::D {
                        d: construct_svg_path(&mixin.vertices),
                    }]),
                NodeMixinChange::Blend(mixin) => {
                    let root_element = self.bundle.get_root_mut();
                    root_element.set_attributes(vec![SVGAttribute::Opacity {
                        opacity: mixin.opacity,
                    }]);
                    root_element.set_styles(vec![SVGStyle::BlendMode {
                        blend_mode: map_blend_mode(&mixin.blend_mode),
                    }]);
                }
                _ => {
                    // do nothing
                }
            }
        }
    }

    fn get_node_append_id(&self) -> Option<&ElementReference> {
        None
    }

    fn get_paint_append_id(&self) -> Option<&ElementReference> {
        Some(&self.fill_wrapper_g)
    }
}

impl ShapeSVGNode {
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
        let defs_index = bundle.append_child_element(defs_element);

        // Create click area element
        let mut click_area_rect_element = SVGElement::new(SVGTag::Rect, id_generator);
        let click_area_rect_id = click_area_rect_element.get_id();
        #[cfg(feature = "tracing")]
        click_area_rect_element.set_attributes(vec![
            SVGAttribute::Name {
                name: ShapeSVGNode::create_element_name(
                    click_area_rect_id,
                    String::from("click-area-rect"),
                    false,
                ),
            },
            SVGAttribute::Fill {
                fill: String::from("rgba(255, 204, 203, 0.5)"),
            },
        ]);
        #[cfg(not(feature = "tracing"))]
        click_area_rect_element.set_attribute(SVGAttribute::Fill {
            fill: String::from("transparent"),
        });
        let click_area_rect_index = bundle.append_child_element(click_area_rect_element);

        // Create fill elements
        let mut fill_clip_path_element = SVGElement::new(SVGTag::ClipPath, id_generator);
        let fill_clip_path_id = fill_clip_path_element.get_id();
        #[cfg(feature = "tracing")]
        fill_clip_path_element.set_attribute(SVGAttribute::Name {
            name: ShapeSVGNode::create_element_name(
                fill_clip_path_id,
                String::from("fill-clip-path"),
                true,
            ),
        });
        let fill_clip_path_index = bundle
            .append_child_element_to(defs_index, fill_clip_path_element)
            .unwrap();

        let mut fill_clipped_path_element = SVGElement::new(SVGTag::Path, id_generator);
        #[cfg(feature = "tracing")]
        fill_clipped_path_element.set_attribute(SVGAttribute::Name {
            name: ShapeSVGNode::create_element_name(
                fill_clipped_path_id,
                String::from("fill-clipped-path"),
                false,
            ),
        });
        let fill_clipped_path_index = bundle
            .append_child_element_to(fill_clip_path_index, fill_clipped_path_element)
            .unwrap();

        let mut fill_wrapper_g_element = SVGElement::new(SVGTag::Group, id_generator);
        #[cfg(feature = "tracing")]
        fill_wrapper_g_element.set_attribute(SVGAttribute::Name {
            name: ShapeSVGNode::create_element_name(
                fill_wrapper_g_id,
                String::from("fill-wrapper-g"),
                false,
            ),
        });
        fill_wrapper_g_element.set_attribute(SVGAttribute::ClipPath {
            clip_path: fill_clip_path_id,
        });
        let fill_wrapper_g_index = bundle.append_child_element(fill_wrapper_g_element);

        Self {
            bundle,
            defs: ElementReference { index: defs_index },

            // Click area element references
            click_area_rect: ElementReference {
                index: click_area_rect_index,
            },

            // Fill element references
            fill_clip_path: ElementReference {
                index: fill_clip_path_index,
            },
            fill_clipped_path: ElementReference {
                index: fill_clipped_path_index,
            },
            fill_wrapper_g: ElementReference {
                index: fill_wrapper_g_index,
            },
        }
    }

    #[cfg(feature = "tracing")]
    fn create_element_name(id: ContinuousId, category: String, is_definition: bool) -> String {
        let def_part = if is_definition { "_def" } else { "" };
        format!("shape_{}_{}{}", category, id, def_part)
    }
}
