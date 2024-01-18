use bevy_ecs::entity::Entity;
use dyn_composition::core::utils::continuous_id::ContinuousId;

use crate::{
    events::output_event::ElementUpdateEvent,
    mixin_change::MixinChange,
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

    // Fill elements
    fill_clip_path: ElementReference,
    fill_clip_path_defs: ElementReference,
    fill_clipped_shape: ElementReference,
    fill_wrapper: ElementReference,

    // Click area elements
    click_area: ElementReference,
}

impl SVGBundle for ShapeSVGNode {
    fn get_bundle(&self) -> &BaseSVGBundle {
        &self.bundle
    }

    fn get_bundle_mut(&mut self) -> &mut BaseSVGBundle {
        &mut self.bundle
    }

    fn drain_updates(&mut self) -> Vec<ElementUpdateEvent> {
        self.get_bundle_mut().drain_updates()
    }

    fn to_string(&self, composition: &SVGCompositionRes) -> String {
        self.bundle.to_string(composition)
    }
}

impl SVGNode for ShapeSVGNode {
    fn apply_node_change(&mut self, changed_node: &ChangedNode) {
        for change in &changed_node.changes {
            match change {
                MixinChange::Dimension(mixin) => {
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
                        .get_child_mut(self.click_area.index)
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
                MixinChange::RelativeTransform(mixin) => {
                    self.bundle.get_root_mut().set_attributes(vec![
                        (SVGAttribute::Transform {
                            transform: mat3_to_svg_transform(mixin.relative_transform.0),
                        }),
                    ]);
                }
                MixinChange::Path(mixin) => self
                    .bundle
                    .get_child_mut(self.fill_clipped_shape.index)
                    .unwrap()
                    .set_attributes(vec![SVGAttribute::D {
                        d: construct_svg_path(&mixin.vertices),
                    }]),
                MixinChange::Blend(mixin) => {
                    let root_element = self.bundle.get_root_mut();
                    root_element.set_attributes(vec![SVGAttribute::Opacity {
                        opacity: mixin.opacity,
                    }]);
                    root_element.set_styles(vec![SVGStyle::BlendMode {
                        blend_mode: map_blend_mode(&mixin.blend_mode),
                    }]);
                }
                MixinChange::Composition(mixin) => {
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
        Some(&self.fill_wrapper)
    }
}

impl ShapeSVGNode {
    pub fn new(entity: Entity, id_generator: &mut ContinuousId) -> Self {
        // Create root element
        let mut element = SVGElement::new(SVGTag::Group, id_generator);
        let element_id = element.get_id();
        #[cfg(feature = "tracing")]
        element.set_attribute(SVGAttribute::Name {
            name: ShapeSVGNode::create_element_name(element.get_id(), String::from("root"), false),
        });
        let mut bundle = BaseSVGBundle::new(element, entity);

        // Create click area element
        let mut click_area = SVGElement::new(SVGTag::Rect, id_generator);
        let click_area_id = click_area.get_id();
        #[cfg(feature = "tracing")]
        click_area.set_attributes(vec![
            SVGAttribute::Name {
                name: ShapeSVGNode::create_element_name(
                    click_area_id,
                    String::from("click-area"),
                    false,
                ),
            },
            SVGAttribute::Fill {
                fill: String::from("rgba(255, 204, 203, 0.5)"),
            },
        ]);
        #[cfg(not(feature = "tracing"))]
        click_area.set_attribute(SVGAttribute::Fill {
            fill: String::from("transparent"),
        });
        let click_area_index = bundle.append_child(click_area);

        // Create fill elements
        let mut fill_clip_path_defs = SVGElement::new(SVGTag::Defs, id_generator);
        let fill_clip_path_defs_id = fill_clip_path_defs.get_id();
        #[cfg(feature = "tracing")]
        fill_clip_path_defs.set_attribute(SVGAttribute::Name {
            name: ShapeSVGNode::create_element_name(
                fill_clip_path_defs_id,
                String::from("fill-defs"),
                false,
            ),
        });
        let fill_clip_path_defs_index = bundle.append_child(fill_clip_path_defs);

        let mut fill_clip_path_element = SVGElement::new(SVGTag::ClipPath, id_generator);
        let fill_clip_path_id = fill_clip_path_element.get_id();
        #[cfg(feature = "tracing")]
        fill_clip_path_element.set_attribute(SVGAttribute::Name {
            name: ShapeSVGNode::create_element_name(
                fill_clip_path_id,
                String::from("fill-clip"),
                true,
            ),
        });
        let fill_clip_path_index = bundle
            .append_child_to(fill_clip_path_defs_index, fill_clip_path_element)
            .unwrap();

        let mut fill_clipped_shape_element = SVGElement::new(SVGTag::Path, id_generator);
        let fill_clipped_shape_id = fill_clipped_shape_element.get_id();
        #[cfg(feature = "tracing")]
        fill_clipped_shape_element.set_attribute(SVGAttribute::Name {
            name: ShapeSVGNode::create_element_name(
                fill_clipped_shape_id,
                String::from("fill-clipped-shape"),
                false,
            ),
        });
        let fill_clipped_shape_index = bundle
            .append_child_to(fill_clip_path_index, fill_clipped_shape_element)
            .unwrap();

        let mut fill_wrapper_element = SVGElement::new(SVGTag::Group, id_generator);
        let fill_wrapper_id = fill_wrapper_element.get_id();
        #[cfg(feature = "tracing")]
        fill_wrapper_element.set_attribute(SVGAttribute::Name {
            name: ShapeSVGNode::create_element_name(fill_wrapper_id, String::from("fill"), false),
        });
        fill_wrapper_element.set_attribute(SVGAttribute::ClipPath {
            clip_path: fill_clip_path_id,
        });
        let fill_wrapper_index = bundle.append_child(fill_wrapper_element);

        Self {
            bundle,

            // Click area element references
            click_area: ElementReference {
                id: click_area_id,
                index: click_area_index,
            },

            // Fill element references
            fill_clip_path_defs: ElementReference {
                id: fill_clip_path_defs_id,
                index: fill_clip_path_defs_index,
            },
            fill_clip_path: ElementReference {
                id: fill_clip_path_id,
                index: fill_clip_path_index,
            },
            fill_clipped_shape: ElementReference {
                id: fill_clipped_shape_id,
                index: fill_clipped_shape_index,
            },
            fill_wrapper: ElementReference {
                id: fill_wrapper_id,
                index: fill_wrapper_index,
            },
        }
    }

    #[cfg(feature = "tracing")]
    fn create_element_name(id: ContinuousId, category: String, is_definition: bool) -> String {
        let def_part = if is_definition { "def" } else { "" };
        format!("shape_{}_{}{}", category, id, def_part)
    }
}
