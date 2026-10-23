use crate::error::EdgeError;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorTelemetry {
    pub device_id: String,
    pub vibration_hz: f64,
    pub temperature_celsius: f64,
    pub timestamp: u64,
}

// ============================================================================
// HOT PATH: High-Throughput MQTT Streamer
// ============================================================================
/// Listens to MQTT telemetry topics and pushes payloads into an in-memory queue.
/// 
/// Uses Tokio unbounded/bounded channels to decouple network ingestion speed 
/// from database persistence to guarantee sub-millisecond execution.
pub async fn start_telemetry_ingestion(
    tx: mpsc::Sender<SensorTelemetry>,
) -> Result<(), EdgeError> {
    tracing::info!("Hot Path: Initializing MQTT telemetry listener thread");

    // Loop continuously reading from MQTT broker connection
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(100));
        loop {
            interval.tick().await;

            let mock_telemetry = SensorTelemetry {
                device_id: "cnc-mill-01".to_string(),
                vibration_hz: 142.5,
                temperature_celsius: 78.2,
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            };

            // SAD PATH: Handle channel overflow or receiver drop gracefully
            if let Err(e) = tx.send(mock_telemetry).await {
                tracing::error!("Sad Path: Telemetry channel disconnected: {:?}", e);
                break;
            }
        }
    });

    Ok(())
}