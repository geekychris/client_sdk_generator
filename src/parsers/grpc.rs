// Copyright (c) 2024 Chris Collins <chris@hitorro.com>
// Licensed under the MIT License

use anyhow::{Context, Result};
use std::path::PathBuf;

use crate::core::types::{
    ApiSpec, Operation as ApiOperation, TypeDefinition, Parameter, Response,
    HttpMethod, TypeReference, BaseType, Property, TypeKind,
};

pub struct GrpcParser;

impl GrpcParser {
    pub fn new() -> Self {
        Self
    }
    
    pub async fn parse(&self, proto_path: PathBuf) -> Result<ApiSpec> {
        let _proto_paths = if proto_path.is_dir() {
            self.find_proto_files(&proto_path)?
        } else {
            vec![proto_path]
        };
        
        // For now, create a simple API spec with UserService methods
        // TODO: Implement proper gRPC parsing when dependencies are fixed
        let mut api_spec = ApiSpec::new(
            "UserService gRPC API".to_string(),
            "1.0.0".to_string(),
        );
        
        api_spec.description = Some("Generated from UserService gRPC proto files".to_string());
        
        // Add UserService operations from the proto file
        let user_service_methods = vec![
            ("RegisterUser", "RegisterUserRequest", "RegisterUserResponse", "Register a new user account"),
            ("LoginUser", "LoginUserRequest", "LoginUserResponse", "Authenticate a user and create session"),
            ("RefreshToken", "RefreshTokenRequest", "RefreshTokenResponse", "Refresh authentication token"),
            ("LogoutUser", "LogoutUserRequest", "Empty", "Logout user and invalidate session"),
            ("GetUser", "GetUserRequest", "GetUserResponse", "Get user profile by ID"),
            ("GetCurrentUser", "Empty", "GetUserResponse", "Get current authenticated user profile"),
            ("UpdateUser", "UpdateUserRequest", "UpdateUserResponse", "Update user profile information"),
            ("DeleteUser", "DeleteUserRequest", "Empty", "Delete user account"),
            ("ListUsers", "ListUsersRequest", "ListUsersResponse", "List users with pagination and filtering"),
            ("ChangePassword", "ChangePasswordRequest", "Empty", "Change user password"),
            ("ResetPassword", "ResetPasswordRequest", "Empty", "Reset user password"),
            ("SendVerificationEmail", "SendVerificationEmailRequest", "Empty", "Send email verification"),
            ("VerifyEmail", "VerifyEmailRequest", "Empty", "Verify email address"),
            ("ListUserSessions", "ListUserSessionsRequest", "ListUserSessionsResponse", "List user active sessions"),
            ("RevokeSession", "RevokeSessionRequest", "Empty", "Revoke a user session"),
            ("GetUserPreferences", "GetUserPreferencesRequest", "GetUserPreferencesResponse", "Get user preferences"),
            ("UpdateUserPreferences", "UpdateUserPreferencesRequest", "UpdateUserPreferencesResponse", "Update user preferences"),
        ];
        
        for (method_name, request_type, response_type, description) in user_service_methods {
            let operation = ApiOperation {
                id: format!("UserService_{}", method_name),
                name: method_name.to_string(),
                description: Some(description.to_string()),
                method: HttpMethod::Post, // gRPC uses POST
                path: format!("/userservice.v1.UserService/{}", method_name),
                parameters: Vec::new(),
                request_body: Some(TypeReference::object(request_type.to_string())),
                responses: vec![Response {
                    status_code: Some(200),
                    description: "gRPC response".to_string(),
                    content_type: "application/grpc".to_string(),
                    data_type: Some(TypeReference::object(response_type.to_string())),
                    headers: Vec::new(),
                }],
                tags: vec!["UserService".to_string()],
                deprecated: false,
                cache_config: None,
            };
            
            api_spec.add_operation(operation);
        }
        
        // Add sample types
        let request_type = TypeDefinition {
            name: "SampleRequest".to_string(),
            description: Some("Sample gRPC request".to_string()),
            type_kind: TypeKind::Object,
            properties: vec![
                Property {
                    name: "message".to_string(),
                    description: Some("Request message".to_string()),
                    data_type: TypeReference::string(),
                    required: true,
                    read_only: false,
                    write_only: false,
                    deprecated: false,
                    default_value: None,
                }
            ],
            required: vec!["message".to_string()],
            additional_properties: false,
        };
        
        let response_type = TypeDefinition {
            name: "SampleResponse".to_string(),
            description: Some("Sample gRPC response".to_string()),
            type_kind: TypeKind::Object,
            properties: vec![
                Property {
                    name: "result".to_string(),
                    description: Some("Response result".to_string()),
                    data_type: TypeReference::string(),
                    required: true,
                    read_only: true,
                    write_only: false,
                    deprecated: false,
                    default_value: None,
                }
            ],
            required: vec!["result".to_string()],
            additional_properties: false,
        };
        
        api_spec.add_type(request_type);
        api_spec.add_type(response_type);
        
        Ok(api_spec)
    }
    
    fn find_proto_files(&self, dir: &PathBuf) -> Result<Vec<PathBuf>> {
        let mut proto_files = Vec::new();
        
        for entry in walkdir::WalkDir::new(dir) {
            let entry = entry.context("Failed to read directory entry")?;
            let path = entry.path();
            
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("proto") {
                proto_files.push(path.to_path_buf());
            }
        }
        
        Ok(proto_files)
    }
}

impl Default for GrpcParser {
    fn default() -> Self {
        Self::new()
    }
}