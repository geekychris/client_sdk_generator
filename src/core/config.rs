// Copyright (c) 2024 Chris Collins <chris@hitorro.com>
// Licensed under the MIT License

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratorConfig {
    pub input_type: InputType,
    pub target_language: TargetLanguage,
    pub features: FeatureConfig,
    pub output_config: OutputConfig,
    pub authentication: Option<AuthenticationConfig>,
    pub template_overrides: Option<PathBuf>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum InputType {
    OpenApi,
    GraphQL,
    Grpc,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TargetLanguage {
    Java,
    Python,
    Rust,
    Go,
    TypeScript,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureConfig {
    pub retry: RetryConfig,
    pub telemetry: TelemetryConfig,
    pub caching: CachingConfig,
    pub async_support: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
    pub enabled: bool,
    pub max_attempts: u32,
    pub backoff_strategy: BackoffStrategy,
    pub retry_on: Vec<String>, // HTTP status codes or error types
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BackoffStrategy {
    Fixed { delay_ms: u64 },
    Exponential { initial_delay_ms: u64, multiplier: f64, max_delay_ms: u64 },
    Linear { initial_delay_ms: u64, increment_ms: u64 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryConfig {
    pub enabled: bool,
    pub provider: TelemetryProvider,
    pub metrics: MetricsConfig,
    pub tracing: TracingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TelemetryProvider {
    Prometheus,
    OpenTelemetry,
    Custom { implementation: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsConfig {
    pub enabled: bool,
    pub include_request_duration: bool,
    pub include_request_count: bool,
    pub include_error_count: bool,
    pub custom_labels: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TracingConfig {
    pub enabled: bool,
    pub include_request_body: bool,
    pub include_response_body: bool,
    pub sample_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachingConfig {
    pub enabled: bool,
    pub default_ttl_seconds: u64,
    pub max_cache_size: usize,
    pub cache_key_strategy: CacheKeyStrategy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CacheKeyStrategy {
    MethodAndParams,
    CustomHash,
    UserDefined,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputConfig {
    pub package_name: Option<String>,
    pub version: String,
    pub author: Option<String>,
    pub license: Option<String>,
    pub include_tests: bool,
    pub include_docs: bool,
    pub format_code: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationConfig {
    pub auth_type: AuthenticationType,
    pub location: AuthLocation,
    pub parameter_name: Option<String>,
    pub scheme: Option<String>,
    pub bearer_format: Option<String>,
    pub flows: Option<Vec<OAuthFlow>>,
    pub openid_connect_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthenticationType {
    None,
    ApiKey,
    Http,
    OAuth2,
    OpenIdConnect,
    Custom { name: String, description: Option<String> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthLocation {
    Query,
    Header,
    Cookie,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthFlow {
    pub flow_type: OAuthFlowType,
    pub authorization_url: Option<String>,
    pub token_url: Option<String>,
    pub refresh_url: Option<String>,
    pub scopes: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OAuthFlowType {
    AuthorizationCode,
    Implicit,
    ResourceOwnerPassword,
    ClientCredentials,
}

impl GeneratorConfig {
    pub fn from_file(path: PathBuf) -> Result<Self> {
        let content = std::fs::read_to_string(&path)
            .context(format!("Failed to read config file: {:?}", path))?;
        
        if path.extension().and_then(|s| s.to_str()) == Some("yaml") || 
           path.extension().and_then(|s| s.to_str()) == Some("yml") {
            serde_yaml::from_str(&content)
                .context("Failed to parse YAML config")
        } else {
            serde_json::from_str(&content)
                .context("Failed to parse JSON config")
        }
    }
    
    pub fn default_for(input_type: InputType, target_language: TargetLanguage) -> Self {
        Self {
            input_type,
            target_language,
            features: FeatureConfig::default(),
            output_config: OutputConfig::default_for(target_language),
            authentication: None,
            template_overrides: None,
        }
    }
}

impl Default for FeatureConfig {
    fn default() -> Self {
        Self {
            retry: RetryConfig::default(),
            telemetry: TelemetryConfig::default(),
            caching: CachingConfig::default(),
            async_support: true,
        }
    }
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_attempts: 3,
            backoff_strategy: BackoffStrategy::Exponential {
                initial_delay_ms: 100,
                multiplier: 2.0,
                max_delay_ms: 5000,
            },
            retry_on: vec!["5xx".to_string(), "408".to_string(), "429".to_string()],
        }
    }
}

impl Default for TelemetryConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            provider: TelemetryProvider::Prometheus,
            metrics: MetricsConfig::default(),
            tracing: TracingConfig::default(),
        }
    }
}

impl Default for MetricsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            include_request_duration: true,
            include_request_count: true,
            include_error_count: true,
            custom_labels: HashMap::new(),
        }
    }
}

impl Default for TracingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            include_request_body: false,
            include_response_body: false,
            sample_rate: 1.0,
        }
    }
}

impl Default for CachingConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            default_ttl_seconds: 300,
            max_cache_size: 1000,
            cache_key_strategy: CacheKeyStrategy::MethodAndParams,
        }
    }
}

impl OutputConfig {
    fn default_for(language: TargetLanguage) -> Self {
        Self {
            package_name: None,
            version: "1.0.0".to_string(),
            author: None,
            license: Some("MIT".to_string()),
            include_tests: true,
            include_docs: true,
            format_code: true,
        }
    }
}

impl FromStr for TargetLanguage {
    type Err = anyhow::Error;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "java" => Ok(TargetLanguage::Java),
            "python" | "py" => Ok(TargetLanguage::Python),
            "rust" | "rs" => Ok(TargetLanguage::Rust),
            "go" | "golang" => Ok(TargetLanguage::Go),
            "typescript" | "ts" => Ok(TargetLanguage::TypeScript),
            _ => Err(anyhow::anyhow!("Unsupported target language: {}", s)),
        }
    }
}

impl std::fmt::Display for TargetLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TargetLanguage::Java => write!(f, "Java"),
            TargetLanguage::Python => write!(f, "Python"),
            TargetLanguage::Rust => write!(f, "Rust"),
            TargetLanguage::Go => write!(f, "Go"),
            TargetLanguage::TypeScript => write!(f, "TypeScript"),
        }
    }
}

impl std::fmt::Display for InputType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InputType::OpenApi => write!(f, "OpenAPI"),
            InputType::GraphQL => write!(f, "GraphQL"),
            InputType::Grpc => write!(f, "gRPC"),
        }
    }
}