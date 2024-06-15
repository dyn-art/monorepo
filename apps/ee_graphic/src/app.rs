use crate::{
    environment::{app_config::AppConfig, app_state::AppState},
    error::fallback::fallback_handler,
    routes,
};
use axum::Router;

pub fn setup(app_config: AppConfig) -> Router {
    let app_state = AppState::new(app_config);
    Router::<AppState>::new()
        .fallback(fallback_handler)
        .nest("/", routes::router())
        .with_state(app_state)
}
