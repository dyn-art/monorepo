use crate::core::modules::svg_render::resources::{
    changed_components::ChangedPaint, svg_composition::svg_bundle::SVGBundle,
};

use std::fmt::Debug;

pub mod solid_svg_paint;
mod utils;

pub trait SVGPaint: SVGBundle + Sync + Send + Debug {
    fn apply_paint_change(&mut self, changed_paint: &ChangedPaint) -> ();
}
