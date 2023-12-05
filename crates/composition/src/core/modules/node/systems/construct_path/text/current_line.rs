use std::ops::RangeBounds;

use super::token_with_shape::TokenWithShape;

pub struct CurrentLine {
    pub tokens: Vec<TokenWithShape>,
    pub current_width: f32,
    pub max_width: f32,
}

impl CurrentLine {
    pub fn new(max_width: f32) -> Self {
        CurrentLine {
            tokens: Vec::new(),
            current_width: 0.0,
            max_width,
        }
    }

    pub fn append(&mut self, mut line_token: TokenWithShape) {
        self.current_width += line_token.get_width();
        self.tokens.push(line_token);
    }

    pub fn drain<R>(&mut self, range: R) -> Vec<TokenWithShape>
    where
        R: RangeBounds<usize>,
    {
        self.current_width = 0.0;
        return self.tokens.drain(range).collect();
    }

    pub fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }
}
