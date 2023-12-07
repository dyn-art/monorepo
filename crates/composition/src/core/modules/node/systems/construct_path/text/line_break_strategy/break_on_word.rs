use crate::core::modules::node::systems::construct_path::text::{
    current_line::CurrentLine, token::Token, token_with_shape::TokenWithShape,
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
    ) -> ShouldBreakLine {
        let exceeds_width =
            current_line.current_width + next_token_in_line.get_width() > current_line.max_width;

        return match next_token_in_line.token {
            Token::TextFragment { .. } => {
                if exceeds_width {
                    if self.start_overflow_index.is_none() {
                        self.start_overflow_index = self.get_start_overflow_index(current_line);
                    }
                }
                ShouldBreakLine::False
            }
            _ => self.determine_break_for_non_text_fragment(
                exceeds_width,
                current_line,
                next_token_in_line,
            ),
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
                !matches!(token_with_shape.token, Token::TextFragment { .. })
            })
            .map(|(index, _)| index + 1)
            .or(Some(0))
    }

    /// Determines the line break behavior for non-text fragment tokens.
    fn determine_break_for_non_text_fragment(
        &mut self,
        exceeds_width: bool,
        current_line: &mut CurrentLine,
        next_token_in_line: &TokenWithShape,
    ) -> ShouldBreakLine {
        if exceeds_width && !current_line.is_empty() {
            self.start_overflow_index.take().map_or_else(
                // Case when no previous overflow index was set
                // then append the next token to the new line
                || ShouldBreakLine::True {
                    line_break_behavior: LineBreakBehavior::AppendNextToken(!matches!(
                        next_token_in_line.token,
                        Token::Space { .. }
                    )),
                },
                // Case when a previous overflow index was set
                // then collect overflown tokens and append them to the new line
                |start_overflow_index| {
                    let mut overflown_tokens: Vec<TokenWithShape> = current_line
                        .drain(start_overflow_index..)
                        .filter(|token_with_shape| {
                            matches!(token_with_shape.token, Token::TextFragment { .. })
                        })
                        .collect();
                    let total_width =
                        Self::reduce_to_width(&mut overflown_tokens, current_line.max_width);

                    // Check if the total width of overflown tokens exceeds the maximum width.
                    // If it does, avoid breaking the line to prevent endless loop.
                    return if total_width > current_line.max_width {
                        ShouldBreakLine::False
                    } else {
                        ShouldBreakLine::True {
                            line_break_behavior: LineBreakBehavior::OverflownTokens(
                                overflown_tokens,
                            ),
                        }
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
