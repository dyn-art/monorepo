pub mod convert;
pub mod simplify;

use axum::Router;

pub fn routes() -> Router {
    Router::new()
        .nest("/convert", convert::routes())
        .nest("/simplify", simplify::routes())
}
