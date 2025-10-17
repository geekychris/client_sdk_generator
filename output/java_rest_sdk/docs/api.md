# Pet Store API API Documentation

A sample API that uses a pet store as an example

**Version:** 1.0.0

## Base URL

The base URL for all API operations is configured when creating the client:

```java
ClientConfig config = new ClientConfig("");
```

## Authentication

No authentication is required for this API.

## Data Types

### Pet


**Type:** Object

**Properties:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `id` | String | ✅ | Unique identifier for the pet |
| `name` | String | ✅ | Name of the pet |
| `tag` | String | ❌ | Tag associated with the pet |
| `status` | String | ❌ | Pet status in the store |
| `photoUrls` | String | ❌ | URLs of pet photos |

**Type:** Array of String

**Type:** String

---

### NewPet


**Type:** Object

**Properties:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `name` | String | ✅ | Name of the pet |
| `tag` | String | ❌ | Tag associated with the pet |
| `photoUrls` | String | ❌ | URLs of pet photos |

**Type:** Array of String

**Type:** String

---

### Category


**Type:** Object

**Properties:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `id` | String | ❌ | Category identifier |
| `name` | String | ❌ | Category name |

**Type:** Array of String

**Type:** String

---

### Error


**Type:** Object

**Properties:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `code` | String | ✅ | Error code |
| `message` | String | ✅ | Error message |
| `details` | String | ❌ | Additional error details |

**Type:** Array of String

**Type:** String

---


## Operations

### listPets

Returns a list of all pets in the store

**HTTP Method:** `Get`  
**Path:** `/pets`


#### Parameters

**limit** (Query)
- **Type:** String
- **Required:** ❌ No
- **Description:** Maximum number of pets to return

**tag** (Query)
- **Type:** String
- **Required:** ❌ No
- **Description:** Filter pets by tag



#### Responses

**** - A list of pets

**Response Type:** String


**** - Bad request

**Response Type:** No content



#### Example Usage

**Synchronous:**
```java
PetStoreApiClient client = new PetStoreApiClient(config);

// Set parameters
String limit = ...; // Maximum number of pets to return
String tag = ...; // Filter pets by tag


try {
    String result = client.listpets(limit, tag);
    System.out.println("Response: " + result);
} catch (PetStoreApiException e) {
    System.err.println("Error: " + e.getMessage());
}
```


---

### createPet

Creates a new pet in the store

**HTTP Method:** `Post`  
**Path:** `/pets`


#### Parameters

This operation takes no parameters.


#### Responses

**** - Pet created successfully

**Response Type:** No content


**** - Invalid input

**Response Type:** No content



#### Example Usage

**Synchronous:**
```java
PetStoreApiClient client = new PetStoreApiClient(config);



try {
    client.createpet();
    System.out.println("Operation completed successfully");
} catch (PetStoreApiException e) {
    System.err.println("Error: " + e.getMessage());
}
```


---

### getPet

Returns a single pet by its ID

**HTTP Method:** `Get`  
**Path:** `/pets/{petId}`


#### Parameters

**petId** (Path)
- **Type:** String
- **Required:** ✅ Yes
- **Description:** ID of the pet to retrieve



#### Responses

**** - Pet details

**Response Type:** No content


**** - Pet not found

**Response Type:** No content



#### Example Usage

**Synchronous:**
```java
PetStoreApiClient client = new PetStoreApiClient(config);

// Set parameters
String petid = ...; // ID of the pet to retrieve


try {
    client.getpet(petid);
    System.out.println("Operation completed successfully");
} catch (PetStoreApiException e) {
    System.err.println("Error: " + e.getMessage());
}
```


---

### updatePet

Updates an existing pet

**HTTP Method:** `Put`  
**Path:** `/pets/{petId}`


#### Parameters

**petId** (Path)
- **Type:** String
- **Required:** ✅ Yes
- **Description:** ID of the pet to update



#### Responses

**** - Pet updated successfully

**Response Type:** No content


**** - Pet not found

**Response Type:** No content



#### Example Usage

**Synchronous:**
```java
PetStoreApiClient client = new PetStoreApiClient(config);

// Set parameters
String petid = ...; // ID of the pet to update


try {
    client.updatepet(petid);
    System.out.println("Operation completed successfully");
} catch (PetStoreApiException e) {
    System.err.println("Error: " + e.getMessage());
}
```


---

### deletePet

Deletes a pet from the store

**HTTP Method:** `Delete`  
**Path:** `/pets/{petId}`


#### Parameters

**petId** (Path)
- **Type:** String
- **Required:** ✅ Yes
- **Description:** ID of the pet to delete



#### Responses

**** - Pet deleted successfully

**Response Type:** No content


**** - Pet not found

**Response Type:** No content



#### Example Usage

**Synchronous:**
```java
PetStoreApiClient client = new PetStoreApiClient(config);

// Set parameters
String petid = ...; // ID of the pet to delete


try {
    client.deletepet(petid);
    System.out.println("Operation completed successfully");
} catch (PetStoreApiException e) {
    System.err.println("Error: " + e.getMessage());
}
```


---


## Error Handling

All API errors are wrapped in `PetStoreApiException`. This exception provides:

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
    String result = client.listpets(...);
} catch (PetStoreApiException e) {
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