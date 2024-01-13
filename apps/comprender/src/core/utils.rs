use axum::{extract::Query, http::StatusCode, Json};

use crate::models::app_error::{AppError, ErrorCode};

pub fn extract_query_params<G>(query: Option<Query<G>>) -> Result<G, AppError> {
    match query {
        Some(Query(params)) => Ok(params),
        None => Err(AppError::new(
            StatusCode::BAD_REQUEST,
            ErrorCode::new("INVALID_QUERY_PARAMS"),
        )),
    }
}

pub fn extract_json_body<G>(maybe_body: Option<Json<G>>) -> Result<G, AppError> {
    match maybe_body {
        Some(Json(body)) => Ok(body),
        None => Err(AppError::new(
            StatusCode::BAD_REQUEST,
            ErrorCode::new("INVALID_BODY"),
        )),
    }
}
