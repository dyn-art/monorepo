use serde::Serialize;
use serde_json::json;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppError {
    /// HTTP status code associated with the error, indicating the nature of the failure.
    pub status: u16,

    /// A short, unique error code for identifying the error type.
    pub code: ErrorCode,

    /// A detailed, human-readable description of the error. Provides additional context
    /// and, if applicable, steps to resolve the issue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// An optional URI linking to a document or resource with more information about the error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,

    /// An array of additional error details or nested errors that occurred during the process.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub additional_errors: Vec<HashMap<String, serde_json::Value>>,
}

impl AppError {
    pub fn new(status: vercel_runtime::StatusCode, code: ErrorCode) -> Self {
        AppError {
            status: status.as_u16(),
            code,
            description: None,
            uri: None,
            additional_errors: Vec::new(),
        }
    }

    pub fn new_with_options(
        status: vercel_runtime::StatusCode,
        code: ErrorCode,
        options: AppErrorOptions,
    ) -> Self {
        AppError {
            status: status.as_u16(),
            code,
            description: options.description,
            uri: options.uri,
            additional_errors: options.additional_errors.unwrap_or_else(Vec::new),
        }
    }
}

pub trait IntoVercelResponse {
    fn into_vercel_response(
        self,
    ) -> Result<vercel_runtime::Response<vercel_runtime::Body>, vercel_runtime::Error>;
}

impl IntoVercelResponse for AppError {
    fn into_vercel_response(
        self,
    ) -> Result<vercel_runtime::Response<vercel_runtime::Body>, vercel_runtime::Error> {
        let status = vercel_runtime::StatusCode::from_u16(self.status)
            .unwrap_or(vercel_runtime::StatusCode::INTERNAL_SERVER_ERROR);

        Ok(vercel_runtime::Response::builder()
            .status(status)
            .header("Content-Type", "application/json")
            .body(json!(self).to_string().into())?)
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::new_with_options(
            vercel_runtime::StatusCode::INTERNAL_SERVER_ERROR,
            ErrorCode::new("INTERNAL_SERVER_ERROR"),
            AppErrorOptions {
                description: Some(err.to_string()),
                uri: None,
                additional_errors: None,
            },
        )
    }
}

impl From<vercel_runtime::Error> for AppError {
    fn from(err: vercel_runtime::Error) -> Self {
        AppError::new_with_options(
            vercel_runtime::StatusCode::INTERNAL_SERVER_ERROR,
            ErrorCode::new("LAMBDA_RUNTIME_ERROR"),
            AppErrorOptions {
                description: Some(err.to_string()),
                uri: None,
                additional_errors: None,
            },
        )
    }
}

#[derive(Debug, Serialize)]
pub struct AppErrorOptions {
    pub uri: Option<String>,
    pub description: Option<String>,
    pub additional_errors: Option<Vec<HashMap<String, serde_json::Value>>>,
}

#[derive(Debug, Serialize)]
pub struct ErrorCode(String);

impl ErrorCode {
    pub fn new(code: &str) -> Self {
        ErrorCode(format!("#ERR_{}", code))
    }
}

#[macro_export]
macro_rules! app_error {
    // Basic usage: Just status code and error code.
    ($status:expr, $code:expr) => {
        AppError::new($status, $code)
    };

    // With status code, error code, and description.
    ($status:expr, $code:expr, $description:expr) => {{
        AppError::new_with_options(
            $status,
            $code,
            dyn_web_api::models::app_error::AppErrorOptions {
                description: Some($description.into()),
                uri: None,
                additional_errors: None,
            },
        )
    }};

    // With status code, error code, description, and uri.
    ($status:expr, $code:expr, $description:expr, $uri:expr) => {{
        AppError::new_with_options(
            $status,
            $code,
            dyn_web_api::models::app_error::AppErrorOptions {
                description: Some($description.into()),
                uri: Some($uri.into()),
                additional_errors: None,
            },
        )
    }};

    // Full options: status code, error code, description, uri, and additional_errors.
    ($status:expr, $code:expr, $description:expr, $uri:expr, $additional_errors:expr) => {{
        AppError::new_with_options(
            $status,
            $code,
            dyn_web_api::models::app_error::AppErrorOptions {
                description: Some($description.into()),
                uri: Some($uri.into()),
                additional_errors: Some($additional_errors),
            },
        )
    }};
}
