use super::Token;
use std::ops::Range;

#[derive(Debug, Clone)]
pub struct LinebreakToken {
    range: Range<usize>,
}

impl LinebreakToken {
    pub fn new(range: Range<usize>) -> Self {
        Self { range }
    }
}

impl Token for LinebreakToken {
    fn get_range(&self) -> &Range<usize> {
        &self.range
    }
}
