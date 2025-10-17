# GraphqlApi API Documentation

This document provides detailed information about the GraphQL API API and how to use the generated Rust SDK.

## Base URL

```
https://api.example.com
```

## Authentication

This API does not require authentication.

## Available Methods

### sample

Sample GraphQL query

**HTTP Method**: `POST`  
**Path**: `/graphql`

#### Parameters

- **query** (Query, required): GraphQL query string
  - Type: `String`


#### Request Body

Type: `String`


#### Responses

- **200**: GraphQL response
  - Type: `String`


#### Usage Example

```rust
let query = /* your String value */;

let result = client.sample(
    query,
).await?;

println!("Result: {:?}", result);
```

---


## Data Types

### Graphqlrequest

GraphQL request object

#### Properties

- **query** (required): GraphQL query string
  - Type: `String`


#### Usage Example

```rust
use com.example.client::Graphqlrequest;

let instance = Graphqlrequest {
    query: /* your String value */,
};

// Or use the builder pattern
let instance = Graphqlrequest::new(
    /* query */ /* your String value */,
);
```

---

### Graphqlresponse

GraphQL response object

#### Properties

- **data**: Response data
  - Type: `String`


#### Usage Example

```rust
use com.example.client::Graphqlresponse;

let instance = Graphqlresponse {
    data: None,
};

// Or use the builder pattern
let instance = Graphqlresponse::new(
)
.with_data(/* your String value */);
```

---


## Error Handling

The SDK provides a comprehensive error type that covers all possible error conditions:

```rust
#[derive(Debug, thiserror::Error)]
pub enum GraphqlApiError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),
    
    #[error("API returned error {0}: {1}")]
    Api(u16, String),
    
    #[error("Failed to parse response: {0}")]
    Parsing(String),
    
    #[error("Invalid URL: {0}")]
    Url(#[from] url::ParseError),
}
```

### Common Error Scenarios

1. **Network Errors**: Connection timeouts, DNS resolution failures
2. **HTTP Errors**: 4xx and 5xx status codes from the server
3. **Parsing Errors**: Invalid JSON in response bodies
4. **URL Errors**: Malformed URLs or path parameters

## Best Practices

1. **Error Handling**: Always handle errors appropriately in your application
2. **Timeouts**: Set reasonable timeout values for your use case
3. **Retries**: Use the built-in retry functionality for transient failures
4. **Caching**: Enable response caching for read-heavy workloads
5. **Telemetry**: Enable telemetry in production for monitoring and debugging

## Support

For issues with this SDK, please check the generated code or contact the API provider.