use crate::{
    attrs::Attrs,
    script::script_supports_letter_spacing,
    shape_tokens::{
        glyph::GlyphToken, linebreak::LinebreakToken, text_fragment::TextFragmentToken,
        word_separator::WordSeparatorToken, ShapeBuffer, ShapeToken, ShapeTokenVariant,
    },
    utils::is_range_within,
};
use dyn_fonts_book::FontsBook;
use dyn_utils::units::{abs::Abs, Numeric};
use either::Either;
use rust_lapper::{Interval, Lapper};
use std::ops::Range;
use unicode_linebreak::BreakClass;
use unicode_script::UnicodeScript;

#[derive(Debug, Clone)]
pub struct Span {
    range: Range<usize>,
    dirty: bool,
    tokens: Vec<ShapeTokenVariant>,
    bidi_level: Option<unicode_bidi::Level>,
    attrs: Attrs,
}

impl Span {
    pub fn new(range: Range<usize>, attrs: Attrs) -> Self {
        Self::new_with_bidi(range, attrs, None)
    }

    pub fn new_with_bidi(
        range: Range<usize>,
        attrs: Attrs,
        bidi_level: Option<unicode_bidi::Level>,
    ) -> Self {
        Self {
            range,
            dirty: true,
            tokens: Vec::new(),
            bidi_level,
            attrs,
        }
    }

    #[inline]
    pub fn get_range(&self) -> &Range<usize> {
        &self.range
    }

    #[inline]
    pub fn mark_dirty(&mut self) {
        self.dirty = true;
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
    pub fn get_bidi_level(&self) -> &Option<unicode_bidi::Level> {
        &self.bidi_level
    }

    #[inline]
    pub fn get_attrs(&self) -> &Attrs {
        &self.attrs
    }

    #[inline]
    pub fn is_rtl(&self) -> bool {
        self.bidi_level.map_or(false, |level| level.is_rtl())
    }

    #[inline]
    pub fn is_ltr(&self) -> bool {
        !self.is_rtl()
    }

    pub fn divide_at_bidi_level(mut self, bidi_info: &unicode_bidi::BidiInfo) -> Vec<Self> {
        let mut current_bidi_level = bidi_info.levels[self.range.start];
        let mut new_spans: Vec<Self> = Vec::new();
        let mut span_start = self.range.start;

        for i in self.range.clone() {
            let char_bidi_level = bidi_info.levels[i];
            if char_bidi_level != current_bidi_level {
                new_spans.push(Self::new_with_bidi(
                    span_start..i,
                    self.attrs.clone(),
                    Some(current_bidi_level),
                ));
                span_start = i;
                current_bidi_level = char_bidi_level;
            }
        }

        if new_spans.len() > 0 {
            new_spans.push(Self::new_with_bidi(
                span_start..self.range.end,
                self.attrs.clone(),
                Some(current_bidi_level),
            ))
        } else {
            self.bidi_level = Some(current_bidi_level);
            new_spans.push(self)
        }

        return new_spans;
    }

    pub fn compute_tokens(&mut self, text: &str, fonts_book: &mut FontsBook) {
        let mut tokens: Vec<ShapeTokenVariant> = Vec::new();
        let span_text = match text.get(self.range.clone()) {
            Some(v) => v,
            None => {
                log::error!("Failed to retrieve text slice for range {:?}! Either the range is out of bound or partly ranges into a multi byte char (e.g. '·' if range ends at index 1 although the char is 2 bytes long).", self.range);
                return;
            }
        };
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

    /// Applies letter-spacing to a span.
    ///
    /// [In the CSS spec](https://www.w3.org/TR/css-text-3/#letter-spacing-property)
    pub fn apply_letter_spacing(&mut self) {
        let letter_spacing = self.attrs.get_letter_spacing();
        if letter_spacing.is_zero() {
            return;
        }

        let glyps_len = self.get_glyph_count();
        let font_size = self.attrs.get_font_size();
        for (index, glyph_token) in self.iter_glyphs_mut().enumerate() {
            let script = glyph_token.get_glyph().codepoint.script();
            if script_supports_letter_spacing(script) {
                // A space after the last cluster should be ignored,
                // since it affects the bbox and text alignment.
                if index != glyps_len - 1 {
                    glyph_token.x_advance += letter_spacing.at(font_size);
                }

                // If the cluster advance became negative - clear it.
                // This is an UB so we can do whatever we want, and we mimic Chrome's behavior.
                if !glyph_token.x_advance.is_finite() && glyph_token.x_advance < Abs::zero() {
                    glyph_token.x_advance = Abs::zero();
                }
            }
        }
    }

    /// Applies word-spacing to a span.
    ///
    /// [In the CSS spec](https://www.w3.org/TR/css-text-3/#propdef-word-spacing)
    pub fn apply_word_spacing(&mut self) {
        let word_spacing = self.attrs.get_word_spacing();
        if word_spacing.is_zero() {
            return;
        }

        for token_variant in self.tokens.iter_mut() {
            match token_variant {
                ShapeTokenVariant::WordSeparator(token) => {
                    if let Some(glyph_token) = token.get_tokens_mut().first_mut() {
                        // Technically, word spacing 'should be applied half on each
                        // side of the character', but it doesn't affect us in any way,
                        // so we are ignoring this.
                        glyph_token.x_advance += word_spacing.at(self.attrs.get_font_size());
                    }
                }
                _ => {}
            }
        }
    }

    pub fn iter_tokens<'a>(&'a self) -> impl Iterator<Item = &'a ShapeTokenVariant> + 'a {
        if self.is_rtl() {
            Either::Left(self.tokens.iter().rev())
        } else {
            Either::Right(self.tokens.iter())
        }
    }

    pub fn iter_tokens_in_range<'a>(
        &'a self,
        range: &'a Range<usize>,
    ) -> impl Iterator<Item = &'a ShapeTokenVariant> + 'a {
        self.iter_tokens()
            .filter(move |token| is_range_within(token.get_shape_token().get_range(), &range))
    }

    pub fn iter_tokens_mut<'a>(
        &'a mut self,
    ) -> impl Iterator<Item = &'a mut ShapeTokenVariant> + 'a {
        if self.is_rtl() {
            Either::Left(self.tokens.iter_mut().rev())
        } else {
            Either::Right(self.tokens.iter_mut())
        }
    }

    pub fn get_glyph_count(&self) -> usize {
        let mut length = 0;
        for token_variant in self.tokens.iter() {
            length += match token_variant {
                ShapeTokenVariant::Glyph(_) => 1,
                ShapeTokenVariant::TextFragment(token) => token.get_tokens().len(),
                ShapeTokenVariant::WordSeparator(token) => token.get_tokens().len(),
                _ => 0,
            };
        }
        return length;
    }

    pub fn iter_glyphs<'a>(&'a self) -> impl Iterator<Item = &'a GlyphToken> + 'a {
        self.iter_tokens()
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
        self.iter_glyphs()
            .filter(move |glyph| is_range_within(glyph.get_range(), &range))
    }

    pub(crate) fn iter_glyphs_mut<'a>(
        &'a mut self,
    ) -> impl Iterator<Item = &'a mut GlyphToken> + 'a {
        self.iter_tokens_mut()
            .flat_map(move |token_variant| match token_variant {
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
        self.iter_glyphs_mut()
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

        return Self { glyphs, index: 0 };
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
