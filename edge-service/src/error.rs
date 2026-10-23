use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use std::fmt;

/// Centralized error enum for the Manufacturing Edge App
#[derive(Debug)]
pub enum EdgeError {
    #[allow(dead_code)]
    MqttConnectionFailed(String),
    InferenceExecutionError(String),
    #[allow(dead_code)]
    DatabaseError(String),
    #[allow(dead_code)]
    InternalServerError,
}

impl fmt::Display for EdgeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EdgeError::MqttConnectionFailed(msg) => write!(f, "MQTT Connection Failed: {}", msg),
            EdgeError::InferenceExecutionError(msg) => write!(f, "Inference Engine Error: {}", msg),
            EdgeError::DatabaseError(msg) => write!(f, "Database Error: {}", msg),
            EdgeError::InternalServerError => write!(f, "Internal Server Error"),
        }
    }
}

impl std::error::Error for EdgeError {}

impl IntoResponse for EdgeError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            EdgeError::MqttConnectionFailed(msg) => (StatusCode::SERVICE_UNAVAILABLE, msg.clone()),
            EdgeError::InferenceExecutionError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
            EdgeError::DatabaseError(msg) => (StatusCode::BAD_GATEWAY, msg.clone()),
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