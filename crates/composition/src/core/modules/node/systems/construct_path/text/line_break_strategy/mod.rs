use super::{current_line::CurrentLine, token_with_shape::TokenWithShape};

pub mod break_on_word;
pub mod simple_break_on_word;

pub enum ShouldBreakLine {
    False,
    True {
        line_break_behavior: LineBreakBehavior,
    },
}

pub enum LineBreakBehavior {
    OverflownTokens(Vec<TokenWithShape>),
    AppendNextToken(bool),
}

pub trait LineBreakStrategy {
    /// Determines whether to break the line based on the next token meta data.
    fn should_break(
        &mut self,
        current_line: &mut CurrentLine,
        next_token_in_line: &mut TokenWithShape,
    ) -> ShouldBreakLine;
}
