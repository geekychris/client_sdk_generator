//! A sample API that uses a pet store as an example
//! 
//! Generated SDK for Pet Store API v1.0.0

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
use crate::error::PetStoreApiError;
use crate::error::Result;
use crate::models::pet::Pet;
use crate::models::new_pet::Newpet;
use crate::models::category::Category;
use crate::models::error::Error;

/// A sample API that uses a pet store as an example
pub struct PetStoreApiClient {
    client: Client,
    base_url: Url,
    config: ClientConfig,
    telemetry: TelemetryHandler,
}

impl PetStoreApiClient {
    /// Create a new client instance
    pub fn new(config: ClientConfig) -> Result<Self> {
        let base_url = Url::parse(&config.base_url)
            .map_err(|e| PetStoreApiError::Configuration(format!("Invalid base URL: {}", e)))?;
        
        let client = Client::builder()
            .timeout(Duration::from_secs(config.timeout_seconds))
            .build()
            .map_err(|e| PetStoreApiError::Http(e))?;
        
        Ok(Self {
            client,
            base_url,
            config,
            telemetry: TelemetryHandler::new(),
        })
    }
    
    /// Returns a list of all pets in the store
    pub async fn list_pets(
        &self,
        limit: Option<String>,
        tag: Option<String>,
    ) -> Result<String> {
        
        
        let result = async {
            let mut url = self.build_url("/pets", &limit, &tag)?;
            
            // Add query parameters
            {
                let mut query_pairs = url.query_pairs_mut();
                if let Some(ref value) = limit {
                    query_pairs.append_pair("limit", &value.to_string());
                }
                if let Some(ref value) = tag {
                    query_pairs.append_pair("tag", &value.to_string());
                }
            }
            
            let mut request_builder = self.client.request(
                reqwest::Method::GET,
                url.as_str()
            );
            
            // Add headers
            if let Some(ref value) = limit {
                request_builder = request_builder.header("limit", value.to_string());
            }
            if let Some(ref value) = tag {
                request_builder = request_builder.header("tag", value.to_string());
            }
            
            // Add request body
            
            let response = request_builder.send().await
                .map_err(PetStoreApiError::Http)?;
            
            self.parse_response(response).await
        }.await;
        
        
        match result {
            Ok(parsed_result) => {
                Ok(parsed_result)
            }
            Err(e) => Err(e),
        }
    }
    
    /// Creates a new pet in the store
    pub async fn create_pet(
        &self,
    ) -> Result<()> {
        
        
        let result = async {
            let mut url = self.build_url("/pets")?;
            
            // Add query parameters
            {
                let mut query_pairs = url.query_pairs_mut();
            }
            
            let mut request_builder = self.client.request(
                reqwest::Method::POST,
                url.as_str()
            );
            
            // Add headers
            
            // Add request body
            
            let response = request_builder.send().await
                .map_err(PetStoreApiError::Http)?;
            
            self.parse_response(response).await
        }.await;
        
        
        match result {
            Ok(parsed_result) => {
                Ok(parsed_result)
            }
            Err(e) => Err(e),
        }
    }
    
    /// Returns a single pet by its ID
    pub async fn get_pet(
        &self,
        pet_id: String,
    ) -> Result<()> {
        
        
        let result = async {
            let mut url = self.build_url("/pets/{petId}", &pet_id)?;
            
            // Add query parameters
            {
                let mut query_pairs = url.query_pairs_mut();
                query_pairs.append_pair("petId", &pet_id.to_string());
            }
            
            let mut request_builder = self.client.request(
                reqwest::Method::GET,
                url.as_str()
            );
            
            // Add headers
            request_builder = request_builder.header("petId", pet_id.to_string());
            
            // Add request body
            
            let response = request_builder.send().await
                .map_err(PetStoreApiError::Http)?;
            
            self.parse_response(response).await
        }.await;
        
        
        match result {
            Ok(parsed_result) => {
                Ok(parsed_result)
            }
            Err(e) => Err(e),
        }
    }
    
    /// Updates an existing pet
    pub async fn update_pet(
        &self,
        pet_id: String,
    ) -> Result<()> {
        
        
        let result = async {
            let mut url = self.build_url("/pets/{petId}", &pet_id)?;
            
            // Add query parameters
            {
                let mut query_pairs = url.query_pairs_mut();
                query_pairs.append_pair("petId", &pet_id.to_string());
            }
            
            let mut request_builder = self.client.request(
                reqwest::Method::PUT,
                url.as_str()
            );
            
            // Add headers
            request_builder = request_builder.header("petId", pet_id.to_string());
            
            // Add request body
            
            let response = request_builder.send().await
                .map_err(PetStoreApiError::Http)?;
            
            self.parse_response(response).await
        }.await;
        
        
        match result {
            Ok(parsed_result) => {
                Ok(parsed_result)
            }
            Err(e) => Err(e),
        }
    }
    
    /// Deletes a pet from the store
    pub async fn delete_pet(
        &self,
        pet_id: String,
    ) -> Result<()> {
        
        
        let result = async {
            let mut url = self.build_url("/pets/{petId}", &pet_id)?;
            
            // Add query parameters
            {
                let mut query_pairs = url.query_pairs_mut();
                query_pairs.append_pair("petId", &pet_id.to_string());
            }
            
            let mut request_builder = self.client.request(
                reqwest::Method::DELETE,
                url.as_str()
            );
            
            // Add headers
            request_builder = request_builder.header("petId", pet_id.to_string());
            
            // Add request body
            
            let response = request_builder.send().await
                .map_err(PetStoreApiError::Http)?;
            
            self.parse_response(response).await
        }.await;
        
        
        match result {
            Ok(parsed_result) => {
                Ok(parsed_result)
            }
            Err(e) => Err(e),
        }
    }
    
    
    fn build_url(&self, path: &str, limit: &String, tag: &String, pet_id: &String, pet_id: &String, pet_id: &String) -> Result<Url> {
        let mut url = self.base_url.join(path.trim_start_matches('/'))
            .map_err(|e| PetStoreApiError::Url(e))?;
        
        // Replace path parameters
        let path_str = url.path().to_string();
        let path_str = path_str.replace(&format!("limit"), &limit.to_string());
        let path_str = path_str.replace(&format!("tag"), &tag.to_string());
        let path_str = path_str.replace(&format!("petId"), &pet_id.to_string());
        let path_str = path_str.replace(&format!("petId"), &pet_id.to_string());
        let path_str = path_str.replace(&format!("petId"), &pet_id.to_string());
        
        url.set_path(&path_str);
        Ok(url)
    }
    
    async fn parse_response<T>(&self, response: Response) -> Result<T> 
    where
        T: for<'de> Deserialize<'de>,
    {
        let status = response.status();
        
        if status.is_success() {
            if status == reqwest::StatusCode::NO_CONTENT {
                // For void responses, we need to handle this case
                // This is a workaround for the type system
                return Err(PetStoreApiError::Parsing("No content to parse".to_string()));
            }
            
            let text = response.text().await
                .map_err(PetStoreApiError::Http)?;
            
            serde_json::from_str(&text)
                .map_err(|e| PetStoreApiError::Parsing(format!("Failed to parse JSON: {}", e)))
        } else {
            let error_text = response.text().await
                .unwrap_or_else(|_| format!("HTTP {}", status));
            Err(PetStoreApiError::Api(status.as_u16(), error_text))
        }
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

impl Default for PetStoreApiClient {
    fn default() -> Self {
        let config = ClientConfig::default();
        Self::new(config).expect("Failed to create default client")
    }
}