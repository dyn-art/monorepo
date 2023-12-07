use crate::core::modules::node::systems::construct_path::text::{
    current_line::CurrentLine, token_with_shape::TokenWithShape,
};

use super::{LineBreakBehavior, LineBreakStrategy, ShouldBreakLine};

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
        is_last_token: bool,
    ) -> ShouldBreakLine {
        let should_break =
            current_line.current_width + next_token_in_line.get_width() > current_line.max_width;

        return if should_break && !current_line.is_empty() {
            ShouldBreakLine::True {
                line_break_behavior: LineBreakBehavior::AppendNextToken(true),
            }
        } else {
            ShouldBreakLine::False
        };
    }
}
