# Pet Store API Java Client SDK

A sample API that uses a pet store as an example

This is a generated Java client SDK for the Pet Store API API (version 1.0.0). 

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
import com.example.client.PetStoreApiClient;

// Create a configuration
ClientConfig config = new ClientConfig("https://api.example.com")
    .setConnectTimeoutSeconds(30)
    .setRequestTimeoutSeconds(60);

// Create the client
PetStoreApiClient client = new PetStoreApiClient(config);

// Example usage: Returns a list of all pets in the store
// String limit = ...; // Maximum number of pets to return
// String tag = ...; // Filter pets by tag
String response = client.listpets(limit, tag);
```

### Async Usage

```java
import com.example.client.PetStoreApiAsyncClient;
import java.util.concurrent.CompletableFuture;

// Create async client
PetStoreApiAsyncClient asyncClient = new PetStoreApiAsyncClient(config);

// Async example: Returns a list of all pets in the store
CompletableFuture<String> future = asyncClient.listpetsAsync(limit, tag);

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

### listPets

Returns a list of all pets in the store

**Method:** `Get /pets`

**Parameters:**
- `limit` (String) - Maximum number of pets to return
- `tag` (String) - Filter pets by tag


**Response:** String


---

### createPet

Creates a new pet in the store

**Method:** `Post /pets`



**Response:** void


---

### getPet

Returns a single pet by its ID

**Method:** `Get /pets/{petId}`

**Parameters:**
- `petid` (String) *required* - ID of the pet to retrieve


**Response:** void


---

### updatePet

Updates an existing pet

**Method:** `Put /pets/{petId}`

**Parameters:**
- `petid` (String) *required* - ID of the pet to update


**Response:** void


---

### deletePet

Deletes a pet from the store

**Method:** `Delete /pets/{petId}`

**Parameters:**
- `petid` (String) *required* - ID of the pet to delete


**Response:** void


---


## Error Handling

The client throws `PetStoreApiException` for API errors:

```java
try {
    String result = client.listpets(...);
} catch (PetStoreApiException e) {
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