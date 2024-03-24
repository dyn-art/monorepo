pub mod path_builder;

use self::path_builder::PathBuilder;
use dyn_fonts_book::font::Font;
use rustybuzz::ttf_parser::GlyphId;

pub fn outline(glyph_id: GlyphId, font: &Font) -> Option<tiny_skia_path::Path> {
    let mut builder = PathBuilder {
        builder: tiny_skia_path::PathBuilder::new(),
    };
    font.get_rustybuzz().outline_glyph(glyph_id, &mut builder)?;
    builder.builder.finish()
}
