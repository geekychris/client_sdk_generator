// Copyright (c) 2024 Chris Collins <chris@hitorro.com>
// Licensed under the MIT License

use client_sdk_generator::core::config::TargetLanguage;
use std::str::FromStr;

#[test]
fn test_target_language_parsing() {
    match TargetLanguage::from_str("python") {
        Ok(lang) => assert_eq!(lang, TargetLanguage::Python),
        Err(_) => panic!("Failed to parse python"),
    }
    
    match TargetLanguage::from_str("rust") {
        Ok(lang) => assert_eq!(lang, TargetLanguage::Rust),
        Err(_) => panic!("Failed to parse rust"),
    }
    
    match TargetLanguage::from_str("java") {
        Ok(lang) => assert_eq!(lang, TargetLanguage::Java),
        Err(_) => panic!("Failed to parse java"),
    }
    
    match TargetLanguage::from_str("go") {
        Ok(lang) => assert_eq!(lang, TargetLanguage::Go),
        Err(_) => panic!("Failed to parse go"),
    }
    
    match TargetLanguage::from_str("typescript") {
        Ok(lang) => assert_eq!(lang, TargetLanguage::TypeScript),
        Err(_) => panic!("Failed to parse typescript"),
    }
    
    // Test aliases
    match TargetLanguage::from_str("golang") {
        Ok(lang) => assert_eq!(lang, TargetLanguage::Go),
        Err(_) => panic!("Failed to parse golang alias"),
    }
    
    match TargetLanguage::from_str("ts") {
        Ok(lang) => assert_eq!(lang, TargetLanguage::TypeScript),
        Err(_) => panic!("Failed to parse ts alias"),
    }
    
    assert!(TargetLanguage::from_str("invalid").is_err());
}

#[test] 
fn test_basic_functionality() {
    // Just test that we can create target languages
    let python = TargetLanguage::Python;
    let rust = TargetLanguage::Rust;
    let java = TargetLanguage::Java;
    let go = TargetLanguage::Go;
    let typescript = TargetLanguage::TypeScript;
    
    assert_eq!(python, TargetLanguage::Python);
    assert_eq!(rust, TargetLanguage::Rust);
    assert_eq!(java, TargetLanguage::Java);
    assert_eq!(go, TargetLanguage::Go);
    assert_eq!(typescript, TargetLanguage::TypeScript);
}

#[test]
fn test_authentication_configuration() {
    use client_sdk_generator::core::config::*;
    
    // Test API Key authentication
    let api_key_auth = AuthenticationConfig {
        auth_type: AuthenticationType::ApiKey,
        location: AuthLocation::Header,
        parameter_name: Some("X-API-Key".to_string()),
        scheme: None,
        bearer_format: None,
        flows: None,
        openid_connect_url: None,
    };
    
    assert!(matches!(api_key_auth.auth_type, AuthenticationType::ApiKey));
    assert!(matches!(api_key_auth.location, AuthLocation::Header));
    assert_eq!(api_key_auth.parameter_name, Some("X-API-Key".to_string()));
    
    // Test HTTP Bearer authentication
    let bearer_auth = AuthenticationConfig {
        auth_type: AuthenticationType::Http,
        location: AuthLocation::Header,
        parameter_name: Some("Authorization".to_string()),
        scheme: Some("Bearer".to_string()),
        bearer_format: Some("JWT".to_string()),
        flows: None,
        openid_connect_url: None,
    };
    
    assert!(matches!(bearer_auth.auth_type, AuthenticationType::Http));
    assert_eq!(bearer_auth.scheme, Some("Bearer".to_string()));
    assert_eq!(bearer_auth.bearer_format, Some("JWT".to_string()));
    
    // Test OAuth2 authentication
    let oauth2_auth = AuthenticationConfig {
        auth_type: AuthenticationType::OAuth2,
        location: AuthLocation::Header,
        parameter_name: None,
        scheme: None,
        bearer_format: None,
        flows: Some(vec![OAuthFlow {
            flow_type: OAuthFlowType::AuthorizationCode,
            authorization_url: Some("https://example.com/auth".to_string()),
            token_url: Some("https://example.com/token".to_string()),
            refresh_url: None,
            scopes: std::collections::HashMap::new(),
        }]),
        openid_connect_url: None,
    };
    
    assert!(matches!(oauth2_auth.auth_type, AuthenticationType::OAuth2));
    assert!(oauth2_auth.flows.is_some());
    let flows = oauth2_auth.flows.unwrap();
    assert_eq!(flows.len(), 1);
    assert!(matches!(flows[0].flow_type, OAuthFlowType::AuthorizationCode));
}
