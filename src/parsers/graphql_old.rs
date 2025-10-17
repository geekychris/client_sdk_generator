// Copyright (c) 2024 Chris Collins <chris@hitorro.com>
// Licensed under the MIT License

use anyhow::{Context, Result};
use async_graphql_parser::{parse_schema, types::*, Positioned};
use async_graphql_value::Value;
use std::path::PathBuf;

use crate::core::types::{
    ApiSpec, Operation as ApiOperation, TypeDefinition, Parameter, Response,
    HttpMethod, ParameterType, TypeReference, BaseType, Property, TypeKind,
};

pub struct GraphQLParser;

impl GraphQLParser {
    pub fn new() -> Self {
        Self
    }
    
    pub async fn parse(&self, schema_path: PathBuf) -> Result<ApiSpec> {
        let content = std::fs::read_to_string(&schema_path)
            .context("Failed to read GraphQL schema file")?;
        
        let schema = parse_schema(&content)
            .context("Failed to parse GraphQL schema")?;
        
        self.convert_to_api_spec(schema).await
    }
    
    async fn convert_to_api_spec(&self, schema: ServiceDocument) -> Result<ApiSpec> {
        let mut api_spec = ApiSpec::new(
            "GraphQL API".to_string(),
            "1.0.0".to_string(),
        );
        
        api_spec.description = Some("Generated from GraphQL schema".to_string());
        api_spec.base_url = Some("/graphql".to_string());
        
        // Parse type definitions
        for definition in schema.definitions {
            match definition {
                TypeSystemDefinition::Type(type_def) => {
                    match &type_def.node {
                        TypeDefinition::Object(obj_type) => {
                            let api_type = self.parse_object_type(&obj_type)?;
                            api_spec.add_type(api_type);
                            
                            // Check if this is Query or Mutation type
                            if obj_type.name.node == "Query" {
                                self.parse_query_operations(&obj_type, &mut api_spec)?;
                            } else if obj_type.name.node == "Mutation" {
                                self.parse_mutation_operations(&obj_type, &mut api_spec)?;
                            }
                        }
                        TypeDefinition::Interface(interface_type) => {
                            let api_type = self.parse_interface_type(&interface_type)?;
                            api_spec.add_type(api_type);
                        }
                        TypeDefinition::Union(union_type) => {
                            let api_type = self.parse_union_type(&union_type)?;
                            api_spec.add_type(api_type);
                        }
                        TypeDefinition::Enum(enum_type) => {
                            let api_type = self.parse_enum_type(&enum_type)?;
                            api_spec.add_type(api_type);
                        }
                        TypeDefinition::InputObject(input_type) => {
                            let api_type = self.parse_input_object_type(&input_type)?;
                            api_spec.add_type(api_type);
                        }
                        TypeDefinition::Scalar(scalar_type) => {
                            let api_type = self.parse_scalar_type(&scalar_type)?;
                            api_spec.add_type(api_type);
                        }
                    }
                }
                _ => {
                    // Handle other definitions like directives, schema definitions, etc.
                }
            }
        }
        
        Ok(api_spec)
    }
    
    fn parse_query_operations(&self, query_type: &ObjectType, api_spec: &mut ApiSpec) -> Result<()> {
        for field in &query_type.fields {
            let operation = ApiOperation {
                id: format!("query_{}", field.node.name.node),
                name: field.node.name.node.clone(),
                description: field.node.description.as_ref().map(|d| d.node.clone()),
                method: HttpMethod::Post, // GraphQL typically uses POST
                path: "/graphql".to_string(),
                parameters: self.parse_field_arguments(&field.node.arguments)?,
                request_body: Some(TypeReference::object("GraphQLRequest".to_string())),
                responses: vec![Response {
                    status_code: Some(200),
                    description: "GraphQL response".to_string(),
                    content_type: "application/json".to_string(),
                    data_type: Some(self.parse_graphql_type(&field.node.ty.node)?),
                    headers: Vec::new(),
                }],
                tags: vec!["Query".to_string()],
                deprecated: false,
                cache_config: None,
            };
            api_spec.add_operation(operation);
        }
        Ok(())
    }
    
    fn parse_mutation_operations(&self, mutation_type: &ObjectType, api_spec: &mut ApiSpec) -> Result<()> {
        for field in &mutation_type.fields {
            let operation = ApiOperation {
                id: format!("mutation_{}", field.node.name.node),
                name: field.node.name.node.clone(),
                description: field.node.description.as_ref().map(|d| d.node.clone()),
                method: HttpMethod::Post,
                path: "/graphql".to_string(),
                parameters: self.parse_field_arguments(&field.node.arguments)?,
                request_body: Some(TypeReference::object("GraphQLRequest".to_string())),
                responses: vec![Response {
                    status_code: Some(200),
                    description: "GraphQL response".to_string(),
                    content_type: "application/json".to_string(),
                    data_type: Some(self.parse_graphql_type(&field.node.ty.node)?),
                    headers: Vec::new(),
                }],
                tags: vec!["Mutation".to_string()],
                deprecated: false,
                cache_config: None,
            };
            api_spec.add_operation(operation);
        }
        Ok(())
    }
    
    fn parse_field_arguments(&self, arguments: &[Positioned<InputValueDefinition>]) -> Result<Vec<Parameter>> {
        let mut parameters = Vec::new();
        
        for arg in arguments {
            let parameter = Parameter {
                name: arg.node.name.node.clone(),
                description: arg.node.description.as_ref().map(|d| d.node.clone()),
                parameter_type: ParameterType::Query, // GraphQL arguments are like query parameters
                data_type: self.parse_graphql_type(&arg.node.ty.node)?,
                required: matches!(arg.node.ty.node, Type::NonNull(_)),
                default_value: arg.node.default_value.as_ref().map(|v| self.convert_graphql_value(&v.node)),
            };
            parameters.push(parameter);
        }
        
        Ok(parameters)
    }
    
    fn parse_object_type(&self, obj_type: &ObjectType) -> Result<TypeDefinition> {
        let mut type_def = TypeDefinition {
            name: obj_type.name.node.clone(),
            description: obj_type.description.as_ref().map(|d| d.node.clone()),
            type_kind: TypeKind::Object,
            properties: Vec::new(),
            required: Vec::new(),
            additional_properties: false,
        };
        
        for field in &obj_type.fields {
            let property = Property {
                name: field.node.name.node.clone(),
                description: field.node.description.as_ref().map(|d| d.node.clone()),
                data_type: self.parse_graphql_type(&field.node.ty.node)?,
                required: matches!(field.node.ty.node, Type::NonNull(_)),
                read_only: false,
                write_only: false,
                deprecated: false,
                default_value: None,
            };
            
            if property.required {
                type_def.required.push(property.name.clone());
            }
            
            type_def.properties.push(property);
        }
        
        Ok(type_def)
    }
    
    fn parse_interface_type(&self, interface_type: &InterfaceType) -> Result<TypeDefinition> {
        let mut type_def = TypeDefinition {
            name: interface_type.name.node.clone(),
            description: interface_type.description.as_ref().map(|d| d.node.clone()),
            type_kind: TypeKind::Interface,
            properties: Vec::new(),
            required: Vec::new(),
            additional_properties: false,
        };
        
        for field in &interface_type.fields {
            let property = Property {
                name: field.node.name.node.clone(),
                description: field.node.description.as_ref().map(|d| d.node.clone()),
                data_type: self.parse_graphql_type(&field.node.ty.node)?,
                required: matches!(field.node.ty.node, Type::NonNull(_)),
                read_only: false,
                write_only: false,
                deprecated: false,
                default_value: None,
            };
            
            if property.required {
                type_def.required.push(property.name.clone());
            }
            
            type_def.properties.push(property);
        }
        
        Ok(type_def)
    }
    
    fn parse_union_type(&self, union_type: &UnionType) -> Result<TypeDefinition> {
        Ok(TypeDefinition {
            name: union_type.name.node.clone(),
            description: union_type.description.as_ref().map(|d| d.node.clone()),
            type_kind: TypeKind::Union,
            properties: Vec::new(),
            required: Vec::new(),
            additional_properties: false,
        })
    }
    
    fn parse_enum_type(&self, enum_type: &EnumType) -> Result<TypeDefinition> {
        let mut type_def = TypeDefinition {
            name: enum_type.name.node.clone(),
            description: enum_type.description.as_ref().map(|d| d.node.clone()),
            type_kind: TypeKind::Enum,
            properties: Vec::new(),
            required: Vec::new(),
            additional_properties: false,
        };
        
        // For enums, we can add enum values as properties for template processing
        for value in &enum_type.values {
            let property = Property {
                name: value.node.value.node.clone(),
                description: value.node.description.as_ref().map(|d| d.node.clone()),
                data_type: TypeReference::string(),
                required: false,
                read_only: true,
                write_only: false,
                deprecated: false,
                default_value: None,
            };
            type_def.properties.push(property);
        }
        
        Ok(type_def)
    }
    
    fn parse_input_object_type(&self, input_type: &InputObjectType) -> Result<TypeDefinition> {
        let mut type_def = TypeDefinition {
            name: input_type.name.node.clone(),
            description: input_type.description.as_ref().map(|d| d.node.clone()),
            type_kind: TypeKind::Object,
            properties: Vec::new(),
            required: Vec::new(),
            additional_properties: false,
        };
        
        for field in &input_type.fields {
            let property = Property {
                name: field.node.name.node.clone(),
                description: field.node.description.as_ref().map(|d| d.node.clone()),
                data_type: self.parse_graphql_type(&field.node.ty.node)?,
                required: matches!(field.node.ty.node, Type::NonNull(_)),
                read_only: false,
                write_only: true, // Input objects are write-only
                deprecated: false,
                default_value: field.node.default_value.as_ref().map(|v| self.convert_graphql_value(&v.node)),
            };
            
            if property.required {
                type_def.required.push(property.name.clone());
            }
            
            type_def.properties.push(property);
        }
        
        Ok(type_def)
    }
    
    fn parse_scalar_type(&self, scalar_type: &ScalarType) -> Result<TypeDefinition> {
        Ok(TypeDefinition {
            name: scalar_type.name.node.clone(),
            description: scalar_type.description.as_ref().map(|d| d.node.clone()),
            type_kind: TypeKind::Object, // Treat scalars as basic objects
            properties: Vec::new(),
            required: Vec::new(),
            additional_properties: false,
        })
    }
    
    fn parse_graphql_type(&self, gql_type: &Type) -> Result<TypeReference> {
        match gql_type {
            Type::Named(name) => {
                let base_type = match name.node.as_str() {
                    "String" | "ID" => BaseType::String,
                    "Int" => BaseType::Integer,
                    "Float" => BaseType::Float,
                    "Boolean" => BaseType::Boolean,
                    "DateTime" => BaseType::DateTime,
                    type_name => BaseType::Object(type_name.to_string()),
                };
                
                Ok(TypeReference {
                    base_type,
                    nullable: true, // GraphQL types are nullable by default
                    array: false,
                    generic_parameters: Vec::new(),
                })
            }
            Type::NonNull(inner_type) => {
                let mut type_ref = self.parse_graphql_type(inner_type)?;
                type_ref.nullable = false;
                Ok(type_ref)
            }
            Type::List(inner_type) => {
                let item_type = self.parse_graphql_type(inner_type)?;
                Ok(TypeReference::array(item_type))
            }
        }
    }
    
    fn convert_graphql_value(&self, value: &async_graphql_parser::types::Value) -> serde_json::Value {
        match value {
            async_graphql_parser::types::Value::Null => serde_json::Value::Null,
            async_graphql_parser::types::Value::Boolean(b) => serde_json::Value::Bool(*b),
            async_graphql_parser::types::Value::Number(n) => {
                if let Ok(i) = n.as_i64() {
                    serde_json::Value::Number(serde_json::Number::from(i))
                } else if let Ok(f) = n.as_f64() {
                    serde_json::Number::from_f64(f)
                        .map(serde_json::Value::Number)
                        .unwrap_or(serde_json::Value::Null)
                } else {
                    serde_json::Value::Null
                }
            }
            async_graphql_parser::types::Value::String(s) => serde_json::Value::String(s.clone()),
            async_graphql_parser::types::Value::Enum(e) => serde_json::Value::String(e.clone()),
            async_graphql_parser::types::Value::List(list) => {
                let converted_list: Vec<serde_json::Value> = list.iter()
                    .map(|v| self.convert_graphql_value(&v.node))
                    .collect();
                serde_json::Value::Array(converted_list)
            }
            async_graphql_parser::types::Value::Object(obj) => {
                let mut json_obj = serde_json::Map::new();
                for (key, value) in obj {
                    json_obj.insert(key.node.clone(), self.convert_graphql_value(&value.node));
                }
                serde_json::Value::Object(json_obj)
            }
            async_graphql_parser::types::Value::Variable(_) => {
                // Variables should be resolved at runtime
                serde_json::Value::Null
            }
        }
    }
}

impl Default for GraphQLParser {
    fn default() -> Self {
        Self::new()
    }
}