//! 
//! 

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, NaiveDate, Utc};

/// 
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pet {
    /// Unique identifier for the pet
    #[serde(rename = "id")]
    pub id: String,
    
    /// Name of the pet
    #[serde(rename = "name")]
    pub name: String,
    
    /// Tag associated with the pet
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tag")]
    pub tag: Option<String>,
    
    /// Pet status in the store
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "status")]
    pub status: Option<String>,
    
    /// URLs of pet photos
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "photoUrls")]
    pub photo_urls: Option<String>,
    
}

impl Pet {
    /// Create a new instance with required fields
    pub fn new(id: String, name: String, ) -> Self {
        Self {
            id,
            name,
            tag: None,
            status: None,
            photo_urls: None,
        }
    }
    
    
    
    /// Set Tag associated with the pet
    pub fn with_tag(mut self, tag: String) -> Self {
        self.tag = Some(tag);
        self
    }
    
    /// Set Pet status in the store
    pub fn with_status(mut self, status: String) -> Self {
        self.status = Some(status);
        self
    }
    
    /// Set URLs of pet photos
    pub fn with_photo_urls(mut self, photo_urls: String) -> Self {
        self.photo_urls = Some(photo_urls);
        self
    }
    
    
    
    /// Validate the instance
    pub fn validate(&self) -> Result<(), String> {
        // Add validation logic here if needed
        // id is required and present
        // name is required and present
        Ok(())
    }
}

impl Default for Pet {
    fn default() -> Self {
        Self {
            id: Default::default(),
            name: Default::default(),
            tag: None,
            status: None,
            photo_urls: None,
        }
    }
}

impl std::fmt::Display for Pet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Pet \{ ")?;
        write!(f, "id: {:?}", self.id)?;
        write!(f, ", ")?;
        write!(f, "name: {:?}", self.name)?;
        write!(f, ", ")?;
        write!(f, "tag: {:?}", self.tag)?;
        write!(f, ", ")?;
        write!(f, "status: {:?}", self.status)?;
        write!(f, ", ")?;
        write!(f, "photo_urls: {:?}", self.photo_urls)?;
        write!(f, " \}")
    }
}