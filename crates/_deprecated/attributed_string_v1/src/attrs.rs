use crate::usvg::text::{FontFamily, FontStretch, FontStyle};
use ordered_float::OrderedFloat;
use rust_lapper::Interval;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Attrs {
    font_family: Option<FontFamily>,
    font_style: Option<FontStyle>,
    font_stretch: Option<FontStretch>,
    font_weight: Option<u16>,
    font_size: Option<OrderedFloat<f32>>,
    small_caps: Option<bool>,
    apply_kerning: Option<bool>,
    // dominant_baseline: DominantBaseline,
    // alignment_baseline: AlignmentBaseline,
    // letter_spacing: OrderedFloat<f32>,
    // word_spacing: OrderedFloat<f32>,
}

impl Attrs {
    pub fn new() -> Self {
        Self {
            font_family: None,
            font_style: None,
            font_stretch: None,
            font_weight: None,
            font_size: None,
            small_caps: None,
            apply_kerning: None,
        }
    }

    pub fn font_family(mut self, font_family: FontFamily) -> Self {
        self.font_family = Some(font_family);
        self
    }

    pub fn get_font_family<'a>(&'a self) -> &'a FontFamily {
        match self.font_family.as_ref() {
            Some(v) => v,
            None => &FontFamily::SansSerif,
        }
    }

    pub fn font_style(mut self, font_style: FontStyle) -> Self {
        self.font_style = Some(font_style);
        self
    }

    pub fn get_font_style(&self) -> FontStyle {
        self.font_style.unwrap_or(FontStyle::Normal)
    }

    pub fn font_stretch(mut self, font_stretch: FontStretch) -> Self {
        self.font_stretch = Some(font_stretch);
        self
    }

    pub fn get_font_stretch(&self) -> FontStretch {
        self.font_stretch.unwrap_or(FontStretch::Normal)
    }

    pub fn font_weight(mut self, font_weight: u16) -> Self {
        self.font_weight = Some(font_weight);
        self
    }

    pub fn get_font_weight(&self) -> u16 {
        self.font_weight.unwrap_or(400)
    }

    pub fn font_size(mut self, font_size: f32) -> Self {
        self.font_size = Some(OrderedFloat(font_size));
        self
    }

    pub fn get_font_size(&self) -> f32 {
        self.font_size.map(|fs| fs.0).unwrap_or(16.0)
    }

    pub fn small_caps(mut self, small_caps: bool) -> Self {
        self.small_caps = Some(small_caps);
        self
    }

    pub fn get_small_caps(&self) -> bool {
        self.small_caps.unwrap_or(false)
    }

    pub fn apply_kerning(mut self, apply_kerning: bool) -> Self {
        self.apply_kerning = Some(apply_kerning);
        self
    }

    pub fn get_apply_kerning(&self) -> bool {
        self.apply_kerning.unwrap_or(false)
    }

    pub fn merge(&mut self, to_merge_attrs: Attrs) {
        if self.font_family.is_none() {
            self.font_family = to_merge_attrs.font_family;
        }
        if self.font_style.is_none() {
            self.font_style = to_merge_attrs.font_style;
        }
        if self.font_stretch.is_none() {
            self.font_stretch = to_merge_attrs.font_stretch;
        }
        if self.font_weight.is_none() {
            self.font_weight = to_merge_attrs.font_weight;
        }
        if self.font_size.is_none() {
            self.font_size = to_merge_attrs.font_size;
        }
        if self.small_caps.is_none() {
            self.small_caps = to_merge_attrs.small_caps;
        }
        if self.apply_kerning.is_none() {
            self.apply_kerning = to_merge_attrs.apply_kerning;
        }
    }
}

pub type AttrsInterval = Interval<usize, Attrs>;
