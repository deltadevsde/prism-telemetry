use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Basic auth configuration
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct BasicAuth {
    pub enabled: bool,
    pub username: String,
    pub password: String,
}

// Configuration for metrics
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct MetricsConfig {
    pub enabled: bool,
    pub endpoint: String,
    pub auth: BasicAuth,
}

// Configuration for logs
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct LogsConfig {
    pub enabled: bool,
    pub endpoint: String,
    pub auth: BasicAuth,
}

// Configuration for traces
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct TracesConfig {
    pub enabled: bool,
    pub endpoint: String,
    pub auth: BasicAuth,
}

// Configuration for profiles
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ProfilesConfig {
    pub enabled: bool,
    pub endpoint: String,
    pub auth: BasicAuth,
}

// Global labels to be added to all telemetry types
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct TelemetryLabels {
    pub labels: HashMap<String, String>,
}

// Configuration for telemetry components
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct TelemetryConfig {
    pub metrics: MetricsConfig,
    pub logs: LogsConfig,
    pub traces: TracesConfig,
    pub profiles: ProfilesConfig,
    pub global_labels: TelemetryLabels,
}
