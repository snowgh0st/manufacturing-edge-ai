mod config;
mod error;
mod inference;
mod storage;
mod telemetry;

use axum::{
    extract::ws::{WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use config::AppConfig;
use error::EdgeError;
use inference::EdgeInferenceEngine;
use serde_json::json;
use std::sync::Arc;
use tokio::sync::mpsc;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Shared Application State available across all Axum HTTP/WS handlers
pub struct AppState {
    pub inference_engine: EdgeInferenceEngine,
    pub config: AppConfig,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging output
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new("info"))
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Initializing Manufacturing Edge Vision Service");

    let config = AppConfig::from_env();
    let inference_engine = EdgeInferenceEngine::new("yolov8_manufacturing_v1.onnx");

    let shared_state = Arc::new(AppState {
        inference_engine,
        config: config.clone(),
    });

    // ========================================================================
    // HOT PATH decoupling channel creation
    // ========================================================================
    let (telemetry_tx, mut telemetry_rx) = mpsc::channel::<telemetry::SensorTelemetry>(1000);

    // Start background MQTT listener task
    telemetry::start_telemetry_ingestion(telemetry_tx).await?;

    // Background task consuming telemetry from Tokio channel
    tokio::spawn(async move {
        while let Some(data) = telemetry_rx.recv().await {
            // Process or persist high-frequency telemetry
            tracing::debug!("Hot Path Channel Processing: Device {}", data.device_id);
        }
    });

    // Configure CORS for Vue.js frontend
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build Axum HTTP and WebSocket router
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/v1/inspect", post(trigger_vision_inspection))
        .route("/ws/telemetry", get(websocket_telemetry_handler))
        .layer(cors)
        .with_state(shared_state);

    let addr = format!("{}:{}", config.server_host, config.server_port);
    tracing::info!("Server executing on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

/// Simple health check route
async fn health_check() -> impl IntoResponse {
    Json(json!({ "status": "healthy", "service": "edge-vision-pipeline" }))
}

/// Trigger on-demand vision frame inspection
async fn trigger_vision_inspection(
    axum::extract::State(state): axum::extract::State<Arc<AppState>>,
    body: axum::body::Bytes,
) -> Result<impl IntoResponse, EdgeError> {
    // EXECUTION PATH: Run inspection on frame payload
    let result = state
        .inference_engine
        .run_vision_inference(&body)
        .await
        .map_err(EdgeError::InferenceExecutionError)?;

    Ok(Json(json!({
        "status": "success",
        "detection": result
    })))
}

/// Upgrades HTTP GET to a WebSocket connection for real-time telemetry streaming
async fn websocket_telemetry_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    tracing::info!("New WebSocket connection established with operator dashboard");
    let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(500));

    loop {
        interval.tick().await;

        // Stream real-time status frames to Vue frontend
        let mock_update = json!({
            "event": "sensor_update",
            "vibration": 142.8,
            "temperature": 76.4,
            "bearing_status": "NORMAL"
        });

        // SAD PATH: Handle client disconnection gracefully
        if socket
            .send(axum::extract::ws::Message::Text(mock_update.to_string()))
            .await
            .is_err()
        {
            tracing::warn!("WebSocket client disconnected");
            break;
        }
    }
}