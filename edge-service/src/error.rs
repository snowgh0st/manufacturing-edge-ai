use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

/// Centralized error enum for the Manufacturing Edge App
#[derive(Debug)]
pub enum EdgeError {
    MqttConnectionFailed(String),
    InferenceExecutionError(String),
    DatabaseError(String),
    InternalServerError,
}

// Implement Axum IntoResponse for automatic HTTP error responses
impl IntoResponse for EdgeError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            EdgeError::MqttConnectionFailed(msg) => (StatusCode::SERVICE_UNAVAILABLE, msg),
            EdgeError::InferenceExecutionError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            EdgeError::DatabaseError(msg) => (StatusCode::BAD_GATEWAY, msg),
            EdgeError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Edge Server Error".to_string(),
            ),
        };

        let body = Json(json!({
            "status": "error",
            "message": error_message
        }));

        (status, body).into_response()
    }
}