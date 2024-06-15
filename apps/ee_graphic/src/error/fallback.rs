use super::app_error::{AppError, AppErrorOptions, ErrorCode};
use axum::{
    http::{StatusCode, Uri},
    response::IntoResponse,
};

pub async fn fallback_handler(uri: Uri) -> impl IntoResponse {
    AppError::new_with_options(
        StatusCode::NOT_FOUND,
        ErrorCode::new("NOT_FOUND"),
        AppErrorOptions {
            description: Some(format!("No route found at: {uri}")),
            ..Default::default()
        },
    )
}
