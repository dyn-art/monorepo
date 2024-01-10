use axum::{response::IntoResponse, Json};

pub async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Server is up and running!";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}
