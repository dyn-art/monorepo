use crate::core::modules::node::components::types::TextStyle;

#[derive(Debug, Clone)]
pub enum Token {
    TextFragment {
        value: String,
        style: TextStyle,
        metric: TokenMetric,
    },
    Space {
        style: TextStyle,
        metric: TokenMetric,
    },
    Linebreak,
}

impl Token {
    pub fn compute_token_metric(buzz_face: &rustybuzz::Face, font_size: f32) -> TokenMetric {
        let scale = font_size / (buzz_face.units_per_em() as f32);
        let ascender = buzz_face.ascender() as f32 * scale;
        let descender = buzz_face.descender() as f32 * scale;
        buzz_face.height();
        return TokenMetric {
            ascender,
            descender,
            height: ascender - descender,
            scale,
        };
    }
}

#[derive(Clone, Debug)]
pub struct TokenMetric {
    pub height: f32,
    pub ascender: f32,
    pub descender: f32,
    pub scale: f32,
}
