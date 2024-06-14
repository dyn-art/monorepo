pub mod arb;
pub mod svg;

use axum::Router;

use crate::environment::app_state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .nest("/arb", arb::router())
        .nest("/svg", svg::router())
}
