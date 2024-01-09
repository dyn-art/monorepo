use axum::{
    routing::{get, post},
    Router,
};

use self::handler::{health_checker_handler, render_composition};

mod handler;
mod model;

// TODO:
// 1. Integrate https://github.com/juhaku/utoipa/blob/master/examples/todo-axum/src/main.rs
//    - Generate OpenAPI types from it
// 2. Create proper error model and error handling
// 3. Add support for generate composition by id (fetch from bucket here)
// 4. Load fonts
// 5. Create updated struct e.g. with font url instead of content itself in Rust
//    - 6. Or do it on Express site?

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
