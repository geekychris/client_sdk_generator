//! Generated from GraphQL schema
//! 
//! Generated GraphQL SDK for GraphQL API v1.0.0

use lazy_static::lazy_static;
use prometheus::{Counter, Histogram, Registry, Opts, HistogramOpts};
use std::time::Duration;
use std::time::Instant;
use tokio_retry::{strategy::*, Retry};

use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use url::Url;
use crate::telemetry::TelemetryHandler;

use crate::config::ClientConfig;
use crate::error::{ApiError, Result};
use crate::models::graph_qlrequest::Graphqlrequest;
use crate::models::graph_qlresponse::Graphqlresponse;

/// GraphQL Request structure
#[derive(Debug, Serialize)]
pub struct GraphQLRequest {
    pub query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<String>,
}

/// GraphQL Response structure
#[derive(Debug, Deserialize)]
pub struct GraphQLResponse<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<GraphQLError>>,
}

/// GraphQL Error structure
#[derive(Debug, Deserialize)]
pub struct GraphQLError {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locations: Option<Vec<GraphQLErrorLocation>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<Vec<serde_json::Value>>,
}

/// GraphQL Error Location
#[derive(Debug, Deserialize)]
pub struct GraphQLErrorLocation {
    pub line: u32,
    pub column: u32,
}

/// Generated from GraphQL schema
pub struct GraphqlApiClient {
    client: Client,
    base_url: Url,
    config: ClientConfig,
    telemetry: TelemetryHandler,
}

impl GraphqlApiClient {
    /// Create a new GraphQL client instance
    pub fn new(config: ClientConfig) -> Result<Self> {
        let base_url = Url::parse(&config.base_url)
            .map_err(|e| ApiError::Configuration(format!("Invalid base URL: {}", e)))?;
        
        let client = Client::builder()
            .timeout(Duration::from_secs(config.timeout_seconds))
            .build()
            .map_err(|e| ApiError::Http(e))?;
        
        Ok(Self {
            client,
            base_url,
            config,
            telemetry: TelemetryHandler::new(),
        })
    }
    
    /// Execute a GraphQL query
    pub async fn query<T>(&self, query: &str, variables: Option<serde_json::Value>) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let request = GraphQLRequest {
            query: query.to_string(),
            variables,
            operation_name: None,
        };
        
        self.execute_request(request).await
    }
    
    /// Execute a GraphQL mutation
    pub async fn mutation<T>(&self, mutation: &str, variables: Option<serde_json::Value>) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let request = GraphQLRequest {
            query: mutation.to_string(),
            variables,
            operation_name: None,
        };
        
        self.execute_request(request).await
    }
    
    /// Execute a raw GraphQL request
    pub async fn execute_request<T>(&self, request: GraphQLRequest) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        
        let start_time = std::time::Instant::now();
        
        let result = async {
            let mut request_builder = self.client
                .post(self.base_url.as_str())
                .json(&request)
                .header("Content-Type", "application/json")
                .header("Accept", "application/json");
            
            // Add authentication headers if configured
            if let Some(ref auth_header) = self.config.auth_header {
                if let Some(ref auth_value) = self.config.auth_value {
                    request_builder = request_builder.header(auth_header, auth_value);
                }
            }
            
            let response = request_builder.send().await
                .map_err(ApiError::Http)?;
            
            self.parse_graphql_response(response).await
        }.await;
        
        let duration = start_time.elapsed();
        self.telemetry.record_request(
            "POST", 
            self.base_url.path(), 
            duration.as_secs_f64(), 
            result.is_ok()
        );
        
        match result {
            Ok(parsed_result) => {
                Ok(parsed_result)
            }
            Err(e) => Err(e),
        }
    }
    
    async fn parse_graphql_response<T>(&self, response: Response) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let status = response.status();
        
        if !status.is_success() {
            let error_text = response.text().await
                .unwrap_or_else(|_| format!("HTTP {}", status));
            return Err(ApiError::Api(status.as_u16(), error_text));
        }
        
        let text = response.text().await
            .map_err(ApiError::Http)?;
        
        let graphql_response: GraphQLResponse<T> = serde_json::from_str(&text)
            .map_err(|e| ApiError::Parsing(format!("Failed to parse JSON: {}", e)))?;
        
        if let Some(errors) = graphql_response.errors {
            let error_messages: Vec<String> = errors.iter()
                .map(|err| err.message.clone())
                .collect();
            return Err(ApiError::GraphQL(error_messages.join("; ")));
        }
        
        graphql_response.data
            .ok_or_else(|| ApiError::Parsing("No data in GraphQL response".to_string()))
    }
    
    
use tokio_retry::{strategy::*, Retry};

pub async fn execute_with_retry<F, T, E>(operation: F) -> Result<T, E>
where
    F: Fn() -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<T, E>> + Send>> + Send,
    E: std::fmt::Debug,
{
    let retry_strategy = ExponentialBackoff::from_millis(100).factor(2).max_delay(Duration::from_millis(5000)).take(3);
    
    Retry::spawn(retry_strategy, || async {
        operation().await
    }).await
}



lazy_static! {
    static ref REQUEST_DURATION: Histogram = Histogram::with_opts(
        HistogramOpts::new("http_request_duration_seconds", "HTTP request duration")
    ).unwrap();
    
    static ref REQUEST_COUNT: Counter = Counter::with_opts(
        Opts::new("http_requests_total", "Total HTTP requests")
    ).unwrap();
}

pub struct TelemetryHandler {
    registry: Registry,
}

impl TelemetryHandler {
    pub fn new() -> Self {
        let registry = Registry::new();
        registry.register(Box::new(REQUEST_DURATION.clone())).unwrap();
        registry.register(Box::new(REQUEST_COUNT.clone())).unwrap();
        
        Self { registry }
    }
    
    pub fn record_request(&self, method: &str, path: &str, duration: f64) {
        REQUEST_DURATION.observe(duration);
        REQUEST_COUNT.inc();
    }
}

}

impl Default for GraphqlApiClient {
    fn default() -> Self {
        let config = ClientConfig::default();
        Self::new(config).expect("Failed to create default GraphQL client")
    }
}