// Rust-specific generator logic

pub struct RustGenerator;

impl RustGenerator {
    pub fn new() -> Self {
        Self
    }
    
    pub fn map_type_to_rust(base_type: &crate::core::types::BaseType) -> String {
        match base_type {
            crate::core::types::BaseType::String => "String".to_string(),
            crate::core::types::BaseType::Integer => "i32".to_string(),
            crate::core::types::BaseType::Long => "i64".to_string(),
            crate::core::types::BaseType::Float => "f32".to_string(),
            crate::core::types::BaseType::Double => "f64".to_string(),
            crate::core::types::BaseType::Boolean => "bool".to_string(),
            crate::core::types::BaseType::Date => "chrono::NaiveDate".to_string(),
            crate::core::types::BaseType::DateTime => "chrono::DateTime<chrono::Utc>".to_string(),
            crate::core::types::BaseType::Binary => "Vec<u8>".to_string(),
            crate::core::types::BaseType::Object(name) => name.clone(),
            crate::core::types::BaseType::Any => "serde_json::Value".to_string(),
            crate::core::types::BaseType::Void => "()".to_string(),
            crate::core::types::BaseType::Array(item_type) => {
                format!("Vec<{}>", Self::map_type_to_rust(&item_type.base_type))
            },
            crate::core::types::BaseType::Map(key_type, value_type) => {
                format!("std::collections::HashMap<{}, {}>", 
                    Self::map_type_to_rust(&key_type.base_type),
                    Self::map_type_to_rust(&value_type.base_type)
                )
            },
        }
    }
}

impl Default for RustGenerator {
    fn default() -> Self {
        Self::new()
    }
}