use axum::Router;

pub mod render;

pub fn routes() -> Router {
    Router::new().nest("/render", render::routes())
}
