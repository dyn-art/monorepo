use std::error::Error;

pub fn string_to_tiny_skia_path(value: &String) -> Result<tiny_skia_path::Path, Box<dyn Error>> {
    let mut builder = tiny_skia_path::PathBuilder::new();
    let commands = value.split_whitespace();

    let mut command_iter = commands.peekable();
    while let Some(command) = command_iter.next() {
        match command {
            "M" => {
                if let (Some(x), Some(y)) = (command_iter.next(), command_iter.next()) {
                    builder.move_to(x.parse::<f32>()?, y.parse::<f32>()?);
                }
            }
            "L" => {
                if let (Some(x), Some(y)) = (command_iter.next(), command_iter.next()) {
                    builder.line_to(x.parse::<f32>()?, y.parse::<f32>()?);
                }
            }
            "Q" => {
                if let (Some(x1), Some(y1), Some(x2), Some(y2)) = (
                    command_iter.next(),
                    command_iter.next(),
                    command_iter.next(),
                    command_iter.next(),
                ) {
                    builder.quad_to(
                        x1.parse::<f32>()?,
                        y1.parse::<f32>()?,
                        x2.parse::<f32>()?,
                        y2.parse::<f32>()?,
                    );
                }
            }
            "C" => {
                if let (Some(x1), Some(y1), Some(x2), Some(y2), Some(x3), Some(y3)) = (
                    command_iter.next(),
                    command_iter.next(),
                    command_iter.next(),
                    command_iter.next(),
                    command_iter.next(),
                    command_iter.next(),
                ) {
                    builder.cubic_to(
                        x1.parse::<f32>()?,
                        y1.parse::<f32>()?,
                        x2.parse::<f32>()?,
                        y2.parse::<f32>()?,
                        x3.parse::<f32>()?,
                        y3.parse::<f32>()?,
                    );
                }
            }
            "Z" => {
                builder.close();
            }
            _ => {}
        }
    }

    return builder.finish().ok_or(Box::new(FailedToParseStringPath));
}

#[derive(Debug, Clone)]
pub struct FailedToParseStringPath;

impl std::fmt::Display for FailedToParseStringPath {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Failed to parse string path!")
    }
}

impl std::error::Error for FailedToParseStringPath {}
