pub mod comp;
pub mod svg;

use axum::Router;

use crate::environment::app_state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .nest("/comp", comp::router())
        .nest("/svg", svg::router())
}
