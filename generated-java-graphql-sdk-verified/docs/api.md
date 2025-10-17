# GraphQL API API Documentation

Generated from GraphQL schema

**Version:** 1.0.0

## Base URL

The base URL for all API operations is configured when creating the client:

```java
ClientConfig config = new ClientConfig("");
```

## Authentication

No authentication is required for this API.

## Data Types

### GraphQLRequest

GraphQL request object

**Type:** Object

**Properties:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `query` | String | ✅ | GraphQL query string |

**Type:** Array of String

**Type:** String

---

### GraphQLResponse

GraphQL response object

**Type:** Object

**Properties:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `data` | String | ❌ | Response data |

**Type:** Array of String

**Type:** String

---


## Operations

### sample

Sample GraphQL query

**HTTP Method:** `Post`  
**Path:** `/graphql`


#### Parameters

**query** (Query)
- **Type:** String
- **Required:** ✅ Yes
- **Description:** GraphQL query string


#### Request Body

**Type:** String


#### Responses

**** - GraphQL response

**Response Type:** String



#### Example Usage

**Synchronous:**
```java
GraphqlApiClient client = new GraphqlApiClient(config);

// Set parameters
String query = ...; // GraphQL query string

// Create request body
String requestBody = ...;

try {
    String result = client.sample(query, requestBody);
    System.out.println("Response: " + result);
} catch (GraphqlApiException e) {
    System.err.println("Error: " + e.getMessage());
}
```


---


## Error Handling

All API errors are wrapped in `GraphqlApiException`. This exception provides:

- **Message**: A human-readable error description
- **Cause**: The underlying exception (if any)
- **HTTP Status Code**: Available through the error message

### Common Error Responses

| Status Code | Description |
|-------------|-------------|
| 400 | Bad Request - Invalid parameters or request body |
| 401 | Unauthorized - Missing or invalid authentication |
| 403 | Forbidden - Insufficient permissions |
| 404 | Not Found - Resource does not exist |
| 429 | Too Many Requests - Rate limit exceeded |
| 500 | Internal Server Error - Server-side error |

### Example Error Handling

```java
try {
    // API call
    String result = client.sample(...);
} catch (GraphqlApiException e) {
    if (e.getMessage().contains("HTTP 401")) {
        System.err.println("Authentication failed - check your credentials");
    } else if (e.getMessage().contains("HTTP 429")) {
        System.err.println("Rate limit exceeded - please retry after some time");
    } else {
        System.err.println("API error: " + e.getMessage());
    }
}
```

## Client Configuration

The `ClientConfig` class provides various configuration options:

### Timeouts

```java
config.setConnectTimeoutSeconds(30)    // Connection timeout
      .setRequestTimeoutSeconds(60);   // Request timeout
```

### User Agent

```java
config.setUserAgent("MyApplication/1.0");
```

### Retry Configuration

```java
config.setRetryEnabled(true)      // Enable automatic retries
      .setMaxRetries(3)           // Maximum number of retries
      .setRetryDelayMs(1000);     // Delay between retries in milliseconds
```

Retries are automatically performed for:
- Network connectivity issues
- HTTP 5xx server errors
- HTTP 429 (rate limit) responses


### Telemetry Configuration

```java
config.setTelemetryEnabled(true)                           // Enable metrics collection
      .setMetricsEndpoint("https://metrics.example.com");  // Metrics endpoint
```

Collected metrics include:
- Request duration
- Request count by operation
- Error count by operation
- Success/failure rates

---

*This documentation was generated automatically from the OpenAPI specification.*