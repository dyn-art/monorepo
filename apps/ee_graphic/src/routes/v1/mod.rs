pub mod artboard;
pub mod svg;

use axum::Router;

use crate::environment::app_state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .nest("/artboard", artboard::router())
        .nest("/svg", svg::router())
}
