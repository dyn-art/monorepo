use crate::core::modules::node::systems::construct_path::text::{
    current_line::CurrentLine, token::TokenKind, token_with_shape::TokenWithShape,
};

use super::{LineBreakBehavior, LineBreakStrategy, ShouldBreakLine};

pub struct BreakOnWordLineBreakStrategy2 {
    start_overflow_index: Option<usize>,
}

// TODO: Endless loop
impl BreakOnWordLineBreakStrategy2 {
    pub fn new() -> Self {
        Self {
            start_overflow_index: None,
        }
    }

    // Determines the starting index for overflow in a line of tokens.
    fn get_start_overflow_index(&self, current_line: &CurrentLine) -> Option<usize> {
        current_line
            .tokens
            .iter()
            .enumerate()
            .rfind(|(_, token)| !matches!(token.token.kind, TokenKind::TextFragment { .. }))
            .map(|(index, _)| index + 1)
            .or(Some(0))
    }

    // Determines the behavior when a line break might be necessary.
    fn determine_line_break_behavior(
        &mut self,
        exceeds_width: bool,
        current_line: &mut CurrentLine,
        next_token: &TokenWithShape,
    ) -> ShouldBreakLine {
        if exceeds_width && !current_line.is_empty() {
            if let Some(start_overflow_index) = self.start_overflow_index.take() {
                // Collect the tokens that need to be moved to the next line
                let mut overflown_tokens: Vec<TokenWithShape> = current_line
                    .tokens
                    .drain(start_overflow_index..)
                    .filter(|token_with_shape| {
                        matches!(token_with_shape.token.kind, TokenKind::TextFragment { .. })
                    })
                    .collect();

                if self.can_break_line(&mut overflown_tokens, current_line.max_width) {
                    return ShouldBreakLine::True(LineBreakBehavior::AppendOverflownTokens(
                        overflown_tokens,
                    ));
                } else {
                    return ShouldBreakLine::False;
                }
            } else {
                if !matches!(next_token.token.kind, TokenKind::Space { .. }) {
                    return ShouldBreakLine::True(LineBreakBehavior::AppendNextToken);
                } else {
                    return ShouldBreakLine::True(LineBreakBehavior::None);
                }
            }
        } else {
            return ShouldBreakLine::False;
        }
    }

    // Checks if breaking the line with the given tokens will not cause an endless loop.
    fn can_break_line(&self, tokens: &mut [TokenWithShape], max_width: f32) -> bool {
        tokens
            .iter_mut()
            .map(|token_with_shape| token_with_shape.get_width())
            .sum::<f32>()
            < max_width
    }
}

impl LineBreakStrategy for BreakOnWordLineBreakStrategy2 {
    fn should_break(
        &mut self,
        current_line: &mut CurrentLine,
        next_token_in_line: &mut TokenWithShape,
        is_last_token: bool,
    ) -> ShouldBreakLine {
        let exceeds_width =
            current_line.current_width + next_token_in_line.get_width() > current_line.max_width;

        // Set overflow index if needed
        if exceeds_width && self.start_overflow_index.is_none() {
            self.start_overflow_index = self.get_start_overflow_index(current_line);
        }

        let should_break = match next_token_in_line.token.kind {
            TokenKind::TextFragment { .. } if is_last_token || exceeds_width => true,
            _ => exceeds_width,
        };

        return if should_break {
            self.determine_line_break_behavior(exceeds_width, current_line, next_token_in_line)
        } else {
            ShouldBreakLine::False
        };
    }
}
