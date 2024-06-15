pub mod _template_id;
pub mod render;

use crate::environment::app_state::AppState;
use axum::Router;

pub fn router() -> Router<AppState> {
    Router::new()
        .nest("/render", render::router())
        .nest("/:id", _template_id::router())
}
