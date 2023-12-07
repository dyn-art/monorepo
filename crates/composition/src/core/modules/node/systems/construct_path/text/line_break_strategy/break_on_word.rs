use crate::core::modules::node::systems::construct_path::text::{
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

        return match next_token_in_line.token.kind {
            TokenKind::TextFragment { .. } => {
                // Set the start overflow index only once when needed
                if exceeds_width && self.start_overflow_index.is_none() {
                    self.start_overflow_index = self.get_start_overflow_index(current_line);
                }

                // Determine a line break for the last token,
                // as no further non-text fragment token will follow
                if is_last_token {
                    self.determine_break(exceeds_width, current_line, next_token_in_line)
                } else {
                    ShouldBreakLine::False
                }
            }
            // For non-TextFragment tokens, directly determine line break
            _ => self.determine_break(exceeds_width, current_line, next_token_in_line),
        };
    }
}

impl BreakOnWordLineBreakStrategy {
    /// Determines the index at which text overflow begins in a sequence of `TextFragments`.
    fn get_start_overflow_index(&mut self, current_line: &CurrentLine) -> Option<usize> {
        current_line
            .tokens
            .iter()
            .enumerate()
            .rev()
            .find(|(_, token_with_shape)| {
                !matches!(token_with_shape.token.kind, TokenKind::TextFragment { .. })
            })
            .map(|(index, _)| index + 1)
            .or(Some(0))
    }

    fn determine_break(
        &mut self,
        exceeds_width: bool,
        current_line: &mut CurrentLine,
        next_token_in_line: &TokenWithShape,
    ) -> ShouldBreakLine {
        if exceeds_width && !current_line.is_empty() {
            self.start_overflow_index.take().map_or_else(
                // Case when no previous overflow index was set
                // then append the next token to the new line
                || {
                    if !matches!(next_token_in_line.token.kind, TokenKind::Space { .. }) {
                        ShouldBreakLine::True(LineBreakBehavior::AppendNextToken)
                    } else {
                        ShouldBreakLine::True(LineBreakBehavior::None)
                    }
                },
                // Case when a previous overflow index was set
                // then collect overflown tokens and append them to the new line
                |start_overflow_index| {
                    let mut overflown_tokens: Vec<TokenWithShape> = current_line
                        .drain(start_overflow_index..)
                        .filter(|token_with_shape| {
                            matches!(token_with_shape.token.kind, TokenKind::TextFragment { .. })
                        })
                        .collect();
                    let total_width =
                        Self::reduce_to_width(&mut overflown_tokens, current_line.max_width);

                    // Check if the total width of overflown tokens exceeds the maximum width.
                    // If it does, avoid breaking the line to prevent endless loop.
                    return if total_width < current_line.max_width {
                        ShouldBreakLine::True(LineBreakBehavior::AppendOverflownTokens(
                            overflown_tokens,
                        ))
                    } else {
                        ShouldBreakLine::False
                    };
                },
            )
        } else {
            ShouldBreakLine::False
        }
    }

    fn reduce_to_width(tokens: &mut Vec<TokenWithShape>, max_width: f32) -> f32 {
        let mut total_width: f32 = 0.0;

        for token in tokens {
            total_width += token.get_width();
            if total_width > max_width {
                return total_width;
            }
        }

        return total_width;
    }
}
