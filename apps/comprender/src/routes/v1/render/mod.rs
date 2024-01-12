use axum::{routing::post, Router};

use self::controller::render_composition;

mod controller;

pub fn routes() -> Router {
    Router::new().route("/", post(render_composition))
}
