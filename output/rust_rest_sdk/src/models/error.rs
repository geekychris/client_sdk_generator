//! 
//! 

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, NaiveDate, Utc};

/// 
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Error {
    /// Error code
    #[serde(rename = "code")]
    pub code: String,
    
    /// Error message
    #[serde(rename = "message")]
    pub message: String,
    
    /// Additional error details
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "details")]
    pub details: Option<String>,
    
}

impl Error {
    /// Create a new instance with required fields
    pub fn new(code: String, message: String, ) -> Self {
        Self {
            code,
            message,
            details: None,
        }
    }
    
    
    
    /// Set Additional error details
    pub fn with_details(mut self, details: String) -> Self {
        self.details = Some(details);
        self
    }
    
    
    
    /// Validate the instance
    pub fn validate(&self) -> Result<(), String> {
        // Add validation logic here if needed
        // code is required and present
        // message is required and present
        Ok(())
    }
}

impl Default for Error {
    fn default() -> Self {
        Self {
            code: Default::default(),
            message: Default::default(),
            details: None,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error \{ ")?;
        write!(f, "code: {:?}", self.code)?;
        write!(f, ", ")?;
        write!(f, "message: {:?}", self.message)?;
        write!(f, ", ")?;
        write!(f, "details: {:?}", self.details)?;
        write!(f, " \}")
    }
}