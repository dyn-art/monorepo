use super::{current_line::CurrentLine, token_with_shape::TokenWithShape};

pub trait LineBreakStrategy {
    fn should_break(
        &self,
        current_line: &CurrentLine,
        next_token_in_line: &mut TokenWithShape,
    ) -> bool;
}

pub struct BreakOnWordLineBreakStrategy;

impl LineBreakStrategy for BreakOnWordLineBreakStrategy {
    fn should_break(
        &self,
        current_line: &CurrentLine,
        next_token_in_line: &mut TokenWithShape,
    ) -> bool {
        match next_token_in_line.token {
            // Token::TextFragment { .. } => false,
            _ => {
                current_line.current_width + next_token_in_line.get_width() > current_line.max_width
            }
        }
    }
}
