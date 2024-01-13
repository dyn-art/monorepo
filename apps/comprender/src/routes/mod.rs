use axum::{routing::get, Router};

use self::controller::health_checker_handler;

pub mod controller;
pub mod v1;

pub fn routes() -> Router {
    Router::new()
        .route(
            "/",
            get(|| async {
                format!(
                    "dyn_comprender (v{}) is up and running!",
                    env!("CARGO_PKG_VERSION")
                )
            }),
        )
        .nest("/v1", v1::routes())
        .route("/health", get(health_checker_handler))
}
