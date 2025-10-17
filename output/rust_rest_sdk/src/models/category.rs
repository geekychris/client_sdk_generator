//! 
//! 

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, NaiveDate, Utc};

/// 
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Category {
    /// Category identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "id")]
    pub id: Option<String>,
    
    /// Category name
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "name")]
    pub name: Option<String>,
    
}

impl Category {
    /// Create a new instance with required fields
    pub fn new() -> Self {
        Self {
            id: None,
            name: None,
        }
    }
    
    /// Set Category identifier
    pub fn with_id(mut self, id: String) -> Self {
        self.id = Some(id);
        self
    }
    
    /// Set Category name
    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }
    
    
    
    /// Validate the instance
    pub fn validate(&self) -> Result<(), String> {
        // Add validation logic here if needed
        Ok(())
    }
}

impl Default for Category {
    fn default() -> Self {
        Self {
            id: None,
            name: None,
        }
    }
}

impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Category \{ ")?;
        write!(f, "id: {:?}", self.id)?;
        write!(f, ", ")?;
        write!(f, "name: {:?}", self.name)?;
        write!(f, " \}")
    }
}