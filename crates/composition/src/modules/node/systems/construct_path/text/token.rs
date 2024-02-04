use crate::modules::node::components::types::TextStyle;

#[derive(Debug, Clone)]
pub enum TokenKind {
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

#[derive(Debug, Clone)]
pub struct Token {
    processed_count: u8,
    max_processed_count: u8,
    pub kind: TokenKind,
}

impl Token {
    pub fn new(token_kind: TokenKind) -> Self {
        Self {
            kind: token_kind,
            processed_count: 0,
            max_processed_count: 100,
        }
    }

    pub fn compute_token_metric(buzz_face: &rustybuzz::Face, font_size: f32) -> TokenMetric {
        let font_scale = font_size / (buzz_face.units_per_em() as f32);
        let ascender = buzz_face.ascender() as f32 * font_scale;
        let descender = buzz_face.descender() as f32 * font_scale;
        buzz_face.height();
        return TokenMetric {
            ascender,
            descender,
            height: ascender - descender,
            font_scale,
        };
    }

    pub fn get_str(&self) -> &str {
        match &self.kind {
            TokenKind::Space { .. } => " ",
            TokenKind::TextFragment { value, .. } => value.as_str(),
            TokenKind::Linebreak => "\n",
        }
    }

    pub fn track_processed(&mut self) {
        if self.processed_count < self.max_processed_count {
            self.processed_count += 1;
        } else {
            panic!(
                "Token with the value '{}' reached the max allowed processing count!",
                self.get_str()
            )
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct TokenMetric {
    pub height: f32,
    pub ascender: f32,
    pub descender: f32,
    pub font_scale: f32,
}
