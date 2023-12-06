use super::{current_line::CurrentLine, token_with_shape::TokenWithShape};

pub mod break_on_word;
pub mod simple_break_on_word;

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
