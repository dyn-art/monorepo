use crate::core::modules::svg_render::resources::svg_composition::SVGComposition;

use self::base_svg_paint::BaseSVGPaint;
use std::fmt::Debug;

pub mod base_svg_paint;

pub trait SVGPaint: Sync + Send + Debug {
    fn get_base(&self) -> &BaseSVGPaint;
    fn get_base_mut(&mut self) -> &mut BaseSVGPaint;
    fn to_string(&self, composition: &SVGComposition) -> String;
}
