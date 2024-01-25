use std::fmt::Debug;

use dyn_composition::core::utils::continuous_id::ContinuousId;

use crate::resources::changed_components::ChangedPaint;

use super::svg_bundle::SVGBundle;

pub mod gradient_svg_paint;
pub mod image_svg_paint;
pub mod solid_svg_paint;
mod utils;

pub trait SVGPaint: SVGBundle + Sync + Send + Debug {
    fn apply_paint_change(
        &mut self,
        changed_paint: &ChangedPaint,
        id_generator: &mut ContinuousId,
    ) -> ();
}
