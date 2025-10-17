# Client SDK Generator

A comprehensive, multi-language client SDK generator that creates robust, feature-rich SDKs from OpenAPI specifications, GraphQL schemas, and gRPC protocol definitions.

## Features

- **Multiple Input Formats**: OpenAPI 3.0+, GraphQL schemas, gRPC proto files
- **Multi-Language Support**: Java, Python, and Rust with extensible architecture
- **Built-in Best Practices**: 
  - Automatic retry logic with configurable backoff strategies
  - Telemetry integration (Prometheus, OpenTelemetry)
  - Client-side caching with TTL and size limits
  - Async/await patterns where supported
  - Type-safe request/response models
- **Production Ready**: Generated SDKs include proper error handling, logging, and configuration
- **Extensible**: Plugin architecture for adding new languages and features

## Quick Start

### Installation

Build from source:
```bash
git clone <repository>
cd client-sdk-generator
cargo build --release
```

### Basic Usage

The SDK generator can be used both as a command-line tool and programmatically. Here are examples for both approaches:

#### Command Line Usage

Generate a Java SDK from OpenAPI specification:
```bash
./target/release/sdk-gen openapi \
  --spec examples/petstore-openapi.yaml \
  --language java \
  --output ./generated-java-sdk
```

Generate a Python SDK from GraphQL schema:
```bash
./target/release/sdk-gen graphql \
  --schema examples/petstore.schema \
  --language python \
  --output ./generated-python-sdk
```

Generate a Rust SDK from gRPC proto files:
```bash
./target/release/sdk-gen grpc \
  --proto examples/petstore.proto \
  --language rust \
  --output ./generated-rust-sdk
```

With configuration file:
```bash
./target/release/sdk-gen openapi \
  --spec examples/petstore-openapi.yaml \
  --language java \
  --output ./generated-java-sdk \
  --config config.yaml
```

List available generators and capabilities:
```bash
./target/release/sdk-gen list
```

#### Programmatic Usage

The SDK generator is also designed to be used programmatically. Here are examples for different input formats:

**Generate a Java SDK from OpenAPI specification:**
```rust
use client_sdk_generator::core::config::{GeneratorConfig, TargetLanguage, OutputConfig, Features};
use client_sdk_generator::core::generator::SdkGenerator;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = GeneratorConfig {
        target_language: TargetLanguage::Java,
        output_config: OutputConfig {
            package_name: Some("com.example.petstore".to_string()),
            version: "1.0.0".to_string(),
            author: Some("Your Name".to_string()),
            license: Some("MIT".to_string()),
            include_tests: true,
            include_docs: true,
            format_code: true,
        },
        features: Features::default(),
        template_overrides: None,
    };
    
    let generator = SdkGenerator::new(config)?;
    let spec_path = PathBuf::from("examples/petstore-openapi.yaml");
    let output_path = PathBuf::from("./generated-java-sdk");
    
    generator.generate_from_openapi(spec_path, output_path).await?;
    Ok(())
}
```

**Generate a Python SDK from GraphQL schema:**
```rust
let config = GeneratorConfig {
    target_language: TargetLanguage::Python,
    output_config: OutputConfig {
        package_name: Some("graphql_client".to_string()),
        version: "1.0.0".to_string(),
        // ... other config
    },
    // ... other settings
};

let generator = SdkGenerator::new(config)?;
let schema_path = PathBuf::from("examples/schema.graphql");
let output_path = PathBuf::from("./generated-python-sdk");

generator.generate_from_graphql(schema_path, output_path).await?;
```

**Generate a Rust SDK from gRPC proto files:**
```rust
let config = GeneratorConfig {
    target_language: TargetLanguage::Rust,
    output_config: OutputConfig {
        package_name: Some("grpc-client".to_string()),
        version: "1.0.0".to_string(),
        // ... other config
    },
    // ... other settings
};

let generator = SdkGenerator::new(config)?;
let proto_path = PathBuf::from("examples/service.proto");
let output_path = PathBuf::from("./generated-rust-sdk");

generator.generate_from_grpc(proto_path, output_path).await?;
```

## Configuration

Create a configuration file to customize generation:

```yaml
# config.yaml
input_type: OpenApi
target_language: Java
features:
  retry:
    enabled: true
    max_attempts: 3
    backoff_strategy:
      Exponential:
        initial_delay_ms: 100
        multiplier: 2.0
        max_delay_ms: 5000
    retry_on:
      - "5xx"
      - "408"
      - "429"
  
  telemetry:
    enabled: true
    provider: Prometheus
    metrics:
      enabled: true
      include_request_duration: true
      include_request_count: true
      include_error_count: true
    tracing:
      enabled: true
      include_request_body: false
      include_response_body: false
      sample_rate: 1.0
  
  caching:
    enabled: false
    default_ttl_seconds: 300
    max_cache_size: 1000
    cache_key_strategy: MethodAndParams
  
  async_support: true

output_config:
  package_name: "com.example.petstore"
  version: "1.0.0"
  author: "Your Name"
  license: "MIT"
  include_tests: true
  include_docs: true
  format_code: true
```

Use with configuration:
```rust
use client_sdk_generator::core::config::{
    GeneratorConfig, TargetLanguage, OutputConfig, Features,
    RetryConfig, BackoffStrategy, TelemetryConfig, TelemetryProvider
};
use client_sdk_generator::core::generator::SdkGenerator;
use std::path::PathBuf;

// Load configuration from YAML file or create programmatically
let config = GeneratorConfig {
    target_language: TargetLanguage::Java,
    output_config: OutputConfig {
        package_name: Some("com.example.petstore".to_string()),
        version: "1.0.0".to_string(),
        author: Some("Your Name".to_string()),
        license: Some("MIT".to_string()),
        include_tests: true,
        include_docs: true,
        format_code: true,
    },
    features: Features {
        retry: RetryConfig {
            enabled: true,
            max_attempts: 3,
            backoff_strategy: BackoffStrategy::Exponential {
                initial_delay_ms: 100,
                multiplier: 2.0,
                max_delay_ms: 5000,
            },
            retry_on: vec!["5xx".to_string(), "408".to_string(), "429".to_string()],
        },
        telemetry: TelemetryConfig {
            enabled: true,
            provider: TelemetryProvider::Prometheus,
            // ... other telemetry settings
        },
        // ... other feature configs
    },
    template_overrides: None,
};

let generator = SdkGenerator::new(config)?;
generator.generate_from_openapi(spec_path, output_path).await?;
```

## Complete Working Examples

For complete working examples that you can run, see the files in the `examples/` directory:

### OpenAPI SDK Generation

```rust
// examples/generate_openapi_sdks.rs
use client_sdk_generator::core::config::{GeneratorConfig, TargetLanguage, OutputConfig, Features};
use client_sdk_generator::core::generator::SdkGenerator;
use std::path::PathBuf;
use tokio;

/// Example demonstrating OpenAPI SDK generation for multiple languages
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Generating OpenAPI SDKs for PetStore API");

    let spec_path = PathBuf::from("examples/petstore-openapi.yaml");
    
    // Generate Java SDK
    generate_java_sdk(&spec_path).await?;
    
    // Generate Python SDK
    generate_python_sdk(&spec_path).await?;
    
    // Generate Rust SDK
    generate_rust_sdk(&spec_path).await?;
    
    println!("✅ All SDKs generated successfully!");
    Ok(())
}

async fn generate_java_sdk(spec_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let config = GeneratorConfig {
        target_language: TargetLanguage::Java,
        output_config: OutputConfig {
            package_name: Some("com.example.petstore".to_string()),
            version: "1.0.0".to_string(),
            author: Some("SDK Generator".to_string()),
            license: Some("MIT".to_string()),
            include_tests: true,
            include_docs: true,
            format_code: true,
        },
        features: Features {
            retry: Default::default(),
            telemetry: Default::default(),
            caching: Default::default(),
            async_support: true,
        },
        template_overrides: None,
    };
    
    let generator = SdkGenerator::new(config)?;
    let output_path = PathBuf::from("generated/petstore-java-sdk");
    
    generator.generate_from_openapi(spec_path.clone(), output_path).await?;
    println!("✅ Java SDK generated");
    Ok(())
}
```

### GraphQL SDK Generation

```rust
// examples/generate_graphql_sdks.rs
use client_sdk_generator::core::config::{GeneratorConfig, TargetLanguage, OutputConfig, Features};
use client_sdk_generator::core::generator::SdkGenerator;
use std::path::PathBuf;
use tokio;

/// Example demonstrating GraphQL SDK generation for multiple languages
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Generating GraphQL SDKs for E-commerce API");

    let schema_path = PathBuf::from("examples/petstore.schema");
    
    // Generate Java SDK
    generate_java_graphql_sdk(&schema_path).await?;
    
    // Generate TypeScript SDK
    generate_typescript_graphql_sdk(&schema_path).await?;
    
    // Generate Python SDK
    generate_python_graphql_sdk(&schema_path).await?;
    
    println!("✅ All GraphQL SDKs generated successfully!");
    Ok(())
}

async fn generate_java_graphql_sdk(schema_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let config = GeneratorConfig {
        target_language: TargetLanguage::Java,
        output_config: OutputConfig {
            package_name: Some("com.example.ecommerce".to_string()),
            version: "1.0.0".to_string(),
            author: Some("SDK Generator".to_string()),
            license: Some("MIT".to_string()),
            include_tests: true,
            include_docs: true,
            format_code: true,
        },
        features: Features {
            retry: Default::default(),
            telemetry: Default::default(),
            caching: Default::default(),
            async_support: true,
        },
        template_overrides: None,
    };
    
    let generator = SdkGenerator::new(config)?;
    let output_path = PathBuf::from("generated/ecommerce-java-graphql-sdk");
    
    generator.generate_from_graphql(schema_path.clone(), output_path).await?;
    println!("✅ Java GraphQL SDK generated");
    Ok(())
}
```

### gRPC SDK Generation

```rust
// examples/generate_grpc_sdks.rs
use client_sdk_generator::core::config::{GeneratorConfig, TargetLanguage, OutputConfig, Features};
use client_sdk_generator::core::generator::SdkGenerator;
use std::path::PathBuf;
use tokio;

/// Example demonstrating gRPC SDK generation for multiple languages
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Generating gRPC SDKs for User Service API");

    let proto_path = PathBuf::from("examples/petstore.proto");
    
    // Generate Go SDK (most common for gRPC)
    generate_go_grpc_sdk(&proto_path).await?;
    
    // Generate Java SDK
    generate_java_grpc_sdk(&proto_path).await?;
    
    // Generate Rust SDK
    generate_rust_grpc_sdk(&proto_path).await?;
    
    println!("✅ All gRPC SDKs generated successfully!");
    Ok(())
}

async fn generate_go_grpc_sdk(proto_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let config = GeneratorConfig {
        target_language: TargetLanguage::Go,
        output_config: OutputConfig {
            package_name: Some("github.com/example/userservice-client".to_string()),
            version: "1.0.0".to_string(),
            author: Some("SDK Generator".to_string()),
            license: Some("MIT".to_string()),
            include_tests: true,
            include_docs: true,
            format_code: true,
        },
        features: Features {
            retry: Default::default(),
            telemetry: Default::default(),
            caching: Default::default(),
            async_support: true,
        },
        template_overrides: None,
    };
    
    let generator = SdkGenerator::new(config)?;
    let output_path = PathBuf::from("generated/userservice-go-grpc-sdk");
    
    generator.generate_from_grpc(proto_path.clone(), output_path).await?;
    println!("✅ Go gRPC SDK generated");
    Ok(())
}
```

### Running the Examples

```bash
# Run OpenAPI example
cargo run --example generate_openapi_sdks

# Run GraphQL example
cargo run --example generate_graphql_sdks

# Run gRPC example
cargo run --example generate_grpc_sdks
```

## Generated SDK Features

### Java SDK Features

- **HTTP Client**: Built on Java 11+ HttpClient
- **JSON Serialization**: Jackson-based with configurable deserialization
- **Retry Logic**: Failsafe library integration
- **Telemetry**: Micrometer metrics with Prometheus support
- **Caching**: Caffeine cache implementation
- **Async Support**: CompletableFuture-based async methods
- **Type Safety**: Strong typing with generated POJOs
- **Builder Pattern**: Fluent API for complex requests

Example generated Java code:
```java
// Create client
ClientConfig config = ClientConfig.builder()
    .baseUrl("https://api.petstore.com/v1")
    .connectTimeout(Duration.ofSeconds(10))
    .requestTimeout(Duration.ofSeconds(30))
    .build();

PetStoreClient client = new PetStoreClient(config);

// Synchronous call
List<Pet> pets = client.listPets(10, "cats");

// Asynchronous call (if enabled)
CompletableFuture<Pet> petFuture = client.listPetsAsync(10, "cats");
```

### Python SDK Features

- **HTTP Client**: httpx-based with async support
- **JSON Serialization**: Pydantic models for validation
- **Retry Logic**: Tenacity library integration  
- **Telemetry**: Prometheus client support
- **Caching**: Memory-based TTL cache
- **Async Support**: Native async/await
- **Type Hints**: Full typing support
- **Dataclasses**: Clean, Pythonic data models

Example generated Python code:
```python
from client_sdk import PetStoreClient, ClientConfig

# Create client
config = ClientConfig(
    base_url="https://api.petstore.com/v1",
    timeout=30.0
)
client = PetStoreClient(config)

# Synchronous call
pets = client.list_pets(limit=10, tag="cats")

# Asynchronous call
async def get_pets():
    async with PetStoreAsyncClient(config) as client:
        pets = await client.list_pets(limit=10, tag="cats")
        return pets
```

### Rust SDK Features

- **HTTP Client**: reqwest with native async
- **JSON Serialization**: serde with derive macros
- **Retry Logic**: tokio-retry integration
- **Telemetry**: Prometheus metrics
- **Caching**: moka async cache
- **Async Support**: Native async/await with tokio
- **Type Safety**: Strong Rust typing
- **Error Handling**: Result types with custom errors

Example generated Rust code:
```rust
use client_sdk::{PetStoreClient, ClientConfig, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = ClientConfig::builder()
        .base_url("https://api.petstore.com/v1")
        .timeout(std::time::Duration::from_secs(30))
        .build();
        
    let client = PetStoreClient::new(config);
    
    // All calls are async in Rust
    let pets = client.list_pets(Some(10), Some("cats")).await?;
    
    Ok(())
}
```

## Architecture

### Core Components

1. **Parsers**: Convert input specifications to common intermediate representation
   - `OpenApiParser`: Handles OpenAPI 3.0+ specifications
   - `GraphQLParser`: Processes GraphQL schema definitions  
   - `GrpcParser`: Parses Protocol Buffer definitions

2. **Code Generator**: Orchestrates the generation process
   - Template loading and rendering
   - Feature code injection
   - File organization and output

3. **Template Engine**: Handlebars-based templating system
   - Language-specific templates
   - Custom helpers for naming conventions
   - Type mapping and code generation utilities

4. **Feature Generators**: Create language-specific implementations
   - Retry logic with various backoff strategies
   - Telemetry integration 
   - Caching mechanisms
   - Async patterns

### Intermediate Representation

All input formats are converted to a common `ApiSpec` structure:

```rust
pub struct ApiSpec {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub base_url: Option<String>,
    pub operations: Vec<Operation>,
    pub types: Vec<TypeDefinition>,
    pub authentication: Option<AuthenticationSpec>,
}
```

This allows the same generation logic to work across different input formats.

## Extending the Generator

### Adding a New Language

1. Create language-specific templates in `templates/{language}/`
2. Implement type mapping in the template helpers
3. Add feature code generators in `src/core/features.rs`
4. Update the CLI to recognize the new language

### Adding a New Input Format

1. Implement a parser in `src/parsers/`
2. Convert the input format to `ApiSpec`
3. Add CLI command and integration
4. Create example specifications

### Adding New Features

1. Extend `FeatureConfig` in `src/core/config.rs`
2. Implement feature code generation in `src/core/features.rs`
3. Update templates to conditionally include feature code
4. Add configuration options

## Examples

See the `examples/` directory for:
- `petstore-openapi.yaml` - Sample OpenAPI 3.0 specification
- `ecommerce-graphql.schema` - Comprehensive GraphQL schema
- `user-service.proto` - gRPC service definition
- `generate_openapi_sdks.rs` - OpenAPI SDK generation example
- `generate_graphql_sdks.rs` - GraphQL SDK generation example  
- `generate_grpc_sdks.rs` - gRPC SDK generation example
- Generated SDK examples in the `generated/` directory

## Testing

Run the test suite:
```bash
cargo test
```

Test with different input formats:
Test with CLI:
```bash
# Test OpenAPI generation
cargo run -- openapi --spec examples/petstore-openapi.yaml --language java --output /tmp/test-java

# Test GraphQL generation  
cargo run -- graphql --schema examples/petstore.schema --language python --output /tmp/test-python

# Test gRPC generation
cargo run -- grpc --proto examples/petstore.proto --language rust --output /tmp/test-rust

# Test with configuration
cargo run -- openapi --spec examples/petstore-openapi.yaml --language java --output /tmp/test-java --config config.yaml
```

Test with example scripts:
```bash
# Run the example scripts
cargo run --example generate_openapi_sdks
cargo run --example generate_graphql_sdks
cargo run --example generate_grpc_sdks
```

Run unit tests:
```bash
# Run all tests
cargo test

# Run specific tests
cargo test test_openapi_generation
cargo test test_graphql_generation
cargo test test_grpc_generation
```

## Development

### Prerequisites

- Rust 1.70+
- For testing generated SDKs:
  - Java 11+ with Maven
  - Python 3.8+ with pip
  - Rust 1.70+

### Project Structure

```
client-sdk-generator/
├── src/
│   ├── core/           # Core generation logic
│   ├── parsers/        # Input format parsers
│   ├── generators/     # Language-specific generators
│   └── main.rs         # CLI entry point
├── templates/          # Handlebars templates
│   ├── java/
│   ├── python/
│   └── rust/
├── examples/           # Example specifications and generated code
└── docs/              # Additional documentation
```

### Building

```bash
# Development build
cargo build

# Release build
cargo build --release

# With specific features
cargo build --features "telemetry,caching"
```

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass (`cargo test`)
6. Commit your changes (`git commit -m 'Add amazing feature'`)
7. Push to the branch (`git push origin feature/amazing-feature`)
8. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [OpenAPI Initiative](https://www.openapis.org/) for the OpenAPI specification
- [GraphQL Foundation](https://graphql.org/) for GraphQL
- [Protocol Buffers](https://developers.google.com/protocol-buffers) team at Google
- All the open source libraries that make this possible