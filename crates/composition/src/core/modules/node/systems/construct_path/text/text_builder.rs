use glam::Vec2;
use owned_ttf_parser::{GlyphId, OutlineBuilder};
use rustybuzz::{GlyphBuffer, UnicodeBuffer};

use crate::core::modules::node::components::mixins::{Anchor, AnchorCommand};

/// TextBuilder constructs text outlines for rendering.
/// It translates TTF parser's commands into a series of anchors,
/// managing paths and bezier curves.
pub struct TextBuilder {
    current_subpath: Vec<Anchor>,
    subpaths: Vec<Vec<Anchor>>,
    pos: Vec2,
    offset: Vec2,
    ascender: f32,
    scale: f32,
}

impl TextBuilder {
    pub fn initial() -> Self {
        Self {
            current_subpath: Vec::new(),
            subpaths: Vec::new(),
            pos: Vec2::ZERO,
            offset: Vec2::ZERO,
            ascender: 0.0,
            scale: 0.0,
        }
    }

    // Update TextBuilder for a new section
    pub fn update_for_new_section(&mut self, font_face: &rustybuzz::Face, font_size: u32) {
        let font_height = font_face.height();
        let ascender = font_face.ascender();
        self.scale = (font_face.units_per_em() as f32).recip() * font_size as f32;
        self.ascender = (ascender as f32 / font_height as f32) * font_size as f32 / self.scale;
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

    /// Moves the current position to the start of a new line.
    pub fn move_to_new_line(&mut self, line_height: f32) {
        self.pos = Vec2::new(0.0, self.pos.y + line_height);
    }

    /// Decides if a word should be wrapped to the next line based on the available width.
    pub fn should_wrap_word(
        line_width: f32,
        glyph_buffer: &GlyphBuffer,
        scale: f32,
        x_pos: f32,
    ) -> bool {
        let word_length: i32 = glyph_buffer
            .glyph_positions()
            .iter()
            .map(|pos| pos.x_advance)
            .sum();
        let scaled_word_length = word_length as f32 * scale;

        return scaled_word_length + x_pos > line_width;
    }

    /// Processes the glyphs of a text and constructs their paths.
    pub fn process_glyphs(
        &mut self,
        glyph_buffer: &GlyphBuffer,
        line_width: f32,
        line_height: f32,
        font_face: &rustybuzz::Face,
    ) {
        for (glyph_position, glyph_info) in glyph_buffer
            .glyph_positions()
            .iter()
            .zip(glyph_buffer.glyph_infos())
        {
            // Check if glyph exceeds line width and move to new line if needed
            if self.pos.x + (glyph_position.x_advance as f32 * self.scale) >= line_width {
                self.move_to_new_line(line_height);
            }

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

    /// Processes a line of text, breaking it into words and constructing their paths.
    pub fn process_line(
        &mut self,
        mut unicode_buffer: UnicodeBuffer, // Note: No reference as UnicodeBuffer can't be cloned
        line: &str,
        line_width: f32,
        line_height: f32,
        font_face: &rustybuzz::Face,
    ) -> UnicodeBuffer {
        let space = " ";
        let words = line.split(space);
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
            if TextBuilder::should_wrap_word(line_width, &glyph_buffer, self.scale, self.pos.x) {
                self.move_to_new_line(line_height);
            }

            // Render the glyphs and prepare the unicode buffer for the next iteration
            self.process_glyphs(&glyph_buffer, line_width, line_height, font_face);
            unicode_buffer = glyph_buffer.clear();
        }

        return unicode_buffer;
    }

    /// Converts the constructed paths into a flat vector of vertices.
    pub fn into_vertices(&mut self) -> Vec<Anchor> {
        self.subpaths.drain(..).flatten().collect()
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
