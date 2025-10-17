# GraphQL API Java Client SDK

Generated from GraphQL schema

This is a generated Java client SDK for the GraphQL API API (version 1.0.0). 

## Features

- ✅ Async support with CompletableFuture
- ✅ Automatic retry with configurable backoff
- ✅ Built-in metrics and telemetry
- ✅ Jackson JSON serialization
- ✅ Comprehensive error handling
- ✅ Unit tests included

## Requirements

- Java 21 or higher
- Maven 3.6 or higher

## Installation

Add this dependency to your `pom.xml`:

```xml
<dependency>
    <groupId>com.example.client</groupId>
    <artifactId></artifactId>
    <version>1.0.0</version>
</dependency>
```

## Quick Start

### Basic Usage

```java
import com.example.client.ClientConfig;
import com.example.client.GraphqlApiClient;

// Create a configuration
ClientConfig config = new ClientConfig("https://api.example.com")
    .setConnectTimeoutSeconds(30)
    .setRequestTimeoutSeconds(60);

// Create the client
GraphqlApiClient client = new GraphqlApiClient(config);

// Example usage: Sample GraphQL query
// String query = ...; // GraphQL query string
// String request = ...; // Request body
String response = client.sample(query, request);
```

### Async Usage

```java
import com.example.client.GraphqlApiAsyncClient;
import java.util.concurrent.CompletableFuture;

// Create async client
GraphqlApiAsyncClient asyncClient = new GraphqlApiAsyncClient(config);

// Async example: Sample GraphQL query
CompletableFuture<String> future = asyncClient.sampleAsync(query, request);

future.thenAccept(result -> {
    // Handle the result
    System.out.println("Response received: " + result);
}).exceptionally(throwable -> {
    // Handle errors
    System.err.println("Error occurred: " + throwable.getMessage());
    return null;
});
```

### Configuration Options

```java
ClientConfig config = new ClientConfig("https://api.example.com")
    .setConnectTimeoutSeconds(30)      // Connection timeout
    .setRequestTimeoutSeconds(60)      // Request timeout
    .setUserAgent("MyApp/1.0")        // Custom user agent
    .setApiKey("your-api-key")        // API key authentication
    .setBearerToken("your-token");    // Bearer token authentication

// Retry configuration
config.setRetryEnabled(true)
      .setMaxRetries(3)
      .setRetryDelayMs(1000);


// Telemetry configuration
config.setTelemetryEnabled(true)
      .setMetricsEndpoint("https://metrics.example.com");
```

## API Operations

### sample

Sample GraphQL query

**Method:** `Post /graphql`

**Parameters:**
- `query` (String) *required* - GraphQL query string

**Request Body:** String

**Response:** String


---


## Error Handling

The client throws `GraphqlApiException` for API errors:

```java
try {
    String result = client.sample(...);
} catch (GraphqlApiException e) {
    System.err.println("API Error: " + e.getMessage());
}
```

## Building

```bash
mvn clean compile
```

## Testing

```bash
mvn test
```

## Documentation

For more detailed documentation, see the [API Documentation](api.md).

## License

This project is licensed under the MIT License.

---

*This client SDK was generated automatically. Do not modify it directly.*