use std::fmt::Debug;

use crate::resources::changed_components::ChangedPaint;

use super::svg_bundle::SVGBundle;

pub mod solid_svg_paint;
mod utils;

pub trait SVGPaint: SVGBundle + Sync + Send + Debug {
    fn apply_paint_change(&mut self, changed_paint: &ChangedPaint) -> ();
}
