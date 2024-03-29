use std::ops::Range;

use dyn_fonts_book::FontsBook;
use rust_lapper::{Interval, Lapper};
use unicode_linebreak::BreakClass;

use crate::{
    attrs::Attrs,
    shape_tokens::{
        glyph::GlyphToken, linebreak::LinebreakToken, text_fragment::TextFragmentToken,
        word_separator::WordSeparatorToken, ShapeBuffer, ShapeToken, ShapeTokenVariant,
    },
    utils::is_range_within,
};

#[derive(Debug, Clone)]
pub struct Span {
    range: Range<usize>,
    dirty: bool,
    tokens: Vec<ShapeTokenVariant>,
    level: unicode_bidi::Level,
    attrs: Attrs,
}

impl Span {
    pub fn new(range: Range<usize>, attrs: Attrs) -> Self {
        Self {
            range,
            dirty: true,
            tokens: Vec::new(),
            level: unicode_bidi::LTR_LEVEL,
            attrs,
        }
    }

    #[inline]
    pub fn get_range(&self) -> &Range<usize> {
        &self.range
    }

    #[inline]
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    #[inline]
    pub fn get_tokens(&self) -> &Vec<ShapeTokenVariant> {
        &self.tokens
    }

    #[inline]
    pub fn get_level(&self) -> &unicode_bidi::Level {
        &self.level
    }

    #[inline]
    pub fn get_attrs(&self) -> &Attrs {
        &self.attrs
    }

    #[inline]
    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    pub fn divide_at_bidi(self, bidi_info: &unicode_bidi::BidiInfo) -> Vec<Self> {
        let mut current_bidi_level = bidi_info.levels[self.range.start];
        let mut new_spans: Vec<Self> = Vec::new();
        let mut span_start = self.range.start;

        for i in self.range.clone() {
            let char_bidi_level = bidi_info.levels[i];
            if char_bidi_level != current_bidi_level {
                new_spans.push(Self::new(span_start..i, self.attrs.clone()));
                span_start = i;
                current_bidi_level = char_bidi_level;
            }
        }

        if new_spans.len() > 0 {
            new_spans.push(Self::new(span_start..self.range.end, self.attrs.clone()))
        } else {
            new_spans.push(self)
        }

        return new_spans;
    }

    pub fn compute_tokens(&mut self, text: &str, fonts_book: &mut FontsBook) {
        let mut tokens: Vec<ShapeTokenVariant> = Vec::new();
        let span_text = &text[self.range.clone()]; // TODO: This cause issue if range partly ranges into multi byte char (e.g. "·" if range ends at index 1 although the char is 2 bytes long)
        let mut shape_buffer = ShapeBuffer {
            buffer: Some(rustybuzz::UnicodeBuffer::new()),
        };

        log::info!("SpanToken for text: '{}' ({:?})", span_text, self.range);

        // Process each character for potential tokenization within the paragraph
        let mut start = self.range.start;
        for (index, _char) in span_text.char_indices() {
            let global_index = self.range.start + index; // Adjust index relative to the entire text
            let break_class = unicode_linebreak::break_property(_char as u32);

            match break_class {
                // Handle line break
                BreakClass::Mandatory
                | BreakClass::LineFeed
                | BreakClass::NextLine
                | BreakClass::CarriageReturn => {
                    // Add text fragment token
                    if start != global_index {
                        tokens.push(ShapeTokenVariant::TextFragment(
                            TextFragmentToken::from_text(
                                text,
                                start..global_index,
                                &self.attrs,
                                &mut shape_buffer,
                                fonts_book,
                            ),
                        ));
                    }

                    // Add line break token
                    tokens.push(ShapeTokenVariant::Linebreak(LinebreakToken::new(
                        global_index..global_index + 1,
                    )));
                    start = global_index + 1;
                }

                // Handle text segment separation
                BreakClass::Space | BreakClass::ZeroWidthSpace => {
                    // Add text fragment token
                    if start != global_index {
                        tokens.push(ShapeTokenVariant::TextFragment(
                            TextFragmentToken::from_text(
                                text,
                                start..global_index,
                                &self.attrs,
                                &mut shape_buffer,
                                fonts_book,
                            ),
                        ));
                    }

                    // Add word separator token
                    tokens.push(ShapeTokenVariant::WordSeparator(
                        WordSeparatorToken::from_text(
                            text,
                            global_index..global_index + 1,
                            &self.attrs,
                            &mut shape_buffer,
                            fonts_book,
                        ),
                    ));
                    start = global_index + 1;
                }
                _ => {}
            }
        }

        // Handle the last text fragment within the paragraph, if any
        if start < self.range.end {
            tokens.push(ShapeTokenVariant::TextFragment(
                TextFragmentToken::from_text(
                    text,
                    start..self.range.end,
                    &self.attrs,
                    &mut shape_buffer,
                    fonts_book,
                ),
            ));
        }

        self.tokens = tokens;
    }

    pub fn iter_glyphs<'a>(&'a self) -> impl Iterator<Item = &'a GlyphToken> + 'a {
        self.tokens
            .iter()
            .flat_map(|token_variant| match token_variant {
                ShapeTokenVariant::Glyph(token) => Box::new(std::iter::once(token))
                    as Box<dyn Iterator<Item = &'a GlyphToken> + 'a>,
                ShapeTokenVariant::TextFragment(token) => Box::new(token.get_tokens().iter())
                    as Box<dyn Iterator<Item = &'a GlyphToken> + 'a>,
                ShapeTokenVariant::WordSeparator(token) => Box::new(token.get_tokens().iter())
                    as Box<dyn Iterator<Item = &'a GlyphToken> + 'a>,
                _ => Box::new(std::iter::empty()) as Box<dyn Iterator<Item = &'a GlyphToken> + 'a>,
            })
    }

    pub fn iter_glyphs_in_range<'a>(
        &'a self,
        range: &'a Range<usize>,
    ) -> impl Iterator<Item = &'a GlyphToken> + 'a {
        self.tokens
            .iter()
            .flat_map(|token_variant| match token_variant {
                ShapeTokenVariant::Glyph(token) => Box::new(std::iter::once(token))
                    as Box<dyn Iterator<Item = &'a GlyphToken> + 'a>,
                ShapeTokenVariant::TextFragment(token) => Box::new(token.get_tokens().iter())
                    as Box<dyn Iterator<Item = &'a GlyphToken> + 'a>,
                ShapeTokenVariant::WordSeparator(token) => Box::new(token.get_tokens().iter())
                    as Box<dyn Iterator<Item = &'a GlyphToken> + 'a>,
                _ => Box::new(std::iter::empty()) as Box<dyn Iterator<Item = &'a GlyphToken> + 'a>,
            })
            .filter(move |glyph| is_range_within(glyph.get_range(), &range))
    }

    pub(crate) fn iter_glyphs_mut<'a>(
        &'a mut self,
    ) -> impl Iterator<Item = &'a mut GlyphToken> + 'a {
        self.tokens
            .iter_mut()
            .flat_map(|token_variant| match token_variant {
                ShapeTokenVariant::Glyph(token) => Box::new(std::iter::once(token))
                    as Box<dyn Iterator<Item = &'a mut GlyphToken> + 'a>,
                ShapeTokenVariant::TextFragment(token) => {
                    Box::new(token.get_tokens_mut().iter_mut())
                        as Box<dyn Iterator<Item = &'a mut GlyphToken> + 'a>
                }
                ShapeTokenVariant::WordSeparator(token) => {
                    Box::new(token.get_tokens_mut().iter_mut())
                        as Box<dyn Iterator<Item = &'a mut GlyphToken> + 'a>
                }
                _ => Box::new(std::iter::empty())
                    as Box<dyn Iterator<Item = &'a mut GlyphToken> + 'a>,
            })
    }

    pub(crate) fn iter_glyphs_in_range_mut<'a>(
        &'a mut self,
        range: &'a Range<usize>,
    ) -> impl Iterator<Item = &'a mut GlyphToken> + 'a {
        self.tokens
            .iter_mut()
            .flat_map(|token_variant| match token_variant {
                ShapeTokenVariant::Glyph(token) => Box::new(std::iter::once(token))
                    as Box<dyn Iterator<Item = &'a mut GlyphToken> + 'a>,
                ShapeTokenVariant::TextFragment(token) => {
                    Box::new(token.get_tokens_mut().iter_mut())
                        as Box<dyn Iterator<Item = &'a mut GlyphToken> + 'a>
                }
                ShapeTokenVariant::WordSeparator(token) => {
                    Box::new(token.get_tokens_mut().iter_mut())
                        as Box<dyn Iterator<Item = &'a mut GlyphToken> + 'a>
                }
                _ => Box::new(std::iter::empty())
                    as Box<dyn Iterator<Item = &'a mut GlyphToken> + 'a>,
            })
            .filter(move |glyph| is_range_within(glyph.get_range(), &range))
    }

    /// An iterator over glyph clusters within the span.
    ///
    /// This iterator groups adjacent glyphs based on their starting position (byte index),
    /// considering glyphs with the same `start` value as part of the same cluster.
    /// It's particularly useful for processing text where multiple glyphs
    /// contribute to a single visual character (grapheme) or are otherwise logically grouped.
    ///
    /// # Example
    ///
    /// Given glyphs with starting positions like: 0, 2, 2, 2, 3, 4, 4, 5, 5,
    /// the iterator will produce clusters with indices: [0, 1], [1, 4], [4, 5], [5, 7], [7, 9]
    pub fn iter_glyph_clusters<'a>(
        &'a self,
    ) -> impl Iterator<Item = (Vec<&'a GlyphToken>, usize)> + 'a {
        GlyphClusterIterator::new(&self.tokens)
    }
}

pub type SpanInterval = Interval<usize, Span>;
pub type SpanIntervals = Lapper<usize, Span>;

struct GlyphClusterIterator<'a> {
    glyphs: Vec<&'a GlyphToken>,
    index: usize,
}

impl<'a> GlyphClusterIterator<'a> {
    fn new(tokens: &'a [ShapeTokenVariant]) -> Self {
        let glyphs: Vec<&GlyphToken> = tokens
            .iter()
            .flat_map(|token_variant| match token_variant {
                ShapeTokenVariant::Glyph(token) => vec![token],
                ShapeTokenVariant::TextFragment(token) => token.get_tokens().iter().collect(),
                ShapeTokenVariant::WordSeparator(token) => token.get_tokens().iter().collect(),
                _ => Vec::new(),
            })
            .collect();

        Self { glyphs, index: 0 }
    }
}

impl<'a> Iterator for GlyphClusterIterator<'a> {
    type Item = (Vec<&'a GlyphToken>, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.glyphs.len() {
            return None;
        }

        let mut cluster = Vec::new();
        let cluster_start = self.glyphs[self.index].get_range().start;

        // Iterate through the glyphs, and collect glyphs
        // that belong to the current cluster (having the same byte index and thus `start` value)
        while self.index < self.glyphs.len()
            && self.glyphs[self.index].get_range().start == cluster_start
        {
            cluster.push(self.glyphs[self.index]);
            self.index += 1;
        }

        if !cluster.is_empty() {
            Some((cluster, cluster_start))
        } else {
            None
        }
    }
}
