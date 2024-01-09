use axum::{
    routing::{get, post},
    Router,
};

use self::handler::{health_checker_handler, render_composition};

mod handler;

pub fn app() -> Router {
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
        .route("/health", get(health_checker_handler))
        .route("/render", post(render_composition))
}
