// Copyright (c) 2024 Chris Collins <chris@hitorro.com>
// Licensed under the MIT License

use anyhow::{Context, Result};
use openapiv3::{OpenAPI, ReferenceOr, Schema, SchemaKind, Type, Operation};
use std::path::PathBuf;
use std::collections::HashMap;

use crate::core::types::{
    ApiSpec, Operation as ApiOperation, TypeDefinition, Parameter, Response, 
    HttpMethod, ParameterType, TypeReference, BaseType, Property, TypeKind,
    AuthenticationSpec, AuthenticationType, Header
};

pub struct OpenApiParser {
    // Cache for resolved references
    schema_cache: HashMap<String, Schema>,
}

impl OpenApiParser {
    pub fn new() -> Self {
        Self {
            schema_cache: HashMap::new(),
        }
    }
    
    pub async fn parse(&mut self, spec_path: PathBuf) -> Result<ApiSpec> {
        let content = std::fs::read_to_string(&spec_path)
            .context("Failed to read OpenAPI specification file")?;
        
        let openapi: OpenAPI = if spec_path.extension().and_then(|s| s.to_str()) == Some("json") {
            serde_json::from_str(&content)
                .context("Failed to parse OpenAPI JSON")?
        } else {
            serde_yaml::from_str(&content)
                .context("Failed to parse OpenAPI YAML")?
        };
        
        self.convert_to_api_spec(openapi).await
    }
    
    async fn convert_to_api_spec(&mut self, openapi: OpenAPI) -> Result<ApiSpec> {
        let mut api_spec = ApiSpec::new(
            openapi.info.title.clone(),
            openapi.info.version.clone(),
        );
        
        api_spec.description = openapi.info.description.clone();
        
        // Extract base URL from servers
        if let Some(first_server) = openapi.servers.first() {
            api_spec.base_url = Some(first_server.url.clone());
        }
        
        // Parse operations
        for (path, path_item_ref) in openapi.paths.paths {
            if let ReferenceOr::Item(path_item) = path_item_ref {
                if let Some(operation) = &path_item.get {
                    let api_operation = self.parse_operation(&path, HttpMethod::Get, operation)?;
                    api_spec.add_operation(api_operation);
                }
                if let Some(operation) = &path_item.post {
                    let api_operation = self.parse_operation(&path, HttpMethod::Post, operation)?;
                    api_spec.add_operation(api_operation);
                }
                if let Some(operation) = &path_item.put {
                    let api_operation = self.parse_operation(&path, HttpMethod::Put, operation)?;
                    api_spec.add_operation(api_operation);
                }
                if let Some(operation) = &path_item.delete {
                    let api_operation = self.parse_operation(&path, HttpMethod::Delete, operation)?;
                    api_spec.add_operation(api_operation);
                }
                if let Some(operation) = &path_item.patch {
                    let api_operation = self.parse_operation(&path, HttpMethod::Patch, operation)?;
                    api_spec.add_operation(api_operation);
                }
                if let Some(operation) = &path_item.head {
                    let api_operation = self.parse_operation(&path, HttpMethod::Head, operation)?;
                    api_spec.add_operation(api_operation);
                }
                if let Some(operation) = &path_item.options {
                    let api_operation = self.parse_operation(&path, HttpMethod::Options, operation)?;
                    api_spec.add_operation(api_operation);
                }
                if let Some(operation) = &path_item.trace {
                    let api_operation = self.parse_operation(&path, HttpMethod::Trace, operation)?;
                    api_spec.add_operation(api_operation);
                }
            }
        }
        
        // Parse schemas/components
        if let Some(components) = &openapi.components {
            for (name, schema_ref) in &components.schemas {
                if let ReferenceOr::Item(schema) = schema_ref {
                    let type_def = self.parse_schema_to_type_definition(name, schema)?;
                    api_spec.add_type(type_def);
                }
            }
            
            // Parse security schemes
            for (_name, security_ref) in &components.security_schemes {
                if let ReferenceOr::Item(security) = security_ref {
                    let auth_spec = self.parse_security_scheme(security)?;
                    api_spec.authentication = Some(auth_spec);
                    break; // For simplicity, take the first one
                }
            }
        }
        
        Ok(api_spec)
    }
    
    fn parse_operation(&self, path: &str, method: HttpMethod, operation: &Operation) -> Result<ApiOperation> {
        let operation_id = operation.operation_id.clone()
            .unwrap_or_else(|| format!("{}_{}", method.to_string().to_lowercase(), path.replace('/', "_").replace(['{', '}'], "")));
        
        let mut api_operation = ApiOperation {
            id: operation_id.clone(),
            name: operation_id,
            description: operation.description.clone(),
            method,
            path: path.to_string(),
            parameters: Vec::new(),
            request_body: None,
            responses: Vec::new(),
            tags: operation.tags.clone(),
            deprecated: operation.deprecated,
            cache_config: None,
        };
        
        // Parse parameters
        for param_ref in &operation.parameters {
            if let ReferenceOr::Item(param) = param_ref {
                let api_param = Parameter {
                    name: param.parameter_data_ref().name.clone(),
                    description: param.parameter_data_ref().description.clone(),
                    parameter_type: match param {
                        openapiv3::Parameter::Query { .. } => ParameterType::Query,
                        openapiv3::Parameter::Path { .. } => ParameterType::Path,
                        openapiv3::Parameter::Header { .. } => ParameterType::Header,
                        openapiv3::Parameter::Cookie { .. } => ParameterType::Cookie,
                    },
                    data_type: TypeReference::string(), // Simplified for now
                    required: param.parameter_data_ref().required,
                    default_value: None,
                };
                api_operation.parameters.push(api_param);
            }
        }
        
        // Parse request body
        if let Some(request_body_ref) = &operation.request_body {
            if let ReferenceOr::Item(request_body) = request_body_ref {
                if let Some(content) = request_body.content.get("application/json") {
                    if let Some(schema_ref) = &content.schema {
                        if let ReferenceOr::Item(schema) = schema_ref {
                            api_operation.request_body = Some(self.parse_schema_to_type_reference(schema)?);
                        }
                    }
                }
            }
        }
        
        // Parse responses
        for (status_code, response_ref) in &operation.responses.responses {
            if let ReferenceOr::Item(response) = response_ref {
                let status = match status_code {
                    openapiv3::StatusCode::Code(code) => Some(*code),
                    _ => None,
                };
                
                let mut api_response = Response {
                    status_code: status,
                    description: response.description.clone(),
                    content_type: "application/json".to_string(),
                    data_type: None,
                    headers: Vec::new(),
                };
                
                // Parse response content
                if let Some(content) = response.content.get("application/json") {
                    if let Some(schema_ref) = &content.schema {
                        if let ReferenceOr::Item(schema) = schema_ref {
                            api_response.data_type = Some(self.parse_schema_to_type_reference(schema)?);
                        }
                    }
                }
                
                // Parse headers
                for (header_name, header_ref) in &response.headers {
                    if let ReferenceOr::Item(header) = header_ref {
                        let api_header = Header {
                            name: header_name.clone(),
                            description: header.description.clone(),
                            data_type: TypeReference::string(), // Simplified
                            required: false, // OpenAPI headers are optional by default
                        };
                        api_response.headers.push(api_header);
                    }
                }
                
                api_operation.responses.push(api_response);
            }
        }
        
        Ok(api_operation)
    }
    
    fn parse_schema_to_type_definition(&self, name: &str, schema: &Schema) -> Result<TypeDefinition> {
        let mut type_def = TypeDefinition {
            name: name.to_string(),
            description: schema.schema_data.description.clone(),
            type_kind: TypeKind::Object,
            properties: Vec::new(),
            required: Vec::new(),
            additional_properties: false,
        };
        
        match &schema.schema_kind {
            SchemaKind::Type(Type::Object(obj)) => {
                type_def.type_kind = TypeKind::Object;
                type_def.required = obj.required.clone();
                type_def.additional_properties = obj.additional_properties.is_some();
                
                // Parse properties
                for (prop_name, prop_schema_ref) in &obj.properties {
                    if let ReferenceOr::Item(prop_schema) = prop_schema_ref {
                        let property = Property {
                            name: prop_name.clone(),
                            description: prop_schema.schema_data.description.clone(),
                            data_type: self.parse_schema_to_type_reference(prop_schema)?,
                            required: obj.required.contains(prop_name),
                            read_only: prop_schema.schema_data.read_only,
                            write_only: prop_schema.schema_data.write_only,
                            deprecated: prop_schema.schema_data.deprecated,
                            default_value: prop_schema.schema_data.default.clone(),
                        };
                        type_def.properties.push(property);
                    }
                }
            }
            SchemaKind::Type(Type::Array(_arr)) => {
                type_def.type_kind = TypeKind::Array;
                // For arrays, we create a wrapper type
            }
            SchemaKind::OneOf { one_of: _ } => {
                type_def.type_kind = TypeKind::Union;
                // Handle union types
            }
            SchemaKind::AllOf { all_of: _ } => {
                type_def.type_kind = TypeKind::Interface;
                // Handle interface/inheritance
            }
            _ => {
                // Handle other schema kinds
            }
        }
        
        Ok(type_def)
    }
    
    fn parse_schema_to_type_reference(&self, schema: &Schema) -> Result<TypeReference> {
        match &schema.schema_kind {
            SchemaKind::Type(Type::String(string_type)) => {
                let base_type = match &string_type.format {
                    openapiv3::VariantOrUnknownOrEmpty::Item(openapiv3::StringFormat::Date) => BaseType::Date,
                    openapiv3::VariantOrUnknownOrEmpty::Item(openapiv3::StringFormat::DateTime) => BaseType::DateTime,
                    openapiv3::VariantOrUnknownOrEmpty::Item(openapiv3::StringFormat::Binary) => BaseType::Binary,
                    _ => BaseType::String,
                };
                Ok(TypeReference {
                    base_type,
                    nullable: schema.schema_data.nullable,
                    array: false,
                    generic_parameters: Vec::new(),
                })
            }
            SchemaKind::Type(Type::Integer(int_type)) => {
                let base_type = match int_type.format {
                    openapiv3::VariantOrUnknownOrEmpty::Item(openapiv3::IntegerFormat::Int64) => BaseType::Long,
                    _ => BaseType::Integer,
                };
                Ok(TypeReference {
                    base_type,
                    nullable: schema.schema_data.nullable,
                    array: false,
                    generic_parameters: Vec::new(),
                })
            }
            SchemaKind::Type(Type::Number(num_type)) => {
                let base_type = match num_type.format {
                    openapiv3::VariantOrUnknownOrEmpty::Item(openapiv3::NumberFormat::Double) => BaseType::Double,
                    _ => BaseType::Float,
                };
                Ok(TypeReference {
                    base_type,
                    nullable: schema.schema_data.nullable,
                    array: false,
                    generic_parameters: Vec::new(),
                })
            }
            SchemaKind::Type(Type::Boolean(_)) => {
                Ok(TypeReference {
                    base_type: BaseType::Boolean,
                    nullable: schema.schema_data.nullable,
                    array: false,
                    generic_parameters: Vec::new(),
                })
            }
            SchemaKind::Type(Type::Array(array_type)) => {
                if let Some(ReferenceOr::Item(item_schema)) = &array_type.items {
                    let item_type = self.parse_schema_to_type_reference(item_schema)?;
                    Ok(TypeReference::array(item_type))
                } else {
                    Ok(TypeReference {
                        base_type: BaseType::Array(Box::new(TypeReference::string())),
                        nullable: schema.schema_data.nullable,
                        array: true,
                        generic_parameters: Vec::new(),
                    })
                }
            }
            SchemaKind::Type(Type::Object(_)) => {
                // For object references, we need to determine the object name
                // This is simplified - in practice, you'd resolve $ref
                Ok(TypeReference {
                    base_type: BaseType::Object("Object".to_string()),
                    nullable: schema.schema_data.nullable,
                    array: false,
                    generic_parameters: Vec::new(),
                })
            }
            _ => {
                Ok(TypeReference {
                    base_type: BaseType::Any,
                    nullable: schema.schema_data.nullable,
                    array: false,
                    generic_parameters: Vec::new(),
                })
            }
        }
    }
    
    fn parse_security_scheme(&self, security: &openapiv3::SecurityScheme) -> Result<AuthenticationSpec> {
        let auth_type = match security {
            openapiv3::SecurityScheme::APIKey { .. } => AuthenticationType::ApiKey,
            openapiv3::SecurityScheme::HTTP { scheme, .. } => {
                match scheme.as_str() {
                    "basic" => AuthenticationType::Basic,
                    "bearer" => AuthenticationType::Bearer,
                    _ => AuthenticationType::Custom(scheme.clone()),
                }
            }
            openapiv3::SecurityScheme::OAuth2 { .. } => AuthenticationType::OAuth2,
            openapiv3::SecurityScheme::OpenIDConnect { .. } => AuthenticationType::OpenIdConnect,
        };
        
        Ok(AuthenticationSpec {
            auth_type,
            flows: Vec::new(), // Simplified for now
        })
    }
    
}

impl Default for OpenApiParser {
    fn default() -> Self {
        Self::new()
    }
}