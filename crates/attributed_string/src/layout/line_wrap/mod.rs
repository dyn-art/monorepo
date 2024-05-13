pub mod no_wrap;
pub mod word_wrap;

use crate::span::SpanIntervals;
use dyn_utils::properties::size::Size;

use super::line::Line;

pub trait LineWrapStrategy {
    fn compute_lines(&mut self, spans: &SpanIntervals, size: &Size) -> Vec<Line>;
}
