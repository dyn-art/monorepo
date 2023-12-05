use super::token_with_shape::TokenWithShape;

pub struct CurrentLine<'a> {
    pub tokens: Vec<TokenWithShape<'a>>,
    pub current_width: f32,
    pub max_width: f32,
}

impl<'a> CurrentLine<'a> {
    pub fn new(max_width: f32) -> Self {
        CurrentLine {
            tokens: Vec::new(),
            current_width: 0.0,
            max_width,
        }
    }

    pub fn append(&mut self, mut line_token: TokenWithShape<'a>) {
        self.current_width += line_token.get_width();
        self.tokens.push(line_token);
    }

    pub fn drain(&mut self) -> Vec<TokenWithShape> {
        self.current_width = 0.0;
        return self.tokens.drain(..).collect();
    }

    pub fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }
}
