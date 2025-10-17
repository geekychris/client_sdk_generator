# GraphqlApi Rust SDK

Auto-generated Rust SDK for GraphQL API.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
com.example.client = "1.0.0"
```

## Usage

```rust
use com.example.client::GraphqlApiClient;
use com.example.client::ClientConfig;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClientConfig::new("https://api.example.com");
    let client = GraphqlApiClient::new(config)?;

    // Use the client to make API calls
    // let result = client.some_method().await?;

    Ok(())
}
```

## Configuration

The client can be configured with various options:

- `base_url`: The base URL for the API
- `timeout_seconds`: Request timeout in seconds
- `api_key`: Optional API key for authentication
- `default_headers`: Custom headers to include with requests

### Retry Configuration

The client supports automatic retries with exponential backoff:

```rust
use com.example.client::RetryConfig;

let mut config = ClientConfig::new("https://api.example.com");
config.retry_config.max_attempts = 5;
config.retry_config.initial_delay_ms = 200;
```


### Telemetry

The client supports telemetry and metrics collection:

```rust
use com.example.client::TelemetryConfig;

let mut config = ClientConfig::new("https://api.example.com");
config.telemetry_config.enabled = true;
config.telemetry_config.service_name = "my-service".to_string();
```

## Error Handling

The SDK provides comprehensive error handling:

```rust
use com.example.client::GraphqlApiError;

match client.some_method().await {
    Ok(result) => println!("Success: {:?}", result),
    Err(GraphqlApiError::Http(err)) => eprintln!("HTTP error: {}", err),
    Err(GraphqlApiError::Api(status, msg)) => eprintln!("API error {}: {}", status, msg),
    Err(GraphqlApiError::Parsing(err)) => eprintln!("Parse error: {}", err),
    Err(GraphqlApiError::Url(err)) => eprintln!("URL error: {}", err),
}
```

## API Documentation

For detailed API documentation, see [api.md](./api.md).

## License

See the project license for details.
