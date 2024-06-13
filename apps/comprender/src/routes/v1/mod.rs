pub mod comp;
pub mod svg;

use axum::Router;

pub fn routes() -> Router {
    Router::new()
        .nest("/comp", comp::routes())
        .nest("/svg", svg::routes())
}
