// Copyright (c) 2024 Chris Collins <chris@hitorro.com>
// Licensed under the MIT License

pub mod config;
pub mod features;
pub mod generator;
pub mod template;
pub mod types;
pub mod test_generation;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_config_default() {
        let config = config::GeneratorConfig::default_for(
            config::InputType::OpenApi,
            config::TargetLanguage::Java,
        );
        
        assert!(matches!(config.input_type, config::InputType::OpenApi));
        assert!(matches!(config.target_language, config::TargetLanguage::Java));
        assert!(config.features.async_support);
    }
    
    #[test]
    fn test_target_language_from_str() {
        use std::str::FromStr;
        
        assert!(matches!(
            config::TargetLanguage::from_str("java").unwrap(),
            config::TargetLanguage::Java
        ));
        assert!(matches!(
            config::TargetLanguage::from_str("Python").unwrap(),
            config::TargetLanguage::Python
        ));
        assert!(matches!(
            config::TargetLanguage::from_str("RUST").unwrap(),
            config::TargetLanguage::Rust
        ));
        
        assert!(config::TargetLanguage::from_str("invalid").is_err());
    }
    
    #[test]
    fn test_api_spec_creation() {
        let mut api_spec = types::ApiSpec::new(
            "Test API".to_string(),
            "1.0.0".to_string(),
        );
        
        assert_eq!(api_spec.name, "Test API");
        assert_eq!(api_spec.version, "1.0.0");
        assert_eq!(api_spec.operations.len(), 0);
        assert_eq!(api_spec.types.len(), 0);
        
        // Test adding operations and types
        let operation = types::Operation {
            id: "test_op".to_string(),
            name: "testOp".to_string(),
            description: None,
            method: types::HttpMethod::Get,
            path: "/test".to_string(),
            parameters: Vec::new(),
            request_body: None,
            responses: Vec::new(),
            tags: Vec::new(),
            deprecated: false,
            cache_config: None,
        };
        
        api_spec.add_operation(operation);
        assert_eq!(api_spec.operations.len(), 1);
        
        let type_def = types::TypeDefinition {
            name: "TestType".to_string(),
            description: None,
            type_kind: types::TypeKind::Object,
            properties: Vec::new(),
            required: Vec::new(),
            additional_properties: false,
        };
        
        api_spec.add_type(type_def);
        assert_eq!(api_spec.types.len(), 1);
    }
    
    #[test]
    fn test_type_reference_builders() {
        let string_ref = types::TypeReference::string();
        assert!(matches!(string_ref.base_type, types::BaseType::String));
        assert!(!string_ref.nullable);
        assert!(!string_ref.array);
        
        let nullable_string = types::TypeReference::string().nullable();
        assert!(nullable_string.nullable);
        
        let array_string = types::TypeReference::array(types::TypeReference::string());
        assert!(array_string.array);
    }
}
