//! 
//! 

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, NaiveDate, Utc};

/// 
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Newpet {
    /// Name of the pet
    #[serde(rename = "name")]
    pub name: String,
    
    /// Tag associated with the pet
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tag")]
    pub tag: Option<String>,
    
    /// URLs of pet photos
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "photoUrls")]
    pub photo_urls: Option<String>,
    
}

impl Newpet {
    /// Create a new instance with required fields
    pub fn new(name: String, ) -> Self {
        Self {
            name,
            tag: None,
            photo_urls: None,
        }
    }
    
    
    /// Set Tag associated with the pet
    pub fn with_tag(mut self, tag: String) -> Self {
        self.tag = Some(tag);
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
        // name is required and present
        Ok(())
    }
}

impl Default for Newpet {
    fn default() -> Self {
        Self {
            name: Default::default(),
            tag: None,
            photo_urls: None,
        }
    }
}

impl std::fmt::Display for Newpet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Newpet \{ ")?;
        write!(f, "name: {:?}", self.name)?;
        write!(f, ", ")?;
        write!(f, "tag: {:?}", self.tag)?;
        write!(f, ", ")?;
        write!(f, "photo_urls: {:?}", self.photo_urls)?;
        write!(f, " \}")
    }
}