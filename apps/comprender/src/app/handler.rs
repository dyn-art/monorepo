use axum::{http::StatusCode, response::IntoResponse, Json};
use dyn_bevy_render_skeleton::RenderApp;
use dyn_composition::core::{composition::Composition, dtif::DTIFComposition};
use dyn_svg_render::{resources::svg_composition::SVGCompositionRes, SvgRenderPlugin};

use super::model::app_error::{AppError, ErrorCode};

pub async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Server is up and running!";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}

pub async fn render_composition(
    Json(body): Json<DTIFComposition>,
) -> Result<(StatusCode, String), (StatusCode, Json<AppError>)> {
    // Initalize composition
    let mut composition = Composition::new(Some(body));
    let app = composition.get_app_mut();

    // Register plugins
    app.add_plugins(SvgRenderPlugin {
        render_event_sender: None,
    });

    // Update app once
    app.update();

    // Get SVG string
    let maybe_render_app = app.get_sub_app(RenderApp).ok();
    if let Some(render_app) = maybe_render_app {
        let maybe_svg_composition_res = render_app.world.get_resource::<SVGCompositionRes>();
        if let Some(svg_composition_res) = maybe_svg_composition_res {
            if let Some(svg_string) = svg_composition_res.to_string() {
                return Ok((StatusCode::OK, svg_string));
            }
        }
    }

    // let body = Body::from_stream(stream);

    return Err((
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            ErrorCode::new("UNKOWN"),
        )),
    ));
}
