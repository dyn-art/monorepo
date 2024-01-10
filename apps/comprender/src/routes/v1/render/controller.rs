use std::collections::HashMap;

use axum::{extract::Query, http::StatusCode, response::IntoResponse, Json};
use dyn_bevy_render_skeleton::RenderApp;
use dyn_composition::core::{composition::Composition, dtif::DTIFComposition};
use dyn_svg_render::{resources::svg_composition::SVGCompositionRes, SvgRenderPlugin};
use resvg::usvg::Options;
use serde::Deserialize;
use usvg::TreeParsing;

use crate::models::app_error::{AppError, ErrorCode};

#[derive(Deserialize)]
pub struct QueryParams {
    format: String,
}

pub async fn render_composition(
    Query(params): Query<QueryParams>,
    Json(body): Json<DTIFComposition>,
) -> Result<(StatusCode, Vec<u8>), impl IntoResponse> {
    let svg_result = generate_svg(body);

    match svg_result {
        Ok(svg_string) => {
            // Determine response format from query parameter
            match params.format.as_str() {
                "png" => {
                    // Convert SVG to PNG
                    let opts = Options::default();
                    let rtree = usvg::Tree::from_str(&svg_string, &opts).unwrap();
                    let pixmap_size = rtree.size.to_int_size();
                    let mut pixmap =
                        tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();
                    resvg::Tree::from_usvg(&rtree)
                        .render(tiny_skia::Transform::default(), &mut pixmap.as_mut());
                    let png_data = pixmap.encode_png().unwrap();

                    // Return PNG response
                    Ok((StatusCode::OK, png_data))
                }
                "svg" => {
                    // Return SVG response
                    Ok((StatusCode::OK, svg_string.into_bytes()))
                }
                _ => Err(
                    AppError::new(StatusCode::BAD_REQUEST, ErrorCode::new("INVALID_FORMAT"))
                        .into_response(),
                ),
            }
        }
        Err(e) => Err(e.into_response()),
    }
}

fn generate_svg(body: DTIFComposition) -> Result<String, AppError> {
    // Initialize composition
    let mut composition = Composition::new(Some(body));
    let app = composition.get_app_mut();

    // Register plugins
    app.add_plugins(SvgRenderPlugin {
        render_event_sender: None,
    });

    // Update app once
    app.update();

    // Attempt to retrieve the SVG string
    app.get_sub_app(RenderApp)
        .map_err(|_| {
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorCode::new("RENDER_APP_NOT_FOUND"),
            )
        })
        .and_then(|render_app| {
            render_app
                .world
                .get_resource::<SVGCompositionRes>()
                .ok_or_else(|| {
                    AppError::new(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        ErrorCode::new("SVG_COMPOSITION_RES_NOT_FOUND"),
                    )
                })
        })
        .and_then(|svg_composition_res| {
            svg_composition_res.to_string().ok_or_else(|| {
                AppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ErrorCode::new("SVG_CONVERSION_FAILED"),
                )
            })
        })
}
