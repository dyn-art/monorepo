pub mod health;
pub mod v1;

use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};

use crate::environment::app_state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(handler))
        .nest("/v1", v1::router())
        .nest("/health", health::router())
}

async fn handler() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "status": "Success",
            "message": format!(
                "dyn_graphic (v{}) is up and running!",
                env!("CARGO_PKG_VERSION")
            )
        })),
    )
}
