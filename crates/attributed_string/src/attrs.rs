use dyn_fonts_book::font::{
    info::{FontFamily, FontInfo},
    variant::{FontStretch, FontStyle, FontVariant, FontWeight},
    FontId,
};
use dyn_utils::units::abs::Abs;
use rust_lapper::Interval;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Attrs {
    pub font_id: Option<FontId>,
    pub font_family: Option<FontFamily>,
    pub font_style: Option<FontStyle>,
    pub font_stretch: Option<FontStretch>,
    pub font_weight: Option<FontWeight>,
    pub font_size: Option<Abs>,
    pub small_caps: Option<bool>,
    pub apply_kerning: Option<bool>,
}

impl Attrs {
    pub fn new() -> Self {
        Self {
            font_id: None,
            font_family: None,
            font_style: None,
            font_stretch: None,
            font_weight: None,
            font_size: None,
            small_caps: None,
            apply_kerning: None,
        }
    }

    pub fn font_id(mut self, font_id: FontId) -> Self {
        self.font_id = Some(font_id);
        self
    }

    pub fn get_font_id(&self) -> Option<FontId> {
        self.font_id
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
        self.font_stretch.unwrap_or(FontStretch::NORMAL)
    }

    pub fn font_weight(mut self, font_weight: FontWeight) -> Self {
        self.font_weight = Some(font_weight);
        self
    }

    pub fn get_font_weight(&self) -> FontWeight {
        self.font_weight.unwrap_or(FontWeight::REGULAR)
    }

    pub fn font_size(mut self, font_size: Abs) -> Self {
        self.font_size = Some(font_size);
        self
    }

    pub fn get_font_size(&self) -> Abs {
        self.font_size.map(|fs| fs).unwrap_or(Abs::pt(16.0))
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

    pub fn get_font_info(&self) -> FontInfo {
        FontInfo {
            family: self.get_font_family().clone(),
            variant: FontVariant::new(
                self.get_font_style(),
                self.get_font_weight(),
                self.get_font_stretch(),
            ),
        }
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
