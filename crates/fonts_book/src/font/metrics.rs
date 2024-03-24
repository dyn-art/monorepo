use dyn_utils::units::em::Em;

/// Metrics of a font.
#[derive(Debug, Clone, Copy)]
pub struct FontMetrics {
    /// How many font units represent one em unit.
    pub units_per_em: f32,
    /// The distance from the baseline to the typographic ascender
    /// relative the font size.
    pub ascender: Em,
    /// The distance from the baseline to the typographic descender
    /// relative the font size.
    pub descender: Em,
    /// The approximate height of uppercase letters relative the font size.
    pub cap_height: Em,
    /// The approximate height of non-ascending lowercase letters
    /// relative the font size.
    pub x_height: Em,
    /// Recommended metrics for a strikethrough line.
    pub strikethrough: LineMetrics,
    /// Recommended metrics for an underline.
    pub underline: LineMetrics,
    /// Recommended metrics for an overline.
    pub overline: LineMetrics,
}

impl FontMetrics {
    /// Extract the font's metrics from a rustybuzz font face.
    pub fn from_rustybuzz(face: &rustybuzz::Face) -> Self {
        let units_per_em = face.units_per_em() as f32;
        let to_em = |units| Em::from_units(units, units_per_em);

        let ascender = to_em(face.typographic_ascender().unwrap_or(face.ascender()));
        let cap_height = face
            .capital_height()
            .filter(|&h| h > 0)
            .map_or(ascender, to_em);
        let x_height = face.x_height().filter(|&h| h > 0).map_or(ascender, to_em);
        let descender = to_em(face.typographic_descender().unwrap_or(face.descender()));
        let strikeout = face.strikeout_metrics();
        let underline = face.underline_metrics();

        let strikethrough = LineMetrics {
            position: strikeout.map_or(Em::new(0.25), |s| to_em(s.position)),
            thickness: strikeout
                .or(underline)
                .map_or(Em::new(0.06), |s| to_em(s.thickness)),
        };

        let underline = LineMetrics {
            position: underline.map_or(Em::new(-0.2), |s| to_em(s.position)),
            thickness: underline
                .or(strikeout)
                .map_or(Em::new(0.06), |s| to_em(s.thickness)),
        };

        let overline = LineMetrics {
            position: cap_height + Em::new(0.1),
            thickness: underline.thickness,
        };

        Self {
            units_per_em,
            ascender,
            cap_height,
            x_height,
            descender,
            strikethrough,
            underline,
            overline,
        }
    }
}

/// Metrics for a decorative line.
#[derive(Debug, Copy, Clone)]
pub struct LineMetrics {
    /// The vertical offset of the line from the baseline relative the font size.
    /// Positive goes upwards, negative downwards.
    pub position: Em,
    /// The thickness of the line relative the font size.
    pub thickness: Em,
}
