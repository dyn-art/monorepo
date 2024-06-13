use axum::{http::StatusCode, response::IntoResponse, Json};
use axum::{routing::get, Router};
use serde::Serialize;

pub fn routes() -> Router {
    Router::new().route("/", get(handler))
}

#[derive(Serialize, utoipa::ToSchema)]
pub struct HealthDto {
    status: HealthStatus,
    message: String,
}

#[derive(Serialize, utoipa::ToSchema)]
pub enum HealthStatus {
    Running,
}

#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Success", body = HealthDto),
    ),
)]
async fn handler() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(HealthDto {
            status: HealthStatus::Running,
            message: "Server is up and running!".to_string(),
        }),
    )
}
