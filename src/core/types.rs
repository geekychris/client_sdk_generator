use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Common representation of an API specification that can be generated from
/// OpenAPI, GraphQL, or gRPC specifications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiSpec {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub base_url: Option<String>,
    pub operations: Vec<Operation>,
    pub types: Vec<TypeDefinition>,
    pub authentication: Option<AuthenticationSpec>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub method: HttpMethod,
    pub path: String,
    pub parameters: Vec<Parameter>,
    pub request_body: Option<TypeReference>,
    pub responses: Vec<Response>,
    pub tags: Vec<String>,
    pub deprecated: bool,
    pub cache_config: Option<CacheOperation>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Options,
    Head,
    Trace,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub description: Option<String>,
    pub parameter_type: ParameterType,
    pub data_type: TypeReference,
    pub required: bool,
    pub default_value: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParameterType {
    Query,
    Path,
    Header,
    Cookie,
    FormData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub status_code: Option<u16>,
    pub description: String,
    pub content_type: String,
    pub data_type: Option<TypeReference>,
    pub headers: Vec<Header>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Header {
    pub name: String,
    pub description: Option<String>,
    pub data_type: TypeReference,
    pub required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeDefinition {
    pub name: String,
    pub description: Option<String>,
    pub type_kind: TypeKind,
    pub properties: Vec<Property>,
    pub required: Vec<String>,
    pub additional_properties: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TypeKind {
    Object,
    Array,
    String,
    Integer,
    Float,
    Boolean,
    Enum,
    Union,
    Interface,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Property {
    pub name: String,
    pub description: Option<String>,
    pub data_type: TypeReference,
    pub required: bool,
    pub read_only: bool,
    pub write_only: bool,
    pub deprecated: bool,
    pub default_value: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeReference {
    pub base_type: BaseType,
    pub nullable: bool,
    pub array: bool,
    pub generic_parameters: Vec<TypeReference>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BaseType {
    // Primitive types
    String,
    Integer,
    Long,
    Float,
    Double,
    Boolean,
    Date,
    DateTime,
    Binary,
    
    // Complex types
    Object(String), // Reference to a type definition
    Any,
    Void,
    
    // Collection types
    Array(Box<TypeReference>),
    Map(Box<TypeReference>, Box<TypeReference>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationSpec {
    pub auth_type: AuthenticationType,
    pub flows: Vec<AuthFlow>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthenticationType {
    ApiKey,
    Bearer,
    Basic,
    OAuth2,
    OpenIdConnect,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthFlow {
    pub flow_type: String,
    pub authorization_url: Option<String>,
    pub token_url: Option<String>,
    pub scopes: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheOperation {
    pub enabled: bool,
    pub ttl_seconds: Option<u64>,
    pub cache_key: Option<String>,
    pub vary_on: Vec<String>,
}

impl ApiSpec {
    pub fn new(name: String, version: String) -> Self {
        Self {
            name,
            version,
            description: None,
            base_url: None,
            operations: Vec::new(),
            types: Vec::new(),
            authentication: None,
        }
    }
    
    pub fn add_operation(&mut self, operation: Operation) {
        self.operations.push(operation);
    }
    
    pub fn add_type(&mut self, type_def: TypeDefinition) {
        self.types.push(type_def);
    }
    
    pub fn get_operation_by_id(&self, id: &str) -> Option<&Operation> {
        self.operations.iter().find(|op| op.id == id)
    }
    
    pub fn get_type_by_name(&self, name: &str) -> Option<&TypeDefinition> {
        self.types.iter().find(|t| t.name == name)
    }
}

impl TypeReference {
    pub fn string() -> Self {
        Self {
            base_type: BaseType::String,
            nullable: false,
            array: false,
            generic_parameters: Vec::new(),
        }
    }
    
    pub fn integer() -> Self {
        Self {
            base_type: BaseType::Integer,
            nullable: false,
            array: false,
            generic_parameters: Vec::new(),
        }
    }
    
    pub fn boolean() -> Self {
        Self {
            base_type: BaseType::Boolean,
            nullable: false,
            array: false,
            generic_parameters: Vec::new(),
        }
    }
    
    pub fn object(name: String) -> Self {
        Self {
            base_type: BaseType::Object(name),
            nullable: false,
            array: false,
            generic_parameters: Vec::new(),
        }
    }
    
    pub fn array(item_type: TypeReference) -> Self {
        Self {
            base_type: BaseType::Array(Box::new(item_type)),
            nullable: false,
            array: true,
            generic_parameters: Vec::new(),
        }
    }
    
    pub fn nullable(mut self) -> Self {
        self.nullable = true;
        self
    }
}

impl std::fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpMethod::Get => write!(f, "GET"),
            HttpMethod::Post => write!(f, "POST"),
            HttpMethod::Put => write!(f, "PUT"),
            HttpMethod::Delete => write!(f, "DELETE"),
            HttpMethod::Patch => write!(f, "PATCH"),
            HttpMethod::Head => write!(f, "HEAD"),
            HttpMethod::Options => write!(f, "OPTIONS"),
            HttpMethod::Trace => write!(f, "TRACE"),
        }
    }
}