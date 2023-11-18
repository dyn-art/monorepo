use crate::core::{
    events::output_event::RenderUpdateEvent,
    mixin_change::MixinChange,
    modules::svg_render::resources::svg_composition::{
        svg_bundle::{BaseSVGBundle, SVGBundle},
        svg_element::{
            attributes::SVGAttribute,
            helper::{construct_svg_path, mat3_to_svg_transform},
            mapper::map_blend_mode,
            styles::{SVGDisplayStyle, SVGStyle},
            SVGChildElementIdentifier, SVGElement, SVGTag,
        },
        svg_fill::SVGFill,
        SVGComposition,
    },
};

use super::{ElementReference, SVGNode};

#[derive(Debug)]
pub struct ShapeSVGNode {
    bundle: BaseSVGBundle,

    // Fill elements
    fill_clip_path: ElementReference,
    fill_clip_path_defs: ElementReference,
    fill_clipped_shape: ElementReference,

    fill: SVGFill,
}

impl SVGBundle for ShapeSVGNode {
    fn get_bundle(&self) -> &BaseSVGBundle {
        &self.bundle
    }

    fn get_bundle_mut(&mut self) -> &mut BaseSVGBundle {
        &mut self.bundle
    }
}

impl SVGNode for ShapeSVGNode {
    fn apply_mixin_changes(&mut self, changes: &[MixinChange]) {
        for change in changes {
            match change {
                MixinChange::Dimension(mixin) => {
                    self.bundle.set_attributes(vec![
                        SVGAttribute::Width { width: mixin.width },
                        SVGAttribute::Height {
                            height: mixin.height,
                        },
                    ]);
                }
                MixinChange::RelativeTransform(mixin) => {
                    self.bundle.set_attributes(vec![
                        (SVGAttribute::Transform {
                            transform: mat3_to_svg_transform(mixin.relative_transform.0),
                        }),
                    ]);
                }
                MixinChange::Path(mixin) => {
                    let fill_clipped_shape_index = self.fill_clipped_shape.index;
                    self.bundle.set_attributes_at(
                        fill_clipped_shape_index,
                        vec![SVGAttribute::D {
                            d: construct_svg_path(&mixin.vertices),
                        }],
                    )
                }
                MixinChange::Blend(mixin) => {
                    self.bundle.set_attributes(vec![SVGAttribute::Opacity {
                        opacity: mixin.opacity,
                    }]);
                    self.bundle.set_styles(vec![SVGStyle::BlendMode {
                        blend_mode: map_blend_mode(&mixin.blend_mode),
                    }]);
                }
                MixinChange::Composition(mixin) => {
                    self.bundle.set_styles(vec![SVGStyle::Display {
                        display: if mixin.is_visible {
                            SVGDisplayStyle::Block
                        } else {
                            SVGDisplayStyle::None
                        },
                    }]);
                }
                MixinChange::Fill(mixin) => {
                    self.fill.apply_mixin_change(mixin);
                }
                _ => {
                    // do nothing
                }
            }
        }
    }

    fn get_external_child_append_id(&self) -> Option<&ElementReference> {
        None
    }

    fn drain_updates(&mut self) -> Vec<RenderUpdateEvent> {
        let mut updates = self.get_bundle_mut().drain_updates();
        updates.extend(self.fill.drain_updates());
        return updates;
    }

    fn get_fill(&self) -> Option<&SVGFill> {
        Some(&self.fill)
    }

    fn to_string(&self, composition: &SVGComposition) -> String {
        self.bundle.to_string(self, composition)
    }
}

impl ShapeSVGNode {
    pub fn new(maybe_parent_element_id: Option<u32>) -> Self {
        // Create root element and apply it to SVG node
        let mut element = SVGElement::new(SVGTag::Group);
        let element_id = element.get_id();
        #[cfg(feature = "trace")]
        element.set_attribute(SVGAttribute::Name {
            name: ShapeSVGNode::create_element_name(element.get_id(), String::from("root"), false),
        });
        let mut bundle = BaseSVGBundle::new(element, maybe_parent_element_id);

        // Create fill elements
        let mut fill_clip_path_defs = SVGElement::new(SVGTag::Defs);
        let fill_clip_path_defs_id = fill_clip_path_defs.get_id();
        #[cfg(feature = "trace")]
        fill_clip_path_defs.set_attribute(SVGAttribute::Name {
            name: ShapeSVGNode::create_element_name(
                fill_clip_path_defs_id,
                String::from("fill-defs"),
                false,
            ),
        });
        let fill_clip_path_defs_index = bundle.append_child_element(fill_clip_path_defs);

        let mut fill_clip_path_element = SVGElement::new(SVGTag::ClipPath);
        let fill_clip_path_id = fill_clip_path_element.get_id();
        #[cfg(feature = "trace")]
        fill_clip_path_element.set_attribute(SVGAttribute::Name {
            name: ShapeSVGNode::create_element_name(
                fill_clip_path_id,
                String::from("fill-clip"),
                true,
            ),
        });
        let fill_clip_path_index = bundle
            .append_child_element_to(fill_clip_path_defs_index, fill_clip_path_element)
            .unwrap();

        let mut fill_clipped_shape_element = SVGElement::new(SVGTag::Path);
        let fill_clipped_shape_id = fill_clipped_shape_element.get_id();
        #[cfg(feature = "trace")]
        fill_clipped_shape_element.set_attribute(SVGAttribute::Name {
            name: ShapeSVGNode::create_element_name(
                fill_clipped_shape_id,
                String::from("fill-clipped-shape"),
                false,
            ),
        });
        let fill_clipped_shape_index = bundle
            .append_child_element_to(fill_clip_path_index, fill_clipped_shape_element)
            .unwrap();

        // Create and append fill to node
        let fill = SVGFill::new(element_id, fill_clip_path_id);
        bundle
            .get_element_mut()
            .append_child(SVGChildElementIdentifier::Fill);

        Self {
            bundle,

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

            fill,
        }
    }

    #[cfg(feature = "trace")]
    fn create_element_name(id: u32, category: String, is_definition: bool) -> String {
        let def_part = if is_definition { "def" } else { "" };
        format!("shape_{}_{}{}", category, id, def_part)
    }
}
