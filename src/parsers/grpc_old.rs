use anyhow::{Context, Result};
use protobuf_parse::{ProtoPath, pure::parse_and_typecheck};
use std::path::PathBuf;

use crate::core::types::{
    ApiSpec, Operation as ApiOperation, TypeDefinition, Parameter, Response,
    HttpMethod, ParameterType, TypeReference, BaseType, Property, TypeKind,
};

pub struct GrpcParser;

impl GrpcParser {
    pub fn new() -> Self {
        Self
    }
    
    pub async fn parse(&self, proto_path: PathBuf) -> Result<ApiSpec> {
        let proto_paths = if proto_path.is_dir() {
            self.find_proto_files(&proto_path)?
        } else {
            vec![proto_path]
        };
        
        let mut api_spec = ApiSpec::new(
            "gRPC API".to_string(),
            "1.0.0".to_string(),
        );
        
        api_spec.description = Some("Generated from gRPC proto files".to_string());
        
        for proto_file_path in proto_paths {
            self.parse_proto_file(&proto_file_path, &mut api_spec)?;
        }
        
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
    
    fn parse_proto_file(&self, proto_path: &PathBuf, api_spec: &mut ApiSpec) -> Result<()> {
        let proto_path_obj = ProtoPath::new(proto_path.to_string_lossy());
        
        // This is a simplified approach - in practice you'd want to handle includes
        let file_descriptors = parse_and_typecheck(&[proto_path_obj])
            .map_err(|e| anyhow::anyhow!("Failed to parse proto file: {:?}", e))?;
        
        for file_descriptor in &file_descriptors.file {
            self.parse_file_descriptor(file_descriptor, api_spec)?;
        }
        
        Ok(())
    }
    
    fn parse_file_descriptor(
        &self, 
        file_descriptor: &protobuf_parse::FileDescriptorProto,
        api_spec: &mut ApiSpec
    ) -> Result<()> {
        // Parse messages (data types)
        for message in &file_descriptor.message_type {
            let type_def = self.parse_message_type(message)?;
            api_spec.add_type(type_def);
        }
        
        // Parse services (operations)
        for service in &file_descriptor.service {
            self.parse_service(service, api_spec)?;
        }
        
        // Parse enums
        for enum_type in &file_descriptor.enum_type {
            let type_def = self.parse_enum_type(enum_type)?;
            api_spec.add_type(type_def);
        }
        
        Ok(())
    }
    
    fn parse_message_type(
        &self,
        message: &protobuf_parse::DescriptorProto
    ) -> Result<TypeDefinition> {
        let mut type_def = TypeDefinition {
            name: message.name().to_string(),
            description: None,
            type_kind: TypeKind::Object,
            properties: Vec::new(),
            required: Vec::new(),
            additional_properties: false,
        };
        
        // Parse fields
        for field in &message.field {
            let property = self.parse_field(field)?;
            
            // In protobuf, required fields are determined by the label
            if matches!(field.label, Some(protobuf_parse::field_descriptor_proto::Label::Required)) {
                type_def.required.push(property.name.clone());
            }
            
            type_def.properties.push(property);
        }
        
        // Parse nested types
        for nested_message in &message.nested_type {
            // Handle nested messages - for simplicity, we'll add them as separate types
            let nested_type_def = self.parse_message_type(nested_message)?;
            // Note: In a real implementation, you'd need to handle nested type scoping
        }
        
        // Parse nested enums
        for nested_enum in &message.enum_type {
            let enum_type_def = self.parse_enum_type(nested_enum)?;
            // Note: In a real implementation, you'd need to handle nested enum scoping
        }
        
        Ok(type_def)
    }
    
    fn parse_field(
        &self,
        field: &protobuf_parse::FieldDescriptorProto
    ) -> Result<Property> {
        let data_type = self.parse_field_type(field)?;
        
        let property = Property {
            name: field.name().to_string(),
            description: None, // Proto fields don't have built-in descriptions
            data_type,
            required: matches!(field.label, Some(protobuf_parse::field_descriptor_proto::Label::Required)),
            read_only: false,
            write_only: false,
            deprecated: false, // Could check for deprecated option
            default_value: field.default_value.as_ref().map(|v| {
                // Convert proto default value to JSON value
                serde_json::Value::String(v.clone())
            }),
        };
        
        Ok(property)
    }
    
    fn parse_field_type(
        &self,
        field: &protobuf_parse::FieldDescriptorProto
    ) -> Result<TypeReference> {
        use protobuf_parse::field_descriptor_proto::Type;
        
        let base_type = match field.r#type() {
            Type::Double => BaseType::Double,
            Type::Float => BaseType::Float,
            Type::Int64 | Type::Uint64 | Type::Sint64 | Type::Fixed64 | Type::Sfixed64 => BaseType::Long,
            Type::Int32 | Type::Uint32 | Type::Sint32 | Type::Fixed32 | Type::Sfixed32 => BaseType::Integer,
            Type::Bool => BaseType::Boolean,
            Type::String => BaseType::String,
            Type::Bytes => BaseType::Binary,
            Type::Message => {
                // For message types, use the type name
                if let Some(type_name) = &field.type_name {
                    // Remove leading dot and package prefix for simplicity
                    let clean_name = type_name.trim_start_matches('.')
                        .split('.')
                        .last()
                        .unwrap_or(type_name)
                        .to_string();
                    BaseType::Object(clean_name)
                } else {
                    BaseType::Object("Message".to_string())
                }
            }
            Type::Enum => {
                // For enum types, use the type name
                if let Some(type_name) = &field.type_name {
                    let clean_name = type_name.trim_start_matches('.')
                        .split('.')
                        .last()
                        .unwrap_or(type_name)
                        .to_string();
                    BaseType::Object(clean_name)
                } else {
                    BaseType::String // Fallback for unknown enums
                }
            }
            Type::Group => BaseType::Object("Group".to_string()),
        };
        
        let mut type_ref = TypeReference {
            base_type,
            nullable: false,
            array: false,
            generic_parameters: Vec::new(),
        };
        
        // Handle repeated fields (arrays)
        if matches!(field.label, Some(protobuf_parse::field_descriptor_proto::Label::Repeated)) {
            type_ref = TypeReference::array(type_ref);
        }
        
        // Handle optional fields (nullable)
        if matches!(field.label, Some(protobuf_parse::field_descriptor_proto::Label::Optional)) {
            type_ref.nullable = true;
        }
        
        Ok(type_ref)
    }
    
    fn parse_enum_type(
        &self,
        enum_type: &protobuf_parse::EnumDescriptorProto
    ) -> Result<TypeDefinition> {
        let mut type_def = TypeDefinition {
            name: enum_type.name().to_string(),
            description: None,
            type_kind: TypeKind::Enum,
            properties: Vec::new(),
            required: Vec::new(),
            additional_properties: false,
        };
        
        // For enums, add each value as a property
        for value in &enum_type.value {
            let property = Property {
                name: value.name().to_string(),
                description: None,
                data_type: TypeReference::integer(),
                required: false,
                read_only: true,
                write_only: false,
                deprecated: false,
                default_value: Some(serde_json::Value::Number(
                    serde_json::Number::from(value.number())
                )),
            };
            type_def.properties.push(property);
        }
        
        Ok(type_def)
    }
    
    fn parse_service(
        &self,
        service: &protobuf_parse::ServiceDescriptorProto,
        api_spec: &mut ApiSpec
    ) -> Result<()> {
        for method in &service.method {
            let operation = self.parse_method(method, service.name())?;
            api_spec.add_operation(operation);
        }
        
        Ok(())
    }
    
    fn parse_method(
        &self,
        method: &protobuf_parse::MethodDescriptorProto,
        service_name: &str
    ) -> Result<ApiOperation> {
        let operation_id = format!("{}_{}", service_name, method.name());
        
        // Extract input type name
        let input_type = method.input_type.as_ref()
            .map(|type_name| {
                let clean_name = type_name.trim_start_matches('.')
                    .split('.')
                    .last()
                    .unwrap_or(type_name)
                    .to_string();
                TypeReference::object(clean_name)
            });
        
        // Extract output type name
        let output_type = method.output_type.as_ref()
            .map(|type_name| {
                let clean_name = type_name.trim_start_matches('.')
                    .split('.')
                    .last()
                    .unwrap_or(type_name)
                    .to_string();
                TypeReference::object(clean_name)
            });
        
        let operation = ApiOperation {
            id: operation_id.clone(),
            name: method.name().to_string(),
            description: None,
            method: HttpMethod::Post, // gRPC typically maps to HTTP/2 POST
            path: format!("/{}/{}", service_name, method.name()),
            parameters: Vec::new(), // gRPC methods don't have separate parameters
            request_body: input_type,
            responses: vec![Response {
                status_code: Some(200),
                description: "gRPC response".to_string(),
                content_type: "application/grpc".to_string(),
                data_type: output_type,
                headers: Vec::new(),
            }],
            tags: vec![service_name.to_string()],
            deprecated: false,
            cache_config: None,
        };
        
        Ok(operation)
    }
}

impl Default for GrpcParser {
    fn default() -> Self {
        Self::new()
    }
}