pub mod convert;
pub mod simplify;

use crate::environment::app_state::AppState;
use axum::Router;

pub fn router() -> Router<AppState> {
    Router::new()
        .nest("/convert", convert::router())
        .nest("/simplify", simplify::router())
}
