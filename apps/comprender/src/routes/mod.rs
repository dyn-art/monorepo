pub mod health;
pub mod v1;

use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(handler))
        .nest("/v1", v1::routes())
        .nest("/health", health::routes())
}

async fn handler() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "status": "Success",
            "message": format!(
                "dyn_comprender (v{}) is up and running!",
                env!("CARGO_PKG_VERSION")
            )
        })),
    )
}
