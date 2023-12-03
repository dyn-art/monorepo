use bevy_ecs::{
    entity::Entity,
    query::{Changed, Or},
    system::{Commands, Query, ResMut},
};
use glam::Vec2;
use log::info;
use owned_ttf_parser::{GlyphId, OutlineBuilder};
use rustybuzz::{GlyphBuffer, UnicodeBuffer};

use crate::core::modules::{
    composition::resources::font_cache::FontCacheRes,
    node::components::{
        mixins::{Anchor, AnchorCommand, DimensionMixin, PathMixin, RectangleCornerMixin},
        types::Text,
    },
};

// =============================================================================
// Rectangle
// =============================================================================

pub fn construct_rectangle_path(
    mut commands: Commands,
    query: Query<
        (Entity, &RectangleCornerMixin, &DimensionMixin),
        Or<(Changed<RectangleCornerMixin>, Changed<DimensionMixin>)>,
    >,
) {
    for (entity, corners, dimension) in query.iter() {
        let mut path = PathMixin {
            vertices: Vec::new(),
        };
        let max_radius = std::cmp::min(dimension.width, dimension.height) as f32 / 2.0;

        let min_radius =
            |radius: u8| -> f32 { std::cmp::min(radius as i32, max_radius as i32) as f32 };

        // Move to start point, considering the top left radius
        path.vertices.push(Anchor {
            position: Vec2::new(min_radius(corners.top_left_radius), 0.0),
            command: AnchorCommand::MoveTo,
        });

        // Top right corner
        path.vertices.push(Anchor {
            position: Vec2::new(
                dimension.width as f32 - min_radius(corners.top_right_radius),
                0.0,
            ),
            command: AnchorCommand::LineTo,
        });

        if corners.top_right_radius > 0 {
            path.vertices.push(Anchor {
                position: Vec2::new(dimension.width as f32, min_radius(corners.top_right_radius)),
                command: AnchorCommand::ArcTo {
                    radius: Vec2::splat(min_radius(corners.top_right_radius)),
                    x_axis_rotation: 0.0,
                    large_arc_flag: false,
                    sweep_flag: true,
                },
            });
        }

        // Bottom right corner
        path.vertices.push(Anchor {
            position: Vec2::new(
                dimension.width as f32,
                dimension.height as f32 - min_radius(corners.bottom_right_radius),
            ),
            command: AnchorCommand::LineTo,
        });

        if corners.bottom_right_radius > 0 {
            path.vertices.push(Anchor {
                position: Vec2::new(
                    dimension.width as f32 - min_radius(corners.bottom_right_radius),
                    dimension.height as f32,
                ),
                command: AnchorCommand::ArcTo {
                    radius: Vec2::splat(min_radius(corners.bottom_right_radius)),
                    x_axis_rotation: 0.0,
                    large_arc_flag: false,
                    sweep_flag: true,
                },
            });
        }

        // Bottom left corner
        path.vertices.push(Anchor {
            position: Vec2::new(
                min_radius(corners.bottom_left_radius),
                dimension.height as f32,
            ),
            command: AnchorCommand::LineTo,
        });

        if corners.bottom_left_radius > 0 {
            path.vertices.push(Anchor {
                position: Vec2::new(
                    0.0,
                    dimension.height as f32 - min_radius(corners.bottom_left_radius),
                ),
                command: AnchorCommand::ArcTo {
                    radius: Vec2::splat(min_radius(corners.bottom_left_radius)),
                    x_axis_rotation: 0.0,
                    large_arc_flag: false,
                    sweep_flag: true,
                },
            });
        }

        // Back to top left corner
        path.vertices.push(Anchor {
            position: Vec2::new(0.0, min_radius(corners.top_left_radius)),
            command: AnchorCommand::LineTo,
        });

        if corners.top_left_radius > 0 {
            path.vertices.push(Anchor {
                position: Vec2::new(min_radius(corners.top_left_radius), 0.0),
                command: AnchorCommand::ArcTo {
                    radius: Vec2::splat(min_radius(corners.top_left_radius)),
                    x_axis_rotation: 0.0,
                    large_arc_flag: false,
                    sweep_flag: true,
                },
            });
        }

        // Close the path
        path.vertices.push(Anchor {
            position: Vec2::ZERO,
            command: AnchorCommand::ClosePath,
        });

        // Insert or update the PathMixin component for the entity
        commands.entity(entity).insert(path);
    }
}

// =============================================================================
// Text
// =============================================================================

struct TextBuilder {
    current_subpath: Vec<Anchor>,
    other_subpaths: Vec<Vec<Anchor>>,
    pos: Vec2,
    offset: Vec2,
    ascender: f32,
    scale: f32,
}

impl TextBuilder {
    fn point(&self, x: f32, y: f32) -> Vec2 {
        self.pos + self.offset + Vec2::new(x, self.ascender - y) * self.scale
    }
}

impl OutlineBuilder for TextBuilder {
    fn move_to(&mut self, x: f32, y: f32) {
        if !self.current_subpath.is_empty() {
            self.other_subpaths
                .push(std::mem::take(&mut self.current_subpath));
        }
        self.current_subpath.push(Anchor {
            position: self.point(x, y),
            command: AnchorCommand::MoveTo,
        });
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.current_subpath.push(Anchor {
            position: self.point(x, y),
            command: AnchorCommand::LineTo,
        });
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32) {
        // Quadratic bezier curves can be represented as a special case of cubic bezier curves.
        // Hence, we'll convert the quadratic control points to cubic ones.
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

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32) {
        self.current_subpath.push(Anchor {
            position: self.point(x3, y3),
            command: AnchorCommand::CurveTo {
                control_point_1: self.point(x1, y1),
                control_point_2: self.point(x2, y2),
            },
        });
    }

    fn close(&mut self) {
        // If the path is not empty and not already closed, add a ClosePath command.
        if let Some(first_anchor) = self.current_subpath.first() {
            if first_anchor.command != AnchorCommand::ClosePath {
                self.current_subpath.push(Anchor {
                    position: first_anchor.position,
                    command: AnchorCommand::ClosePath,
                });
            }
        }
        self.other_subpaths
            .push(std::mem::take(&mut self.current_subpath));
    }
}

pub fn construct_text_path(
    mut commands: Commands,
    mut font_cache: ResMut<FontCacheRes>,
    query: Query<(Entity, &Text, &DimensionMixin), Or<(Changed<Text>, Changed<DimensionMixin>)>>,
) {
    for (entity, text, dimension) in query.iter() {
        let mut path = PathMixin {
            vertices: Vec::new(),
        };

        // TODO
        info!("construct_text_path for {:?} - {:#?}", entity, text);

        // Construct path for text sections
        for section in &text.sections {
            if let Some(cached_font) = font_cache.get_mut(&section.style.font_hash) {
                // Load font face
                if let Some(font_face) = cached_font.get_or_create_face() {
                    let font_size = section.style.font_size;
                    let scale = (font_face.units_per_em() as f32).recip() * font_size as f32;
                    let mut unicode_buffer = UnicodeBuffer::new();

                    let mut text_builder = TextBuilder {
                        current_subpath: Vec::new(),
                        other_subpaths: Vec::new(),
                        pos: Vec2::ZERO,
                        offset: Vec2::ZERO,
                        ascender: (font_face.ascender() as f32 / font_face.height() as f32)
                            * font_size as f32
                            / scale,
                        scale,
                    };

                    // Go line by line
                    for line in section.value.split('\n') {
                        let lines = line.split(' ');
                        let line_length = lines.clone().count();
                        let line_width = dimension.width as f32;
                        let line_height = font_size as f32;

                        // Go word by word
                        for (index, word) in lines.enumerate() {
                            push_str(&mut unicode_buffer, word, index != line_length - 1);
                            let glyph_buffer = rustybuzz::shape(&font_face, &[], unicode_buffer);

                            // Wrap word
                            if wrap_word(Some(line_width), &glyph_buffer, scale, text_builder.pos.x)
                            {
                                text_builder.pos = Vec2::new(0., text_builder.pos.y + line_height);
                            }

                            for (glyph_position, glyph_info) in glyph_buffer
                                .glyph_positions()
                                .iter()
                                .zip(glyph_buffer.glyph_infos())
                            {
                                if text_builder.pos.x
                                    + (glyph_position.x_advance as f32 * text_builder.scale)
                                    >= line_width
                                {
                                    text_builder.pos =
                                        Vec2::new(0.0, text_builder.pos.y + line_height);
                                }
                                text_builder.offset = Vec2::new(
                                    glyph_position.x_offset as f32,
                                    glyph_position.y_offset as f32,
                                ) * text_builder.scale;
                                font_face.outline_glyph(
                                    GlyphId(glyph_info.glyph_id as u16),
                                    &mut text_builder,
                                );
                                if !text_builder.current_subpath.is_empty() {
                                    text_builder.other_subpaths.push(core::mem::replace(
                                        &mut text_builder.current_subpath,
                                        Vec::new(),
                                    ));
                                }

                                text_builder.pos += Vec2::new(
                                    glyph_position.x_advance as f32,
                                    glyph_position.y_advance as f32,
                                ) * text_builder.scale;
                            }

                            unicode_buffer = glyph_buffer.clear();
                        }
                        text_builder.pos = Vec2::new(0.0, text_builder.pos.y + line_height);
                    }
                    let mut merged_vector: Vec<Anchor> =
                        text_builder.other_subpaths.into_iter().flatten().collect();
                    path.vertices.append(&mut merged_vector);
                }
            }
        }

        // Insert or update the PathMixin component for the entity
        commands.entity(entity).insert(path);
    }
}

fn push_str(buffer: &mut UnicodeBuffer, word: &str, trailing_space: bool) {
    buffer.push_str(word);

    if trailing_space {
        buffer.push_str(" ");
    }
}

fn wrap_word(line_width: Option<f32>, glyph_buffer: &GlyphBuffer, scale: f32, x_pos: f32) -> bool {
    if let Some(line_width) = line_width {
        let word_length: i32 = glyph_buffer
            .glyph_positions()
            .iter()
            .map(|pos| pos.x_advance)
            .sum();
        let scaled_word_length = word_length as f32 * scale;

        if scaled_word_length + x_pos > line_width {
            return true;
        }
    }

    return false;
}
