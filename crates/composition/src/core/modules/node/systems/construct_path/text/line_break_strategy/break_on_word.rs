use crate::core::modules::node::systems::construct_path::text::{
    current_line::CurrentLine, token::Token, token_with_shape::TokenWithShape,
};

use super::{LineBreakStrategy, ShouldLineBreak};

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
    ) -> ShouldLineBreak {
        let should_break =
            current_line.current_width + next_token_in_line.get_width() > current_line.max_width;

        match next_token_in_line.token {
            Token::TextFragment { .. } => {
                if should_break {
                    // Reverse iterate over the tokens to find the start index
                    // of the last continuous group of TextFragments
                    if self.start_overflow_index.is_none() {
                        let mut found_non_text_fragment = false;

                        for (index, token_with_shape) in
                            current_line.tokens.iter().enumerate().rev()
                        {
                            match token_with_shape.token {
                                Token::TextFragment { .. } if found_non_text_fragment => {
                                    self.start_overflow_index = Some(index + 1);
                                    break;
                                }
                                Token::TextFragment { .. } => {}
                                _ => {
                                    found_non_text_fragment = true;
                                }
                            }
                        }

                        // If all tokens are TextFragments, set start_overflow_index to 0
                        if !found_non_text_fragment {
                            self.start_overflow_index = Some(0);
                        }
                    }
                }

                ShouldLineBreak::False
            }
            _ => {
                if should_break {
                    let maybe_start_overflow_index = self.start_overflow_index.take();
                    ShouldLineBreak::True {
                        maybe_overflown_tokens: maybe_start_overflow_index.map(
                            |start_overflow_index| {
                                current_line
                                    .drain(start_overflow_index..)
                                    .filter(|token_with_shape| {
                                        matches!(token_with_shape.token, Token::TextFragment { .. })
                                    })
                                    .collect()
                            },
                        ),
                    }
                } else {
                    ShouldLineBreak::False
                }
            }
        }
    }
}
