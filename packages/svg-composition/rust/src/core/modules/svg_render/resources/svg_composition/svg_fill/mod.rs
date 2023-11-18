use dyn_composition::core::modules::node::components::mixins::{FillMixin, Paint};

use crate::core::events::output_event::RenderUpdateEvent;

use self::svg_paint::{solid_svg_paint::SolidSVGPaint, SVGPaint};

use super::{
    svg_bundle::{BaseSVGBundle, SVGBundle},
    svg_element::{attributes::SVGAttribute, SVGElement, SVGTag},
    svg_node::SVGNode,
    SVGComposition,
};

mod svg_paint;

#[derive(Debug)]
pub struct SVGFill {
    bundle: BaseSVGBundle,
    paints: Vec<Box<dyn SVGPaint>>,
}

impl SVGBundle for SVGFill {
    fn get_bundle(&self) -> &BaseSVGBundle {
        &self.bundle
    }

    fn get_bundle_mut(&mut self) -> &mut BaseSVGBundle {
        &mut self.bundle
    }
}

impl SVGFill {
    pub fn new(parent_element_id: u32, clip_path_id: u32) -> Self {
        // Create root element and apply it to the SVG fill
        let mut element = SVGElement::new(SVGTag::Group);
        #[cfg(feature = "trace")]
        element.set_attribute(SVGAttribute::Name {
            name: SVGFill::create_element_name(element.get_id(), String::from("root"), false),
        });
        element.set_attribute(SVGAttribute::ClipPath {
            clip_path: clip_path_id,
        });
        let bundle = BaseSVGBundle::new(element, Some(parent_element_id));

        Self {
            bundle,
            paints: vec![],
        }
    }

    pub fn get_paint_at(&self, index: usize) -> Option<&Box<dyn SVGPaint>> {
        self.paints.get(index)
    }

    pub fn apply_mixin_change(&mut self, mixin: &FillMixin) {
        let mut updated_paints: Vec<Box<dyn SVGPaint>> = Vec::new();

        for mixin_paint in &mixin.paints {
            match mixin_paint {
                Paint::Solid(solid_paint) => {
                    let mut new_solid_svg_paint =
                        SolidSVGPaint::new(self.bundle.get_element().get_id());
                    new_solid_svg_paint.apply_paint_change(solid_paint);
                    updated_paints.push(Box::new(new_solid_svg_paint));
                }
            }
        }

        self.paints = updated_paints;
    }

    pub fn drain_updates(&mut self) -> Vec<RenderUpdateEvent> {
        let mut updates = self.get_bundle_mut().drain_updates();
        for paint in &mut self.paints {
            updates.extend(paint.get_bundle_mut().drain_updates());
        }
        return updates;
    }

    pub fn to_string(&self, node: &dyn SVGNode, composition: &SVGComposition) -> String {
        self.bundle.to_string(node, composition)
    }

    #[cfg(feature = "trace")]
    fn create_element_name(id: u32, category: String, is_definition: bool) -> String {
        let def_part = if is_definition { "def" } else { "" };
        format!("fill_{}_{}{}", category, id, def_part)
    }
}
