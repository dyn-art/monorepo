use crate::modules::node::systems::construct_path::text::{
    current_line::CurrentLine, token::TokenKind, token_with_shape::TokenWithShape,
};

use super::{LineBreakBehavior, LineBreakStrategy, ShouldBreakLine};

pub struct BreakOnWordLineBreakStrategy {
    start_overflow_index: Option<usize>,
}

impl BreakOnWordLineBreakStrategy {
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

    // Determines the behavior when a line break should be necessary.
    fn determine_line_break_behavior(
        &mut self,
        current_line: &mut CurrentLine,
        next_token: &mut TokenWithShape,
    ) -> ShouldBreakLine {
        // Case when a previous overflow index was set
        // then collect overflown tokens and append them to the next line
        if let Some(start_overflow_index) = self.start_overflow_index.take() {
            // Collect the tokens that should be moved to the next line
            let mut overflown_tokens: Vec<TokenWithShape> = current_line
                .tokens
                .drain(start_overflow_index..)
                .filter(|token_with_shape| {
                    matches!(token_with_shape.token.kind, TokenKind::TextFragment { .. })
                })
                .collect();

            // info!(
            //     "deterime_line_break_behavior: {:?}[{:?}] - {:?}",
            //     overflown_tokens
            //         .iter()
            //         .map(|token_with_shape| token_with_shape.token.get_str())
            //         .collect::<Vec<_>>(),
            //     next_token.token.get_str(),
            //     current_line
            //         .tokens
            //         .iter()
            //         .map(|token_with_shape| token_with_shape.token.get_str())
            //         .collect::<Vec<_>>(),
            // ); // TODO: REMOVE

            // Check if the total width of overflown tokens exceeds the maximum width.
            // If it does, avoid appending tokens to next line to prevent endless loop.
            if current_line.tokens.len() > 0
                || Self::get_tokens_width(&mut overflown_tokens) + next_token.get_width()
                    < current_line.max_width
            {
                return ShouldBreakLine::True(LineBreakBehavior::AppendOverflownTokens(
                    overflown_tokens,
                ));
            } else {
                for overflown_token in overflown_tokens {
                    current_line.tokens.push(overflown_token);
                }
                return ShouldBreakLine::False;
            }
        }
        // Case when no previous overflow index was set
        // then append the next token to the next line
        else {
            return if matches!(next_token.token.kind, TokenKind::TextFragment { .. }) {
                ShouldBreakLine::True(LineBreakBehavior::AppendNextToken)
            } else {
                ShouldBreakLine::True(LineBreakBehavior::None)
            };
        }
    }

    fn get_tokens_width(tokens: &mut [TokenWithShape]) -> f32 {
        tokens
            .iter_mut()
            .map(|token_with_shape| token_with_shape.get_width())
            .sum::<f32>()
    }
}

impl LineBreakStrategy for BreakOnWordLineBreakStrategy {
    fn should_break(
        &mut self,
        current_line: &mut CurrentLine,
        next_token_in_line: &mut TokenWithShape,
        is_last_token: bool,
    ) -> ShouldBreakLine {
        let exceeds_width =
            current_line.current_width + next_token_in_line.get_width() > current_line.max_width;

        // Set start overflow index if needed
        if exceeds_width
            && matches!(
                next_token_in_line.token.kind,
                TokenKind::TextFragment { .. }
            )
            && self.start_overflow_index.is_none()
        {
            self.start_overflow_index = self.get_start_overflow_index(current_line);
        }

        return if exceeds_width {
            self.determine_line_break_behavior(current_line, next_token_in_line)
        } else {
            ShouldBreakLine::False
        };
    }
}
