use crate::error::app_error::{AppError, ErrorCode};
use axum::{
    extract::{
        rejection::{JsonRejection, QueryRejection},
        Query,
    },
    http::StatusCode,
    Json,
};

use super::app_error::AppErrorOptions;

pub type AppQuery<G> = Result<Query<G>, QueryRejection>;

pub fn extract_query_params<G>(app_query: AppQuery<G>) -> Result<G, AppError> {
    match app_query {
        Ok(Query(params)) => Ok(params),
        Err(err) => Err(AppError::new_with_options(
            StatusCode::BAD_REQUEST,
            ErrorCode::new("INVALID_QUERY_PARAMS"),
            AppErrorOptions {
                description: Some(err.body_text()),
                ..Default::default()
            },
        )),
    }
}

pub type AppJson<G> = Result<Json<G>, JsonRejection>;

pub fn extract_json_body<G>(app_body: AppJson<G>) -> Result<G, AppError> {
    match app_body {
        Ok(Json(body)) => Ok(body),
        Err(err) => Err(AppError::new_with_options(
            StatusCode::BAD_REQUEST,
            ErrorCode::new("INVALID_BODY"),
            AppErrorOptions {
                description: Some(err.body_text()),
                ..Default::default()
            },
        )),
    }
}
