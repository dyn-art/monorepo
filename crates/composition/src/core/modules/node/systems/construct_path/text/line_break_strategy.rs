use super::{current_line::CurrentLine, token::Token, token_with_shape::TokenWithShape};

pub enum ShouldLineBreak {
    False,
    True {
        maybe_overflown_tokens: Option<Vec<TokenWithShape>>,
    },
}

pub trait LineBreakStrategy {
    fn should_break(
        &mut self,
        current_line: &mut CurrentLine,
        next_token_in_line: &mut TokenWithShape,
    ) -> ShouldLineBreak;
}

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

        return if should_break {
            ShouldLineBreak::True {
                maybe_overflown_tokens: None,
            }
        } else {
            ShouldLineBreak::False
        };
    }
}

// TODO:
// impl LineBreakStrategy for BreakOnWordLineBreakStrategy {
//     fn should_break(
//         &mut self,
//         current_line: &mut CurrentLine,
//         next_token_in_line: &mut TokenWithShape,
//     ) -> ShouldLineBreak {
//         let should_break =
//             current_line.current_width + next_token_in_line.get_width() > current_line.max_width;

//         match next_token_in_line.token {
//             Token::TextFragment { .. } => {
//                 // Determine if the current line ends with a TextFragment
//                 if should_break && self.start_overflow_index.is_none() {
//                     if let Some((index, _)) = current_line
//                         .tokens
//                         .iter()
//                         .enumerate()
//                         .rev()
//                         .find(|(_, token)| matches!(token.token, Token::TextFragment { .. }))
//                     {
//                         // Check if this TextFragment is directly followed by a Space or the end
//                         if index == current_line.tokens.len() - 1
//                             || matches!(current_line.tokens[index + 1].token, Token::Space { .. })
//                         {
//                             // Set start_overflow_index to the index of the first TextFragment
//                             // in the last continuous group of TextFragments
//                             self.start_overflow_index = Some(index);
//                         }
//                     }
//                 }

//                 ShouldLineBreak::False
//             }
//             _ => {
//                 if should_break {
//                     let maybe_start_overflow_index = self.start_overflow_index.take();
//                     ShouldLineBreak::True {
//                         maybe_overflown_tokens: maybe_start_overflow_index
//                             .map(|start_index| current_line.drain(start_index..)),
//                     }
//                 } else {
//                     ShouldLineBreak::False
//                 }
//             }
//         }
//     }
// }
