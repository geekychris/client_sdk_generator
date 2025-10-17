use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Configuration for the PetStoreApi client
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientConfig {
    /// Base URL for the API
    pub base_url: String,
    
    /// Request timeout in seconds
    pub timeout_seconds: u64,
    
    /// API key for authentication (if required)
    pub api_key: Option<String>,
    
    /// Custom headers to include with requests
    pub default_headers: std::collections::HashMap<String, String>,
    
    /// Retry configuration
    pub retry_config: RetryConfig,
    
    
    /// Telemetry configuration
    pub telemetry_config: TelemetryConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
    pub max_attempts: u32,
    pub initial_delay_ms: u64,
    pub max_delay_ms: u64,
    pub backoff_multiplier: f64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay_ms: 100,
            max_delay_ms: 5000,
            backoff_multiplier: 2.0,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryConfig {
    pub enabled: bool,
    pub endpoint: Option<String>,
    pub service_name: String,
}

impl Default for TelemetryConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            endpoint: None,
            service_name: "pet--store--api-client".to_string(),
        }
    }
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            base_url: "http://localhost:8080".to_string(),
            timeout_seconds: 30,
            api_key: None,
            default_headers: std::collections::HashMap::new(),
            retry_config: RetryConfig::default(),
            telemetry_config: TelemetryConfig::default(),
        }
    }
}

impl ClientConfig {
    /// Create a new config with the specified base URL
    pub fn new<S: Into<String>>(base_url: S) -> Self {
        Self {
            base_url: base_url.into(),
            ..Default::default()
        }
    }
    
    /// Set the request timeout
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout_seconds = timeout.as_secs();
        self
    }
    
    /// Set the API key for authentication
    pub fn with_api_key<S: Into<String>>(mut self, api_key: S) -> Self {
        self.api_key = Some(api_key.into());
        self
    }
    
    /// Add a default header
    pub fn with_header<K, V>(mut self, key: K, value: V) -> Self 
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.default_headers.insert(key.into(), value.into());
        self
    }
}