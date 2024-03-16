use axum::{routing::post, Router};

// use self::controller::{render_composition, simplify_svg};

pub mod controller;

pub fn routes() -> Router {
    Router::new()
    // .route("/", post(render_composition))
    // .route("/svg", post(simplify_svg))
}
