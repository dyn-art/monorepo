use std::collections::HashMap;

use axum::{
    body::Body,
    extract::Query,
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use dyn_bevy_render_skeleton::RenderApp;
use dyn_composition::core::{
    composition::Composition, dtif::DTIFComposition,
    modules::composition::resources::font_cache::font::FontContent,
};
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
    Json(mut body): Json<DTIFComposition>,
) -> Result<Response, impl IntoResponse> {
    let _ = prepare_composition(&mut body).await;
    let svg_result = generate_svg(body);

    return match svg_result {
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
                    Ok(Response::builder()
                        .status(StatusCode::OK)
                        .header(header::CONTENT_TYPE, "image/png")
                        .body(Body::from(png_data))
                        .unwrap())
                }
                "svg" => {
                    // Return SVG response
                    Ok(Response::builder()
                        .status(StatusCode::OK)
                        .header(header::CONTENT_TYPE, "image/svg+xml")
                        .body(Body::from(svg_string.into_bytes()))
                        .unwrap())
                }
                "pdf" => {
                    // Convert SVG to PDF
                    let pdf_data =
                        svg2pdf::convert_str(&svg_string, svg2pdf::Options::default()).unwrap();

                    // Return PDF response
                    Ok(Response::builder()
                        .status(StatusCode::OK)
                        .header(header::CONTENT_TYPE, "application/pdf")
                        .body(Body::from(pdf_data))
                        .unwrap())
                }
                _ => Err(
                    AppError::new(StatusCode::BAD_REQUEST, ErrorCode::new("INVALID_FORMAT"))
                        .into_response(),
                ),
            }
        }
        Err(e) => Err(e.into_response()),
    };
}

async fn prepare_composition(composition: &mut DTIFComposition) -> Result<(), reqwest::Error> {
    // Resolve font urls
    if let Some(fonts) = &mut composition.fonts {
        let mut url_contents: HashMap<String, Vec<u8>> = HashMap::new();

        for (id, font) in fonts.iter() {
            if let FontContent::Url { url } = &font.content {
                let content = reqwest::get(url).await?.bytes().await?.to_vec();
                url_contents.insert(id.clone(), content);
            }
        }

        for (id, content) in url_contents {
            if let Some(font) = fonts.get_mut(&id) {
                font.content = FontContent::Binary { content };
            }
        }
    }

    return Ok(());
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
    return app
        .get_sub_app(RenderApp)
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
        });
}