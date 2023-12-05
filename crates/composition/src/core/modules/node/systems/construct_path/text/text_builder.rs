use bevy_utils::HashMap;
use glam::Vec2;
use owned_ttf_parser::{GlyphId, OutlineBuilder};
use rustybuzz::{GlyphBuffer, UnicodeBuffer};

use crate::core::modules::{
    composition::resources::font_cache::FontCacheRes,
    node::components::{
        mixins::{Anchor, AnchorCommand},
        types::{Text, TextStyle},
    },
};

pub struct TextBuilder {
    subpaths: Vec<Vec<Anchor>>,
    max_line_width: f32,

    // Current
    // TODO: improve
    current_subpath: Vec<Anchor>,
    current_pos: Vec2,
    current_offset: Vec2,
    current_ascender: f32,
    current_max_ascender: f32,
    current_scale: f32,
}

impl TextBuilder {
    pub fn new(max_line_width: f32) -> Self {
        Self {
            subpaths: Vec::new(),
            max_line_width,
            current_subpath: Vec::new(),
            current_pos: Vec2::ZERO,
            current_offset: Vec2::ZERO,
            current_ascender: 0.0,
            current_max_ascender: 0.0,
            current_scale: 0.0,
        }
    }

    pub fn process_text(&mut self, text: &Text, font_cache: &mut FontCacheRes) {
        let token_stream = TokenStream::from_text(text, font_cache);
        let lines = token_stream.into_lines();

        for line in &lines {
            self.process_line(line, &token_stream);
        }
    }

    pub fn process_line(&mut self, line: &Vec<&Token>, token_stream: &TokenStream) {
        let mut unicode_buffer = UnicodeBuffer::new();

        // Compute line style metric
        let mut line_style_metric = TokenStream::compute_line_style_metric(line);
        self.current_max_ascender = line_style_metric.max_ascender;

        // Move to a new line initially to ensure text
        // is within the view box and aligned at the common baseline
        self.move_to_new_line(line_style_metric.height);

        for (index, token) in line.iter().enumerate() {
            if let Token::Space { style, metric } | Token::TextFragment { style, metric, .. } =
                token
            {
                if let Some(font_face) = token_stream.get_buzz_face(style.font_hash) {
                    self.current_scale = metric.scale;
                    self.current_ascender = metric.ascender;

                    // Append to render string to the unicode buffer
                    unicode_buffer.push_str(match token {
                        Token::Space { .. } => " ",
                        Token::TextFragment { value, .. } => value.as_str(),
                        _ => "",
                    });

                    // Shape the accumulated text in the unicode buffer
                    let glyph_buffer = rustybuzz::shape(&font_face, &[], unicode_buffer);

                    // Wrap to a new line if the current word exceeds the line width
                    if self.should_wrap_word(&glyph_buffer, self.current_scale) {
                        line_style_metric = TokenStream::compute_line_style_metric(&line[index..]);
                        self.current_max_ascender = line_style_metric.max_ascender;
                        self.move_to_new_line(line_style_metric.height);
                    }

                    // Render the glyphs and prepare the unicode buffer for the next iteration
                    self.process_glyphs(&glyph_buffer, &font_face);
                    unicode_buffer = glyph_buffer.clear();
                }
            }
        }
    }

    /// Processes the glyphs of a text and constructs their paths.
    fn process_glyphs(&mut self, glyph_buffer: &GlyphBuffer, font_face: &rustybuzz::Face) {
        let baseline_adjustment = self.current_max_ascender - self.current_ascender;

        for (glyph_position, glyph_info) in glyph_buffer
            .glyph_positions()
            .iter()
            .zip(glyph_buffer.glyph_infos())
        {
            // Calculate and set the glyph offset for positioning
            self.current_offset = Vec2::new(
                glyph_position.x_offset as f32,
                glyph_position.y_offset as f32 - baseline_adjustment,
            ) * self.current_scale;

            // Outline the glyph and add it to the current path
            font_face.outline_glyph(GlyphId(glyph_info.glyph_id as u16), self);
            if !self.current_subpath.is_empty() {
                self.subpaths
                    .push(core::mem::replace(&mut self.current_subpath, Vec::new()));
            }

            // Update the position for the next glyph
            self.current_pos += Vec2::new(
                glyph_position.x_advance as f32,
                glyph_position.y_advance as f32,
            ) * self.current_scale;
        }
    }

    /// Decides if a word should be wrapped to the next line based on the available width.
    fn should_wrap_word(&self, glyph_buffer: &GlyphBuffer, scale: f32) -> bool {
        let word_width: i32 = glyph_buffer
            .glyph_positions()
            .iter()
            .map(|pos| pos.x_advance)
            .sum();
        let scaled_word_width = word_width as f32 * scale;

        return scaled_word_width + self.current_pos.x > self.max_line_width;
    }

    /// Moves the current position to the start of a new line.
    fn move_to_new_line(&mut self, line_height: f32) {
        self.current_pos = Vec2::new(0.0, self.current_pos.y + line_height);
    }

    /// Converts a point from local to global coordinates, scaling accordingly.
    fn point(&self, x: f32, y: f32) -> Vec2 {
        self.current_pos
            + self.current_offset
            + Vec2::new(x, self.current_ascender - y) * self.current_scale
    }

    /// Flushes the current subpath into other subpaths if it's not empty.
    fn flush_current_subpath(&mut self) {
        if !self.current_subpath.is_empty() {
            self.subpaths
                .push(std::mem::take(&mut self.current_subpath));
        }
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

#[derive(Debug)]
pub enum Token {
    TextFragment {
        value: String,
        style: TextStyle,
        metric: TokenStyleMetric,
    },
    Space {
        style: TextStyle,
        metric: TokenStyleMetric,
    },
    Linebreak,
}

pub struct TokenStream<'a> {
    tokens: Vec<Token>,
    buzz_face_cache: HashMap<u64, rustybuzz::Face<'a>>,
}

impl<'a> TokenStream<'a> {
    pub fn from_text(text: &Text, font_cache: &'a mut FontCacheRes) -> Self {
        let mut tokens: Vec<Token> = Vec::new();
        let mut font_face_cache: HashMap<u64, rustybuzz::Face<'a>> = HashMap::new();

        // Preload required faces to avoid mutable borrow conflicts during local font face caching
        for section in &text.sections {
            font_cache.load_ttfp_face(&section.style.font_hash);
        }

        // Iterate through text sections, creating tokens
        for section in &text.sections {
            let font_hash = section.style.font_hash;
            let font_size = section.style.font_size as f32;

            // Cache rustybuzz font face locally
            if !font_face_cache.contains_key(&font_hash) {
                if let Some(face) = font_cache.get_buzz_face(&section.style.font_hash) {
                    font_face_cache.insert(section.style.font_hash, face.clone());
                }
            }
            let buzz_face = font_face_cache.get(&font_hash).unwrap();
            let token_metric = Self::compute_token_style_metric(buzz_face, font_size);

            // Tokenize the text, considering spaces and line breaks
            let mut start = 0;
            for (index, match_str) in section
                .value
                .match_indices(|c: char| c.is_whitespace() || c == '\n')
            {
                // Create a text fragment token for non-whitespace sections
                if start != index {
                    tokens.push(Token::TextFragment {
                        value: String::from(&section.value[start..index]),
                        style: section.style.clone(),
                        metric: token_metric.clone(),
                    });
                }

                // Create a token for each space or line break
                tokens.push(match match_str {
                    "\n" => Token::Linebreak,
                    _ => Token::Space {
                        style: section.style.clone(),
                        metric: token_metric.clone(),
                    },
                });

                start = index + match_str.len();
            }

            // Handle the last word in the section, if any
            if start < section.value.len() {
                tokens.push(Token::TextFragment {
                    value: String::from(&section.value[start..]),
                    style: section.style.clone(),
                    metric: token_metric,
                });
            }
        }

        return Self {
            tokens,
            buzz_face_cache: font_face_cache,
        };
    }

    pub fn into_lines(&self) -> Vec<Vec<&Token>> {
        let mut lines: Vec<Vec<&Token>> = Vec::new();

        // Split tokens into lines at each Linebreak token
        let mut current_line: Vec<&Token> = Vec::new();
        for token in &self.tokens {
            match token {
                Token::Linebreak => {
                    lines.push(current_line.drain(..).collect());
                }
                _ => current_line.push(token),
            }
        }

        // Add the last line if it contains any tokens
        if !current_line.is_empty() {
            lines.push(current_line);
        }

        return lines;
    }

    pub fn get_buzz_face(&self, hash: u64) -> Option<&rustybuzz::Face> {
        self.buzz_face_cache.get(&hash)
    }

    pub fn compute_line_style_metric(line: &[&Token]) -> LineStyleMetric {
        line.iter().fold(
            LineStyleMetric {
                height: 0.0,
                max_ascender: 0.0,
            },
            |mut metrics, token| {
                match token {
                    Token::TextFragment { metric, .. } | Token::Space { metric, .. } => {
                        metrics.height = metrics.height.max(metric.height);
                        metrics.max_ascender = metrics.max_ascender.max(metric.ascender);
                    }
                    _ => {}
                }
                metrics
            },
        )
    }

    fn compute_token_style_metric(
        buzz_face: &rustybuzz::Face<'a>,
        font_size: f32,
    ) -> TokenStyleMetric {
        let scale = font_size / (buzz_face.units_per_em() as f32);
        let ascender = buzz_face.ascender() as f32 * scale;
        let descender = buzz_face.descender() as f32 * scale;
        buzz_face.height();
        TokenStyleMetric {
            ascender,
            descender,
            height: ascender - descender,
            scale,
        }
    }
}

#[derive(Clone, Debug)]
pub struct TokenStyleMetric {
    pub height: f32,
    pub ascender: f32,
    pub descender: f32,
    pub scale: f32,
}

#[derive(Debug)]
pub struct LineStyleMetric {
    pub height: f32,
    pub max_ascender: f32,
}
