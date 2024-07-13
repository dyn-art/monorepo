use dyn_composition::modules::node::components::mixins::BlendMode;

use super::styles::SVGBlendMode;

pub fn map_blend_mode(blend_mode: &BlendMode) -> SVGBlendMode {
    match blend_mode {
        BlendMode::Normal => SVGBlendMode::Normal,
        BlendMode::Multiply => SVGBlendMode::Multiply,
        BlendMode::Screen => SVGBlendMode::Screen,
        BlendMode::Overlay => SVGBlendMode::Overlay,
        BlendMode::Darken => SVGBlendMode::Darken,
        BlendMode::Lighten => SVGBlendMode::Lighten,
        BlendMode::ColorDodge => SVGBlendMode::ColorDodge,
        BlendMode::ColorBurn => SVGBlendMode::ColorBurn,
        BlendMode::HardLight => SVGBlendMode::HardLight,
        BlendMode::SoftLight => SVGBlendMode::SoftLight,
        BlendMode::Difference => SVGBlendMode::Difference,
        BlendMode::Exclusion => SVGBlendMode::Exclusion,
        BlendMode::Hue => SVGBlendMode::Hue,
        BlendMode::Saturation => SVGBlendMode::Saturation,
        BlendMode::Color => SVGBlendMode::Color,
        BlendMode::Luminosity => SVGBlendMode::Luminosity,
    }
}
