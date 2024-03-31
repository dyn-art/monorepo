pub mod no_wrap;
pub mod word_wrap;

use crate::{line::Line, span::SpanIntervals};
use dyn_utils::properties::size::Size;

pub trait LineWrapStrategy {
    fn compute_lines(
        &mut self,
        spans: &SpanIntervals,
        size: &Size,
        text: &str, // For debugging
    ) -> Vec<Line>;
}
