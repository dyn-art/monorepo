use std::{ops::RangeBounds, vec::Drain};

use super::{token::TokenKind, token_with_shape::TokenWithShape};

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

    pub fn drain<R>(&mut self, range: R) -> Drain<'_, TokenWithShape>
    where
        R: RangeBounds<usize>,
    {
        self.current_width = 0.0;
        return self.tokens.drain(range);
    }

    pub fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }

    pub fn compute_line_metric(&self) -> CurrentLineMetric {
        self.tokens.iter().fold(
            CurrentLineMetric {
                height: 0.0,
                max_ascender: 0.0,
            },
            |mut metrics, token_with_shape| {
                match &token_with_shape.token.kind {
                    TokenKind::TextFragment { metric, .. } | TokenKind::Space { metric, .. } => {
                        metrics.height = metrics.height.max(metric.height);
                        metrics.max_ascender = metrics.max_ascender.max(metric.ascender);
                    }
                    _ => {}
                }
                metrics
            },
        )
    }
}

#[derive(Debug)]
pub struct CurrentLineMetric {
    pub height: f32,
    pub max_ascender: f32,
}
