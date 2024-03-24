use super::ShapeToken;
use std::ops::Range;

#[derive(Debug, Clone)]
pub struct LinebreakToken {
    range: Range<usize>,
}

impl LinebreakToken {
    pub fn new(range: Range<usize>) -> Self {
        log::info!("LinebreakToken for range: {:?}", range);
        Self { range }
    }
}

impl ShapeToken for LinebreakToken {
    fn get_range(&self) -> &Range<usize> {
        &self.range
    }
}
