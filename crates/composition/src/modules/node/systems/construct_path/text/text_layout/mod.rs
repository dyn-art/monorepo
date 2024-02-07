use tiny_skia_path::{NonZeroPositiveF32, Path, Transform};
use tinyvec::TinyVec;

use crate::modules::{
    composition::resources::font::{
        usvg::{
            geom::BBox,
            text::{
                AlignmentBaseline, BaselineShift, DominantBaseline, Font, FontFamily, FontStretch,
                FontStyle, LengthAdjust, TextAnchor, TextFlow,
            },
            text_to_paths::{GlyphClusters, OutlinedCluster},
        },
        FontContext,
    },
    node::components::types::{TextNode, TextSpan},
};

pub struct TokenChunk {
    /// A text anchor.
    pub anchor: TextAnchor,
    /// A list of text chunk style spans.
    pub spans: Vec<TokenSpan>,
    /// A text chunk flow.
    pub text_flow: TextFlow,
}

impl TokenChunk {
    pub fn from_text_node(text: &TextNode, cx: &mut FontContext) -> Self {
        let token_spans = Self::create_token_spans(text, cx);

        Self {
            spans: token_spans,
            anchor: TextAnchor::Start,
            text_flow: TextFlow::Linear,
        }
    }

    fn create_token_spans(text_node: &TextNode, cx: &mut FontContext) -> Vec<TokenSpan> {
        let mut token_spans: Vec<TokenSpan> = Vec::new();

        // Iterate through text spans, creating tokens
        for TextSpan {
            text,
            style,
            font: font_metadata,
        } in &text_node.spans
        {
            let mut tokens: TinyVec<[Token; 4]> = TinyVec::new();
            let font = Font {
                families: vec![FontFamily::Named(font_metadata.family.clone())],
                stretch: FontStretch::Normal,
                style: match font_metadata.style {
                    crate::modules::node::components::types::FontStyle::Italic => FontStyle::Italic,
                    crate::modules::node::components::types::FontStyle::Normal => FontStyle::Normal,
                    crate::modules::node::components::types::FontStyle::Oblique => {
                        FontStyle::Oblique
                    }
                },
                weight: font_metadata.weight,
            };

            // Resolve font
            let resolved_font = cx.resolve_font(&font);
            if resolved_font.is_none() {
                continue;
            }

            // Tokenize the text, considering spaces and line breaks
            let mut start = 0;
            for (index, match_str) in text.match_indices(|c: char| {
                FontContext::is_word_separator_char(c) || FontContext::is_linebreak_char(c)
            }) {
                // Create a text fragment token for non-whitespace segments
                if start != index {
                    tokens.push(Token::new(TokenVariant::TextFragment {
                        text: String::from(&text[start..index]),
                    }));
                }

                // Create a token for each space or line break
                tokens.push(match match_str.chars().next() {
                    Some(c) if FontContext::is_word_separator_char(c) => {
                        Token::new(TokenVariant::WordSeparator {
                            text: c.to_string(),
                            char: c,
                        })
                    }
                    Some(c) if FontContext::is_linebreak_char(c) => {
                        Token::new(TokenVariant::Linebreak {
                            text: c.to_string(),
                            char: c,
                        })
                    }
                    _ => Token::new(TokenVariant::Unresolved), // Should never happen
                });

                start = index + match_str.len();
            }

            // Handle the last word in the segment, if any
            if start < text.len() {
                tokens.push(Token::new(TokenVariant::TextFragment {
                    text: String::from(&text[start..]),
                }));
            }

            token_spans.push(TokenSpan {
                tokens,
                apply_kerning: false,
                font,
                font_size: NonZeroPositiveF32::new(style.font_size).unwrap(),
                letter_spacing: 0.0,
                small_caps: false,
                text_length: None,
                word_spacing: 0.0,
                alignment_baseline: AlignmentBaseline::default(),
                baseline_shift: Vec::new(),
                dominant_baseline: DominantBaseline::default(),
                length_adjust: LengthAdjust::default(),
            });
        }

        return token_spans;
    }

    pub fn to_paths(&mut self, cx: &mut FontContext) -> Vec<Path> {
        let mut bbox = BBox::default();
        let mut last_x = 0.0;
        let mut last_y = 0.0;
        // let mut new_paths = Vec::new();

        let (x, y) = match self.text_flow {
            TextFlow::Linear => (last_x, last_y),
            TextFlow::Path(_) => (0.0, 0.0),
        };

        self.outline(cx);
        log::info!("[to_paths] After outline: {:#?}", self.spans);

        let mut text_ts = Transform::default();
        for span in &mut self.spans {
            for token in &mut span.tokens {
                token.apply_letter_spacing(span.letter_spacing);
                token.apply_word_spacing(span.word_spacing);
                token.apply_length_adjust(
                    span.length_adjust,
                    span.text_length,
                    self.text_flow.clone(),
                );
            }

            let mut span_ts = text_ts;
            span_ts = span_ts.pre_translate(x, y);
            // if let TextFlow::Linear = self.text_flow {
            //     let shift = resolve_baseline(span, font, text_node.writing_mode);

            //     // In case of a horizontal flow, shift transform and not clusters,
            //     // because clusters can be rotated and an additional shift will lead
            //     // to invalid results.
            //     span_ts = span_ts.pre_translate(0.0, shift);
            // }
        }

        vec![]
    }

    fn outline(&mut self, cx: &mut FontContext) {
        for span in &mut self.spans {
            let font = match cx.resolve_font(&span.font) {
                Some(v) => v.clone(),
                None => continue,
            };

            for token in &mut span.tokens {
                let mut clusters: Vec<OutlinedCluster> = Vec::new();
                let text = token.variant.get_str();
                let glyphs = cx.shape_text(text, font.clone(), span.small_caps, span.apply_kerning);

                // Do nothing with the first run.
                if glyphs.is_empty() {
                    continue;
                }

                // Convert glyphs to clusters.
                for (range, _) in GlyphClusters::new(&glyphs) {
                    clusters.push(cx.outline_cluster(&glyphs[range], &text, span.font_size.get()));
                }

                token.clusters = Some(clusters);
            }
        }
    }
}

#[derive(Debug)]
pub struct TokenSpan {
    pub tokens: TinyVec<[Token; 4]>,
    /// A font.
    pub font: Font,
    /// A font size.
    pub font_size: NonZeroPositiveF32,
    /// Indicates that small caps should be used.
    ///
    /// Set by `font-variant="small-caps"`
    pub small_caps: bool,
    /// Indicates that a kerning should be applied.
    ///
    /// Supports both `kerning` and `font-kerning` properties.
    pub apply_kerning: bool,
    /// A span dominant baseline.
    pub dominant_baseline: DominantBaseline,
    /// A span alignment baseline.
    pub alignment_baseline: AlignmentBaseline,
    /// A list of all baseline shift that should be applied to this span.
    ///
    /// Ordered from `text` element down to the actual `span` element.
    pub baseline_shift: Vec<BaselineShift>,
    /// A letter spacing property.
    pub letter_spacing: f32,
    /// A word spacing property.
    pub word_spacing: f32,
    /// A text length property.
    pub text_length: Option<f32>,
    /// A length adjust property.
    pub length_adjust: LengthAdjust,
}

#[derive(Default, Debug, Clone)]
pub struct Token {
    pub variant: TokenVariant,
    pub clusters: Option<Vec<OutlinedCluster>>,
}

impl Token {
    pub fn new(variant: TokenVariant) -> Self {
        Self {
            variant,
            clusters: None,
        }
    }

    pub fn apply_letter_spacing(&mut self, letter_spacing: f32) {
        if let TokenVariant::TextFragment { .. } = self.variant {
            if let Some(clusters) = &mut self.clusters {
                FontContext::apply_letter_spacing(clusters, letter_spacing);
            }
        }
    }

    pub fn apply_word_spacing(&mut self, word_spacing: f32) {
        if let TokenVariant::WordSeparator { .. } = self.variant {
            if let Some(clusters) = &mut self.clusters {
                FontContext::apply_word_spacing(clusters, word_spacing);
            }
        }
    }

    pub fn apply_length_adjust(
        &mut self,
        length_adjust: LengthAdjust,
        text_length: Option<f32>,
        text_flow: TextFlow,
    ) {
        if let Some(clusters) = &mut self.clusters {
            FontContext::apply_length_adjust(clusters, length_adjust, text_length, text_flow);
        }
    }
}

#[derive(Debug, Default, Clone)]
pub enum TokenVariant {
    TextFragment {
        text: String,
    },
    WordSeparator {
        char: char,
        text: String,
    },
    Linebreak {
        char: char,
        text: String,
    },
    #[default]
    Unresolved,
}

impl TokenVariant {
    pub fn get_str(&self) -> &str {
        match &self {
            TokenVariant::WordSeparator { text: value, .. } => value.as_str(),
            TokenVariant::TextFragment { text: value } => value.as_str(),
            TokenVariant::Linebreak { text: value, .. } => value.as_str(),
            TokenVariant::Unresolved => "_",
        }
    }
}
