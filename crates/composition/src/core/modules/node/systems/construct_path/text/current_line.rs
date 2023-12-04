use crate::core::modules::{
    composition::resources::font_cache::FontCacheRes, node::components::types::TextStyle,
};

pub struct CurrentLine {
    pub text: String,
    pub style_ranges: Vec<TextStyleRange>,
    pub max_ascender: f32,
    pub max_height: f32,
}

pub struct TextStyleRange {
    pub style: TextStyle,
    pub metric: TextStyleMetric,
    pub start: usize,
    pub end: usize,
}

pub struct TextStyleMetric {
    pub height: f32,
    pub ascender: f32,
    pub scale: f32,
}

impl CurrentLine {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            style_ranges: Vec::new(),
            max_ascender: 0.0,
            max_height: 0.0,
        }
    }

    pub fn add_section(&mut self, text: &str, style: TextStyle, font_cache: &mut FontCacheRes) {
        let start = self.text.len();
        self.text.push_str(text);
        let end = self.text.len();

        if let Some(metric) = CurrentLine::calculate_text_style_metric(&style, font_cache) {
            self.max_ascender = self.max_ascender.max(metric.ascender);
            self.max_height = self.max_height.max(metric.height);
            self.style_ranges.push(TextStyleRange {
                style,
                metric,
                start,
                end,
            });
        }
    }

    fn calculate_text_style_metric(
        style: &TextStyle,
        font_cache: &mut FontCacheRes,
    ) -> Option<TextStyleMetric> {
        let font_size = style.font_size as f32;
        let font_hash = style.font_hash;
        font_cache
            .get_or_create_buzz_face(&font_hash)
            .map(|font_face| {
                let scale = (font_face.units_per_em() as f32).recip() * font_size;
                let font_height = font_face.height() as f32;
                TextStyleMetric {
                    ascender: (font_face.ascender() as f32 / font_height) * font_size / scale,
                    height: font_height,
                    scale,
                }
            })
    }
}
