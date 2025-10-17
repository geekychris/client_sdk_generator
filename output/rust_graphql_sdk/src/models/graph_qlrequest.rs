//! GraphQL request object
//! 

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, NaiveDate, Utc};

/// GraphQL request object
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Graphqlrequest {
    /// GraphQL query string
    #[serde(rename = "query")]
    pub query: String,
    
}

impl Graphqlrequest {
    /// Create a new instance with required fields
    pub fn new(query: String) -> Self {
        Self {
            query,
        }
    }
    
    
    
    
    /// Validate the instance
    pub fn validate(&self) -> Result<(), String> {
        // Add validation logic here if needed
        // query is required and present
        Ok(())
    }
}

impl Default for Graphqlrequest {
    fn default() -> Self {
        Self {
            query: Default::default(),
        }
    }
}

impl std::fmt::Display for Graphqlrequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Graphqlrequest \{ ")?;
        write!(f, "query: {:?}", self.query)?;
        write!(f, " \}")
    }
}