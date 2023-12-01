use crate::core::{
    events::output_event::RenderUpdateEvent,
    modules::svg_render::resources::{
        changed_components::ChangedPaint,
        svg_composition::{svg_bundle::SVGBundle, SVGCompositionRes},
    },
};

use std::fmt::Debug;

pub mod solid_svg_paint;
mod utils;

pub trait SVGPaint: SVGBundle + Sync + Send + Debug {
    fn apply_paint_change(&mut self, changed_paint: &ChangedPaint) -> ();
    fn drain_updates(&mut self) -> Vec<RenderUpdateEvent>;
    fn to_string(&self, composition: &SVGCompositionRes) -> String;
}
