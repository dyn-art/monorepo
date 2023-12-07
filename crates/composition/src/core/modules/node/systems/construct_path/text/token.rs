use crate::core::modules::node::components::types::TextStyle;

use super::continuous_id::ContinuousId;

#[derive(Debug, Clone)]
pub enum Token {
    TextFragment {
        id: ContinuousId,
        value: String,
        style: TextStyle,
        metric: TokenMetric,
    },
    Space {
        id: ContinuousId,
        style: TextStyle,
        metric: TokenMetric,
    },
    Linebreak {
        id: ContinuousId,
    },
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

    pub fn get_str(&self) -> &str {
        match self {
            Token::Space { .. } => " ",
            Token::TextFragment { value, .. } => value.as_str(),
            _ => " ",
        }
    }

    pub fn get_id(&self) -> ContinuousId {
        match self {
            Token::Space { id, .. } | Token::TextFragment { id, .. } | Token::Linebreak { id } => {
                id.clone()
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct TokenMetric {
    pub height: f32,
    pub ascender: f32,
    pub descender: f32,
    pub scale: f32,
}
