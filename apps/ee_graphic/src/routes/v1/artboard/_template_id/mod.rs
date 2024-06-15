pub mod render;

use crate::environment::app_state::AppState;
use axum::Router;

pub fn router() -> Router<AppState> {
    Router::new()
}
