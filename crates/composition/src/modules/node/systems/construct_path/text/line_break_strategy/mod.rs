use super::{current_line::CurrentLine, token_with_shape::TokenWithShape};

pub mod break_on_word;
pub mod simple_break_on_word;

pub enum ShouldBreakLine {
    True(LineBreakBehavior),
    False,
}

#[derive(Debug)]
pub enum LineBreakBehavior {
    /// Moves tokens that exceed line width, along with the next token, to the next line.
    AppendOverflownTokens(Vec<TokenWithShape>),

    /// Shifts only the next token to the next line.
    AppendNextToken,

    /// No special line breaking action required.
    None,
}

pub trait LineBreakStrategy {
    /// Determines whether to break the line based on the next token meta data.
    fn should_break(
        &mut self,
        current_line: &mut CurrentLine,
        next_token_in_line: &mut TokenWithShape,
        is_last_token: bool,
    ) -> ShouldBreakLine;
}
