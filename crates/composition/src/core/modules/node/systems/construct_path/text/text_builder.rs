use glam::Vec2;
use owned_ttf_parser::{GlyphId, OutlineBuilder};
use rustybuzz::{GlyphBuffer, UnicodeBuffer};

use crate::core::modules::{
    composition::resources::font_cache::FontCacheRes,
    node::components::{
        mixins::{Anchor, AnchorCommand},
        types::{Text, TextSection},
    },
};

use super::current_line::CurrentLine;

pub struct TextBuilder {
    subpaths: Vec<Vec<Anchor>>,
    current_subpath: Vec<Anchor>,
    pos: Vec2,
    offset: Vec2,
    ascender: f32,
    scale: f32,
    line_width: f32,
}

impl TextBuilder {
    pub fn new(line_width: f32) -> Self {
        Self {
            current_subpath: Vec::new(),
            subpaths: Vec::new(),
            pos: Vec2::ZERO,
            offset: Vec2::ZERO,
            ascender: 0.0,
            scale: 0.0,
            line_width,
        }
    }

    pub fn process_text(&mut self, text: &Text, font_cache: &mut FontCacheRes) {
        let lines: Vec<Vec<TextSection>> = TextBuilder::create_lines_from_sections(&text.sections);

        // Process lines
        for line in &lines {
            self.process_line(line, font_cache);
        }
    }

    /// Converts the constructed paths into a flat vector of vertices.
    pub fn into_vertices(&mut self) -> Vec<Anchor> {
        self.subpaths.drain(..).flatten().collect()
    }

    fn process_line(&mut self, line: &Vec<TextSection>, font_cache: &mut FontCacheRes) {
        let mut current_line = CurrentLine::new();

        // Build the current line with text and styles
        for section in line {
            current_line.add_section(&section.value, section.style.clone(), font_cache);
        }
        self.ascender = current_line.max_ascender;

        let mut unicode_buffer = UnicodeBuffer::new();

        // Process each styled range in the line
        for style_range in &current_line.style_ranges {
            let text_slice = &current_line.text[style_range.start..style_range.end];
            let font_hash = &style_range.style.font_hash;
            self.scale = style_range.metric.scale;

            if let Some(font_face) = font_cache.get_font_face(font_hash) {
                let space = " ";
                let words = text_slice.split(space);
                let word_count = words.clone().count();

                // Process words
                for (index, word) in words.enumerate() {
                    // Append to render word to the unicode buffer
                    unicode_buffer.push_str(word);
                    if index != word_count - 1 {
                        unicode_buffer.push_str(space);
                    }

                    // Shape the accumulated text in the unicode buffer
                    let glyph_buffer = rustybuzz::shape(&font_face, &[], unicode_buffer);

                    // Wrap to a new line if the current word exceeds the line width
                    if self.should_wrap_word(&glyph_buffer, style_range.metric.scale) {
                        // TODO: Recalculate current line as the max_height might have change
                        //  because complete style_ranges might have already been processed
                        self.move_to_new_line(current_line.max_height);
                    }

                    // Render the glyphs and prepare the unicode buffer for the next iteration
                    self.process_glyphs(&glyph_buffer, &font_face);
                    unicode_buffer = glyph_buffer.clear();
                }
            }
        }
    }

    /// Processes the glyphs of a text and constructs their paths.
    pub fn process_glyphs(&mut self, glyph_buffer: &GlyphBuffer, font_face: &rustybuzz::Face) {
        for (glyph_position, glyph_info) in glyph_buffer
            .glyph_positions()
            .iter()
            .zip(glyph_buffer.glyph_infos())
        {
            // Calculate and set the glyph offset for positioning
            self.offset = Vec2::new(
                glyph_position.x_offset as f32,
                glyph_position.y_offset as f32,
            ) * self.scale;

            // Outline the glyph and add it to the current path
            font_face.outline_glyph(GlyphId(glyph_info.glyph_id as u16), self);
            if !self.current_subpath.is_empty() {
                self.subpaths
                    .push(core::mem::replace(&mut self.current_subpath, Vec::new()));
            }

            // Update the position for the next glyph
            self.pos += Vec2::new(
                glyph_position.x_advance as f32,
                glyph_position.y_advance as f32,
            ) * self.scale;
        }
    }

    pub fn move_to_new_line(&mut self, line_height: f32) {
        self.pos = Vec2::new(0.0, self.pos.y + line_height);
    }

    /// Decides if a word should be wrapped to the next line based on the available width.
    fn should_wrap_word(&self, glyph_buffer: &GlyphBuffer, scale: f32) -> bool {
        let word_width: i32 = glyph_buffer
            .glyph_positions()
            .iter()
            .map(|pos| pos.x_advance)
            .sum();
        let scaled_word_width = word_width as f32 * scale;

        return scaled_word_width + self.pos.x > self.line_width;
    }

    /// Converts a point from local to global coordinates, scaling accordingly.
    fn point(&self, x: f32, y: f32) -> Vec2 {
        self.pos + self.offset + Vec2::new(x, self.ascender - y) * self.scale
    }

    /// Flushes the current subpath into other subpaths if it's not empty.
    fn flush_current_subpath(&mut self) {
        if !self.current_subpath.is_empty() {
            self.subpaths
                .push(std::mem::take(&mut self.current_subpath));
        }
    }

    /// Converts a series of text sections into a vector of lines
    /// based on hard coded line breaks like `\n`.
    fn create_lines_from_sections(text_sections: &[TextSection]) -> Vec<Vec<TextSection>> {
        let mut lines = Vec::new();
        let mut current_line = Vec::new();

        for section in text_sections {
            let section_lines: Vec<&str> = section.value.split('\n').collect();
            let total_section_lines = section_lines.len();

            for (index, section_line) in section_lines.iter().enumerate() {
                let text_section = TextSection {
                    value: String::from(*section_line),
                    style: section.style.clone(),
                };

                // If it's not the last part, push to current_line and then to lines
                if index < total_section_lines - 1 {
                    current_line.push(text_section);
                    lines.push(current_line.drain(..).collect());
                }
                // If it's the last part, just append to the current line
                // so that next section can append to it
                else {
                    current_line.push(text_section);
                }
            }
        }

        // Adding the last line if it's not empty
        if !current_line.is_empty() {
            lines.push(current_line);
        }

        return lines;
    }
}

impl OutlineBuilder for TextBuilder {
    /// Starts a new subpath at the given point.
    fn move_to(&mut self, x: f32, y: f32) {
        self.flush_current_subpath();
        self.current_subpath.push(Anchor {
            position: self.point(x, y),
            command: AnchorCommand::MoveTo,
        });
    }

    /// Adds a line to the current subpath.
    fn line_to(&mut self, x: f32, y: f32) {
        self.current_subpath.push(Anchor {
            position: self.point(x, y),
            command: AnchorCommand::LineTo,
        });
    }

    /// Converts a quadratic bezier curve to a cubic one and adds it to the current subpath.
    fn quad_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32) {
        let current_point = self.current_subpath.last().unwrap().position;
        let control_point = self.point(x1, y1);
        let end_point = self.point(x2, y2);

        // Convert quadratic to cubic bezier control points.
        let cubic_control_point1 = current_point + 2.0 / 3.0 * (control_point - current_point);
        let cubic_control_point2 = end_point + 2.0 / 3.0 * (control_point - end_point);

        self.current_subpath.push(Anchor {
            position: end_point,
            command: AnchorCommand::CurveTo {
                control_point_1: cubic_control_point1,
                control_point_2: cubic_control_point2,
            },
        });
    }

    /// Adds a cubic bezier curve to the current subpath.
    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32) {
        self.current_subpath.push(Anchor {
            position: self.point(x3, y3),
            command: AnchorCommand::CurveTo {
                control_point_1: self.point(x1, y1),
                control_point_2: self.point(x2, y2),
            },
        });
    }

    /// Closes the current subpath and adds it to other subpaths.
    fn close(&mut self) {
        if let Some(first_anchor) = self.current_subpath.first() {
            if first_anchor.command != AnchorCommand::ClosePath {
                self.current_subpath.push(Anchor {
                    position: first_anchor.position,
                    command: AnchorCommand::ClosePath,
                });
            }
        }
        self.flush_current_subpath();
    }
}
