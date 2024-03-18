use crate::usvg::text::{AlignmentBaseline, BaselineShift, DominantBaseline, Font, LengthAdjust};
use ordered_float::OrderedFloat;
use rust_lapper::Interval;

// TODO: Make attribute partial? So that they can actually overlap in a useful manner.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Attribute {
    /// A font.
    pub font: Font,
    /// A font size.
    pub font_size: OrderedFloat<f32>,
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
    pub letter_spacing: OrderedFloat<f32>,
    /// A word spacing property.
    pub word_spacing: OrderedFloat<f32>,
    /// A text length property.
    pub text_length: Option<OrderedFloat<f32>>,
    /// A length adjust property.
    pub length_adjust: LengthAdjust,
}

pub type AttributeInterval = Interval<usize, Attribute>;
