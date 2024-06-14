use crate::error::app_error::{AppError, AppErrorOptions, ErrorCode};
use axum::{
    extract::{rejection::JsonRejection, FromRequest, FromRequestParts, Request},
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Error, Json,
};
use serde::de::DeserializeOwned;

// Custom Query implementation to utilize 'serde_qs'
// and for custom error handling
// https://github.com/tokio-rs/axum/issues/434#issuecomment-954889152
#[derive(Debug, Clone, Copy, Default)]
pub struct AppQuery<T>(T);

impl<T> AppQuery<T> {
    pub fn get(self) -> T {
        self.0
    }
}

#[axum::async_trait]
impl<T, S> FromRequestParts<S> for AppQuery<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = AppQueryRejection;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let query = parts.uri.query().unwrap_or_default();
        let value = serde_qs::from_str(query)
            .map_err(|err| AppQueryRejection::FailedToDeserializeQueryString(Error::new(err)))?;
        Ok(AppQuery(value))
    }
}

#[derive(Debug)]
pub enum AppQueryRejection {
    FailedToDeserializeQueryString(Error),
}

impl IntoResponse for AppQueryRejection {
    fn into_response(self) -> Response {
        match self {
            Self::FailedToDeserializeQueryString(inner) => AppError::new_with_options(
                StatusCode::BAD_REQUEST,
                ErrorCode::new("INVALID_QUERY_STRING"),
                AppErrorOptions {
                    description: Some(format!("Failed to deserialize query string: {inner}")),
                    ..Default::default()
                },
            )
            .into_response(),
        }
    }
}

// Thins wrapper around Json for custom error handling
#[derive(Debug, Clone, Copy, Default)]
pub struct AppJson<T>(Json<T>);

impl<T> AppJson<T> {
    pub fn get(self) -> T {
        self.0 .0
    }
}

#[axum::async_trait]
impl<T, S> FromRequest<S> for AppJson<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = AppJsonRejection;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let json = Json::<T>::from_request(req, state)
            .await
            .map_err(|err| AppJsonRejection(err))?;
        Ok(AppJson(json))
    }
}

#[derive(Debug)]
pub struct AppJsonRejection(JsonRejection);

impl IntoResponse for AppJsonRejection {
    fn into_response(self) -> Response {
        AppError::new_with_options(
            StatusCode::BAD_REQUEST,
            ErrorCode::new("INVALID_BODY"),
            AppErrorOptions {
                description: Some(self.0.body_text()),
                ..Default::default()
            },
        )
        .into_response()
    }
}

// pub type AppJsonBody<G> = Result<Json<G>, JsonRejection>;

// pub fn extract_json_body<G>(app_json_body: AppJsonBody<G>) -> Result<G, AppError> {
//     match app_json_body {
//         Ok(Json(body)) => Ok(body),
//         Err(err) => Err(AppError::new_with_options(
//             StatusCode::BAD_REQUEST,
//             ErrorCode::new("INVALID_BODY"),
//             AppErrorOptions {
//                 description: Some(err.body_text()),
//                 ..Default::default()
//             },
//         )),
//     }
// }
