use axum::Router;

mod render;

pub fn routes() -> Router {
    Router::new().nest("/render", render::routes())
}
