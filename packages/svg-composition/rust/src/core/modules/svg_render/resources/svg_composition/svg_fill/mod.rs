use self::svg_paint::SVGPaint;

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
    pub fn new(parent_id: u32, clip_path_id: u32) -> Self {
        // Create root element and apply it to the SVG fill
        let mut element = SVGElement::new(SVGTag::Group);
        #[cfg(feature = "trace")]
        element.set_attribute(SVGAttribute::Name {
            name: SVGFill::create_element_name(element.get_id(), String::from("root"), false),
        });
        element.set_attribute(SVGAttribute::ClipPath {
            clip_path: clip_path_id,
        });
        let bundle = BaseSVGBundle::new(element, Some(parent_id));

        Self {
            bundle,
            paints: vec![],
        }
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
