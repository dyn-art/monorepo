pub mod no_wrap;
pub mod word_wrap;

use crate::{
    attrs::AttrsIntervals,
    tokens::{line::LineToken, span::SpanToken},
};
use dyn_utils::properties::size::Size;

pub trait LineWrapStrategy {
    fn compute_lines(
        &mut self,
        spans: &[SpanToken],
        attrs_intervals: &AttrsIntervals,
        size: &Size,
    ) -> Vec<LineToken>;
}
