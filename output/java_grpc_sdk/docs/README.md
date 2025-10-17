# UserService gRPC API Java Client SDK

Generated from UserService gRPC proto files

This is a generated Java client SDK for the UserService gRPC API API (version 1.0.0). 

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
import com.example.client.UserserviceGrpcApiClient;

// Create a configuration
ClientConfig config = new ClientConfig("https://api.example.com")
    .setConnectTimeoutSeconds(30)
    .setRequestTimeoutSeconds(60);

// Create the client
UserserviceGrpcApiClient client = new UserserviceGrpcApiClient(config);

// Example usage: Register a new user account
// String request = ...; // Request body
String response = client.registeruser(request);
```

### Async Usage

```java
import com.example.client.UserserviceGrpcApiAsyncClient;
import java.util.concurrent.CompletableFuture;

// Create async client
UserserviceGrpcApiAsyncClient asyncClient = new UserserviceGrpcApiAsyncClient(config);

// Async example: Register a new user account
CompletableFuture<String> future = asyncClient.registeruserAsync(request);

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

### RegisterUser

Register a new user account

**Method:** `Post /userservice.v1.UserService/RegisterUser`


**Request Body:** String

**Response:** String


---

### LoginUser

Authenticate a user and create session

**Method:** `Post /userservice.v1.UserService/LoginUser`


**Request Body:** String

**Response:** String


---

### RefreshToken

Refresh authentication token

**Method:** `Post /userservice.v1.UserService/RefreshToken`


**Request Body:** String

**Response:** String


---

### LogoutUser

Logout user and invalidate session

**Method:** `Post /userservice.v1.UserService/LogoutUser`


**Request Body:** String

**Response:** String


---

### GetUser

Get user profile by ID

**Method:** `Post /userservice.v1.UserService/GetUser`


**Request Body:** String

**Response:** String


---

### GetCurrentUser

Get current authenticated user profile

**Method:** `Post /userservice.v1.UserService/GetCurrentUser`


**Request Body:** String

**Response:** String


---

### UpdateUser

Update user profile information

**Method:** `Post /userservice.v1.UserService/UpdateUser`


**Request Body:** String

**Response:** String


---

### DeleteUser

Delete user account

**Method:** `Post /userservice.v1.UserService/DeleteUser`


**Request Body:** String

**Response:** String


---

### ListUsers

List users with pagination and filtering

**Method:** `Post /userservice.v1.UserService/ListUsers`


**Request Body:** String

**Response:** String


---

### ChangePassword

Change user password

**Method:** `Post /userservice.v1.UserService/ChangePassword`


**Request Body:** String

**Response:** String


---

### ResetPassword

Reset user password

**Method:** `Post /userservice.v1.UserService/ResetPassword`


**Request Body:** String

**Response:** String


---

### SendVerificationEmail

Send email verification

**Method:** `Post /userservice.v1.UserService/SendVerificationEmail`


**Request Body:** String

**Response:** String


---

### VerifyEmail

Verify email address

**Method:** `Post /userservice.v1.UserService/VerifyEmail`


**Request Body:** String

**Response:** String


---

### ListUserSessions

List user active sessions

**Method:** `Post /userservice.v1.UserService/ListUserSessions`


**Request Body:** String

**Response:** String


---

### RevokeSession

Revoke a user session

**Method:** `Post /userservice.v1.UserService/RevokeSession`


**Request Body:** String

**Response:** String


---

### GetUserPreferences

Get user preferences

**Method:** `Post /userservice.v1.UserService/GetUserPreferences`


**Request Body:** String

**Response:** String


---

### UpdateUserPreferences

Update user preferences

**Method:** `Post /userservice.v1.UserService/UpdateUserPreferences`


**Request Body:** String

**Response:** String


---


## Error Handling

The client throws `UserserviceGrpcApiException` for API errors:

```java
try {
    String result = client.registeruser(...);
} catch (UserserviceGrpcApiException e) {
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