pub mod cnv;
pub mod svg;

use axum::Router;

use crate::environment::app_state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .nest("/cnv", cnv::router())
        .nest("/svg", svg::router())
}
