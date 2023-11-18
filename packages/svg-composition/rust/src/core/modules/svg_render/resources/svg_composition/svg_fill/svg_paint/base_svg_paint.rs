use crate::core::modules::svg_render::resources::svg_composition::svg_element::SVGElement;

#[derive(Debug)]
pub struct BaseSVGPaint {
    id: u32,
    element: SVGElement, // Paint Wrapper
    child_elements: Vec<SVGElement>,
}
