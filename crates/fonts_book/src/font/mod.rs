pub mod info;
pub mod metrics;
pub mod variant;

use self::metrics::FontMetrics;
use dyn_utils::units::{abs::Abs, em::Em};
use rustybuzz::{ttf_parser::GlyphId, Face as RustybuzzFace};
use self_cell::self_cell;
use std::{
    fmt::{Debug, Formatter},
    sync::Arc,
};

self_cell!(
    pub(crate) struct OwnedRustybuzzFace {
        owner: Arc<dyn AsRef<[u8]> + Send + Sync>,

        #[covariant]
        dependent: RustybuzzFace,
    }
);

/// An OpenType font.
///
/// Values of this type are cheap to clone and hash.
#[derive(Clone)]
pub struct Font(Arc<InnerFont>);

struct InnerFont {
    /// The font's id in the fontdb.
    id: FontId,
    /// The font's index in the buffer.
    index: u32,
    /// The raw font data.
    data: Arc<dyn AsRef<[u8]> + Send + Sync>,
    /// The font's metrics.
    metrics: FontMetrics,
    /// The underlying rustybuzz face.
    rustybuzz: OwnedRustybuzzFace,
}

impl Font {
    pub fn new(id: FontId, data: Arc<dyn AsRef<[u8]> + Send + Sync>, index: u32) -> Option<Self> {
        let rustybuzz = OwnedRustybuzzFace::try_new(Arc::clone(&data), |data| {
            RustybuzzFace::from_slice((**data).as_ref(), index).ok_or(())
        })
        .ok()?;

        return Some(Self(Arc::new(InnerFont {
            id,
            index,
            data,
            metrics: FontMetrics::from_rustybuzz(rustybuzz.borrow_dependent()),
            rustybuzz,
        })));
    }

    /// The font's id in the fontdb.
    pub fn get_id(&self) -> FontId {
        self.0.id
    }

    /// The font's index in the buffer.
    pub fn get_index(&self) -> u32 {
        self.0.index
    }

    /// The underlying buffer.
    pub fn get_data(&self) -> &[u8] {
        (*self.0.data).as_ref()
    }

    /// The font's metrics.
    pub fn get_metrics(&self) -> &FontMetrics {
        &self.0.metrics
    }

    /// A reference to the underlying `rustybuzz` face.
    pub fn get_rustybuzz(&self) -> &rustybuzz::Face<'_> {
        &self.0.rustybuzz.borrow_dependent()
    }

    /// The number of font units per one em.
    pub fn get_units_per_em(&self) -> f32 {
        self.0.metrics.units_per_em
    }

    pub fn get_scale_factor(&self, font_size: Abs) -> Abs {
        font_size / self.get_units_per_em()
    }

    /// Convert from font units to an em length.
    pub fn to_em(&self, units: impl Into<f32>) -> Em {
        Em::from_units(units, self.get_units_per_em())
    }

    /// Look up the horizontal advance width of a glyph.
    pub fn advance(&self, glyph: u16) -> Option<Em> {
        self.get_rustybuzz()
            .glyph_hor_advance(GlyphId(glyph))
            .map(|units| self.to_em(units))
    }
}

impl Debug for Font {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Font")
            .field("id", &self.0.id)
            .finish_non_exhaustive()
    }
}

pub type FontId = fontdb::ID;
