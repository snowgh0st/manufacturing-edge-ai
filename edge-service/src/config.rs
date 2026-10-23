use std::env;

/// Application configuration structure
#[derive(Debug, Clone)]
pub struct AppConfig {
    pub server_host: String,
    pub server_port: u16,
    pub mqtt_broker_host: String,
    pub mqtt_broker_port: u16,
    pub mongodb_uri: String,
}

impl AppConfig {
    /// Loads configuration from environment variables with safe default fallbacks
    pub fn from_env() -> Self {
        Self {
            server_host: env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .unwrap_or(3000),
            mqtt_broker_host: env::var("MQTT_HOST").unwrap_or_else(|_| "localhost".to_string()),
            mqtt_broker_port: env::var("MQTT_PORT")
                .unwrap_or_else(|_| "1883".to_string())
                .parse()
                .unwrap_or(1883),
            mongodb_uri: env::var("MONGODB_URI")
                .unwrap_or_else(|_| "mongodb://localhost:27017".to_string()),
        }
    }
}