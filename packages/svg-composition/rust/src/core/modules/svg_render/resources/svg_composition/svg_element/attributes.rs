use serde::Serialize;
use specta::Type;

// https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/d
#[derive(Debug, Serialize, Clone, Type)]
pub enum SVGPathCommand {
    // (x, y)
    MoveTo(f32, f32),
    // (x, y)
    LineTo(f32, f32),
    // (cx1, cy1, cx2, cy2, x, y)
    CurveTo(f32, f32, f32, f32, f32, f32),
    // (rx, ry, x_axis_rotation, large_arc_flag, sweep_flag, x, y)
    ArcTo(f32, f32, f32, u8, u8, f32, f32),
    ClosePath,
}

// https://developer.mozilla.org/en-US/docs/Web/CSS/transform-function
#[derive(Debug, Serialize, Clone, Type)]
pub enum SVGTransformAttribute {
    // matrix(a, b, c, d, tx, ty)
    Matrix(f32, f32, f32, f32, f32, f32),
}

#[derive(Debug, Serialize, Clone, Type)]
// #[serde(tag = "type")] // TODO: Serialize tag can't be used with tuples -> Restructure
pub enum SVGAttribute {
    Id(u32),
    Width(u32),
    Height(u32),
    Opacity(u8),
    Transform(SVGTransformAttribute),
    D(Vec<SVGPathCommand>),
    ClipPath(u32),
    Fill(String),
    Name(String),
}

impl SVGAttribute {
    pub fn key(&self) -> &'static str {
        match self {
            Self::Id(_) => "id",
            Self::Width(_) => "width",
            Self::Height(_) => "height",
            Self::Opacity(_) => "opacity",
            Self::Transform(_) => "transform",
            Self::D(_) => "d",
            Self::ClipPath(_) => "clipPath",
            Self::Fill(_) => "fill",
            Self::Name(_) => "name",
        }
    }

    pub fn to_svg_string(&self) -> String {
        match self {
            Self::Id(value) => value.to_string(),
            Self::Width(value) => value.to_string(),
            Self::Height(value) => value.to_string(),
            Self::Opacity(value) => value.to_string(),
            Self::Transform(value) => match value {
                SVGTransformAttribute::Matrix(a, b, c, d, tx, ty) => {
                    format!("matrix({a}, {b}, {c}, {d}, {tx}, {ty})")
                }
            },
            Self::D(path_data) => path_data
                .iter()
                .map(|command| match command {
                    SVGPathCommand::MoveTo(x, y) => format!("M{} {}", x, y),
                    SVGPathCommand::LineTo(x, y) => format!("L{} {}", x, y),
                    SVGPathCommand::CurveTo(cx1, cy1, cx2, cy2, x, y) => {
                        format!("C{} {} {} {} {} {}", cx1, cy1, cx2, cy2, x, y)
                    }
                    SVGPathCommand::ArcTo(
                        rx,
                        ry,
                        x_axis_rotation,
                        large_arc_flag,
                        sweep_flag,
                        x,
                        y,
                    ) => format!(
                        "A{} {} {} {} {} {} {}",
                        rx, ry, x_axis_rotation, large_arc_flag, sweep_flag, x, y
                    ),
                    SVGPathCommand::ClosePath => "Z".to_string(),
                })
                .collect::<Vec<_>>()
                .join(" "),
            Self::ClipPath(id) => format!("url(#{id})"),
            Self::Fill(value) => value.clone(),
            Self::Name(value) => value.clone(),
        }
    }
}
