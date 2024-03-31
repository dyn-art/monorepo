use crate::AttributedString;
use dyn_fonts_book::{font::Font, FontsBook};
use dyn_utils::units::abs::Abs;
use rust_lapper::Interval;
use rustybuzz::ttf_parser::{self, GlyphId};

pub struct TinySkiaPathBuilder;

impl TinySkiaPathBuilder {
    pub fn outline_glyph(glyph_id: GlyphId, font: &Font) -> Option<tiny_skia_path::Path> {
        let mut builder = GlyphPathBuilder::new();
        font.get_rustybuzz().outline_glyph(glyph_id, &mut builder)?;
        return builder.builder.finish();
    }

    // TODO: Not performant and needs caching for outlined glyphs
    pub fn outline(
        attributed_string: &AttributedString,
        fonts_book: &mut FontsBook,
    ) -> Option<tiny_skia_path::Path> {
        let mut text_builder = tiny_skia_path::PathBuilder::new();

        for Interval { val: span, .. } in attributed_string.get_spans().iter() {
            let mut span_builder = tiny_skia_path::PathBuilder::new();

            let font_size = span.get_attrs().get_font_size();

            for (cluster, _) in span.iter_glyph_clusters() {
                let mut cluster_builder = tiny_skia_path::PathBuilder::new();
                let mut width = Abs::zero();
                let mut x = Abs::zero();

                for glyph_token in cluster {
                    let font = match fonts_book.get_font_by_id(glyph_token.get_glyph().font_id) {
                        Some(v) => v,
                        None => continue,
                    };
                    let sx = font.get_scale_factor(font_size);

                    if let Some(outline) =
                        Self::outline_glyph(glyph_token.get_glyph().glyph_id, &font)
                    {
                        // By default, glyphs are upside-down, so we have to mirror them
                        let mut transform = tiny_skia_path::Transform::from_scale(1.0, -1.0);

                        // Scale to font-size
                        transform = transform.pre_scale(sx.to_pt(), sx.to_pt());

                        // Apply offset and transform.
                        //
                        // The first glyph in the cluster will have an offset from 0x0,
                        // but the later one will have an offset from the "current position".
                        // So we have to keep an advance.
                        transform.tx += (x + glyph_token.get_glyph().x_offset.at(font_size))
                            .to_pt()
                            + glyph_token.transform.tx;
                        transform.ty += glyph_token.get_glyph().y_offset.at(font_size).to_pt()
                            + glyph_token.transform.ty;

                        if let Some(outline) = outline.transform(transform) {
                            cluster_builder.push_path(&outline);
                        }
                    }

                    x += glyph_token.x_advance;

                    let glyph_width = glyph_token.x_advance;
                    if glyph_width > width {
                        width = glyph_width;
                    }
                }

                if let Some(path) = cluster_builder.finish() {
                    span_builder.push_path(&path);
                }
            }

            if let Some(path) = span_builder.finish() {
                text_builder.push_path(&path);
            }
        }

        // Draw bounding box
        // let rect_path = tiny_skia_path::PathBuilder::from_rect(
        //     tiny_skia_path::Rect::from_xywh(
        //         0.0,
        //         0.0,
        //         self.config.size.width(),
        //         self.config.size.height(),
        //     )
        //     .unwrap(),
        // );
        // let stroked_rect_path = tiny_skia_path::PathStroker::new()
        //     .stroke(
        //         &rect_path,
        //         &tiny_skia_path::Stroke {
        //             width: 1.0,
        //             ..Default::default()
        //         },
        //         1.0,
        //     )
        //     .unwrap();
        // text_builder.push_path(&stroked_rect_path);

        return text_builder.finish();
    }
}

pub struct GlyphPathBuilder {
    builder: tiny_skia_path::PathBuilder,
}

impl GlyphPathBuilder {
    pub fn new() -> Self {
        Self {
            builder: tiny_skia_path::PathBuilder::new(),
        }
    }
}

impl ttf_parser::OutlineBuilder for GlyphPathBuilder {
    fn move_to(&mut self, x: f32, y: f32) {
        self.builder.move_to(x, y);
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.builder.line_to(x, y);
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.builder.quad_to(x1, y1, x, y);
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.builder.cubic_to(x1, y1, x2, y2, x, y);
    }

    fn close(&mut self) {
        self.builder.close();
    }
}
