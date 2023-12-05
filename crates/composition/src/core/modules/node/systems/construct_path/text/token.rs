use crate::core::modules::node::components::types::TextStyle;

#[derive(Debug, Clone)]
pub enum Token {
    TextFragment {
        value: String,
        style: TextStyle,
        metric: TokenStyleMetric,
    },
    Space {
        style: TextStyle,
        metric: TokenStyleMetric,
    },
    Linebreak,
}

#[derive(Clone, Debug)]
pub struct TokenStyleMetric {
    pub height: f32,
    pub ascender: f32,
    pub descender: f32,
    pub scale: f32,
}
