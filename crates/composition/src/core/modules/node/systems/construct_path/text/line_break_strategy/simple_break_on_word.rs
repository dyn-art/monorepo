use crate::core::modules::node::systems::construct_path::text::{
    current_line::CurrentLine, token_with_shape::TokenWithShape,
};

use super::{LineBreakStrategy, ShouldLineBreak};

pub struct SimpleBreakOnWordLineBreakStrategy;

impl SimpleBreakOnWordLineBreakStrategy {
    pub fn new() -> Self {
        Self
    }
}

impl LineBreakStrategy for SimpleBreakOnWordLineBreakStrategy {
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
