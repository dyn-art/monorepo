use crate::error::app_error::{AppError, AppErrorOptions, ErrorCode};
use axum::{
    body::Body,
    http::{header, StatusCode},
    response::Response,
};
use axum::{routing::post, Router};
use usvg::{Options, Tree, WriteOptions};

pub fn routes() -> Router {
    Router::new().route("/", post(handler))
}

#[utoipa::path(
    post,
    path = "/v1/svg/simplify",
    request_body(content = String, description = "SVG input as string"),
    responses(
        (status = 200, description = "Generated SVG", body = String),
        (status = 400, description = "Invalid SVG input", body = String),
        (status = 500, description = "Internal server error", body = String)
    )
)]
async fn handler(body: String) -> Result<Response, AppError> {
    let opts = Options::default();

    let tree = Tree::from_str(&body, &opts).map_err(|err| {
        AppError::new_with_options(
            StatusCode::BAD_REQUEST,
            ErrorCode::new("INVALID_SVG"),
            AppErrorOptions {
                description: Some(err.to_string()),
                ..Default::default()
            },
        )
    })?;

    let svg_string = tree.to_string(&WriteOptions::default());
    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "image/svg+xml")
        .body(Body::from(svg_string.into_bytes()))
        .map_err(|err| {
            AppError::new_with_options(
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorCode::new("CONVERT_BODY"),
                AppErrorOptions {
                    description: Some(err.to_string()),
                    ..Default::default()
                },
            )
        })?;

    return Ok(response);
}
