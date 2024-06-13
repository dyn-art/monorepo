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
    Running,
}

#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Success", body = HealthDto),
    ),
)]
async fn handler(State(app_state): State<AppState>) -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(HealthDto {
            status: HealthStatus::Running,
            message: format!(
                "Server at version v{} is up and running!",
                app_state.config.pkg_version
            ),
        }),
    )
}
