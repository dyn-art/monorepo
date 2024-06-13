pub mod _template_id;
pub mod render;

use axum::Router;

pub fn routes() -> Router {
    Router::new()
        .nest("/render", render::routes())
        .nest("/:id", _template_id::routes())
}
