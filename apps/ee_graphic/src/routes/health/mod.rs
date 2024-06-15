use crate::environment::app_state::AppState;
use axum::extract::State;
use axum::{http::StatusCode, response::IntoResponse, Json};
use axum::{routing::get, Router};
use serde::Serialize;

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(handler))
}

#[derive(Serialize, utoipa::ToSchema)]
pub struct HealthDto {
    status: HealthStatus,
    message: String,
}

#[derive(Serialize, utoipa::ToSchema)]
pub enum HealthStatus {
    Up,
    // Down,
}

#[utoipa::path(
    get,
    path = "/health",
    operation_id = "get_health_handler",
    responses(
        (status = 200, description = "Server is up and running", body = HealthDto),
    ),
)]
async fn handler(State(app_state): State<AppState>) -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(HealthDto {
            status: HealthStatus::Up,
            message: format!(
                "Server at version v{} is up and running!",
                app_state.config.pkg_version
            ),
        }),
    )
}
