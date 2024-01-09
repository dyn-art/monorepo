use axum::{routing::get, Router};

use self::handler::health_checker_handler;

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
    // .route(
    //     "images",
    //     post(|req, next| async move {
    //         // TODO
    //         return "";
    //     }),
    // ) // TODO:
}
