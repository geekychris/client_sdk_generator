# UserService gRPC API API Documentation

Generated from UserService gRPC proto files

**Version:** 1.0.0

## Base URL

The base URL for all API operations is configured when creating the client:

```java
ClientConfig config = new ClientConfig("");
```

## Authentication

No authentication is required for this API.

## Data Types

### SampleRequest

Sample gRPC request

**Type:** Object

**Properties:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `message` | String | ✅ | Request message |

**Type:** Array of String

**Type:** String

---

### SampleResponse

Sample gRPC response

**Type:** Object

**Properties:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `result` | String | ✅ | Response result |

**Type:** Array of String

**Type:** String

---


## Operations

### RegisterUser

Register a new user account

**HTTP Method:** `Post`  
**Path:** `/userservice.v1.UserService/RegisterUser`


#### Parameters

This operation takes no parameters.

#### Request Body

**Type:** String


#### Responses

**** - gRPC response

**Response Type:** String



#### Example Usage

**Synchronous:**
```java
UserserviceGrpcApiClient client = new UserserviceGrpcApiClient(config);


// Create request body
String requestBody = ...;

try {
    String result = client.registeruser(requestBody);
    System.out.println("Response: " + result);
} catch (UserserviceGrpcApiException e) {
    System.err.println("Error: " + e.getMessage());
}
```


---

### LoginUser

Authenticate a user and create session

**HTTP Method:** `Post`  
**Path:** `/userservice.v1.UserService/LoginUser`


#### Parameters

This operation takes no parameters.

#### Request Body

**Type:** String


#### Responses

**** - gRPC response

**Response Type:** String



#### Example Usage

**Synchronous:**
```java
UserserviceGrpcApiClient client = new UserserviceGrpcApiClient(config);


// Create request body
String requestBody = ...;

try {
    String result = client.loginuser(requestBody);
    System.out.println("Response: " + result);
} catch (UserserviceGrpcApiException e) {
    System.err.println("Error: " + e.getMessage());
}
```


---

### RefreshToken

Refresh authentication token

**HTTP Method:** `Post`  
**Path:** `/userservice.v1.UserService/RefreshToken`


#### Parameters

This operation takes no parameters.

#### Request Body

**Type:** String


#### Responses

**** - gRPC response

**Response Type:** String



#### Example Usage

**Synchronous:**
```java
UserserviceGrpcApiClient client = new UserserviceGrpcApiClient(config);


// Create request body
String requestBody = ...;

try {
    String result = client.refreshtoken(requestBody);
    System.out.println("Response: " + result);
} catch (UserserviceGrpcApiException e) {
    System.err.println("Error: " + e.getMessage());
}
```


---

### LogoutUser

Logout user and invalidate session

**HTTP Method:** `Post`  
**Path:** `/userservice.v1.UserService/LogoutUser`


#### Parameters

This operation takes no parameters.

#### Request Body

**Type:** String


#### Responses

**** - gRPC response

**Response Type:** String



#### Example Usage

**Synchronous:**
```java
UserserviceGrpcApiClient client = new UserserviceGrpcApiClient(config);


// Create request body
String requestBody = ...;

try {
    String result = client.logoutuser(requestBody);
    System.out.println("Response: " + result);
} catch (UserserviceGrpcApiException e) {
    System.err.println("Error: " + e.getMessage());
}
```


---

### GetUser

Get user profile by ID

**HTTP Method:** `Post`  
**Path:** `/userservice.v1.UserService/GetUser`


#### Parameters

This operation takes no parameters.

#### Request Body

**Type:** String


#### Responses

**** - gRPC response

**Response Type:** String



#### Example Usage

**Synchronous:**
```java
UserserviceGrpcApiClient client = new UserserviceGrpcApiClient(config);


// Create request body
String requestBody = ...;

try {
    String result = client.getuser(requestBody);
    System.out.println("Response: " + result);
} catch (UserserviceGrpcApiException e) {
    System.err.println("Error: " + e.getMessage());
}
```


---

### GetCurrentUser

Get current authenticated user profile

**HTTP Method:** `Post`  
**Path:** `/userservice.v1.UserService/GetCurrentUser`


#### Parameters

This operation takes no parameters.

#### Request Body

**Type:** String


#### Responses

**** - gRPC response

**Response Type:** String



#### Example Usage

**Synchronous:**
```java
UserserviceGrpcApiClient client = new UserserviceGrpcApiClient(config);


// Create request body
String requestBody = ...;

try {
    String result = client.getcurrentuser(requestBody);
    System.out.println("Response: " + result);
} catch (UserserviceGrpcApiException e) {
    System.err.println("Error: " + e.getMessage());
}
```


---

### UpdateUser

Update user profile information

**HTTP Method:** `Post`  
**Path:** `/userservice.v1.UserService/UpdateUser`


#### Parameters

This operation takes no parameters.

#### Request Body

**Type:** String


#### Responses

**** - gRPC response

**Response Type:** String



#### Example Usage

**Synchronous:**
```java
UserserviceGrpcApiClient client = new UserserviceGrpcApiClient(config);


// Create request body
String requestBody = ...;

try {
    String result = client.updateuser(requestBody);
    System.out.println("Response: " + result);
} catch (UserserviceGrpcApiException e) {
    System.err.println("Error: " + e.getMessage());
}
```


---

### DeleteUser

Delete user account

**HTTP Method:** `Post`  
**Path:** `/userservice.v1.UserService/DeleteUser`


#### Parameters

This operation takes no parameters.

#### Request Body

**Type:** String


#### Responses

**** - gRPC response

**Response Type:** String



#### Example Usage

**Synchronous:**
```java
UserserviceGrpcApiClient client = new UserserviceGrpcApiClient(config);


// Create request body
String requestBody = ...;

try {
    String result = client.deleteuser(requestBody);
    System.out.println("Response: " + result);
} catch (UserserviceGrpcApiException e) {
    System.err.println("Error: " + e.getMessage());
}
```


---

### ListUsers

List users with pagination and filtering

**HTTP Method:** `Post`  
**Path:** `/userservice.v1.UserService/ListUsers`


#### Parameters

This operation takes no parameters.

#### Request Body

**Type:** String


#### Responses

**** - gRPC response

**Response Type:** String



#### Example Usage

**Synchronous:**
```java
UserserviceGrpcApiClient client = new UserserviceGrpcApiClient(config);


// Create request body
String requestBody = ...;

try {
    String result = client.listusers(requestBody);
    System.out.println("Response: " + result);
} catch (UserserviceGrpcApiException e) {
    System.err.println("Error: " + e.getMessage());
}
```


---

### ChangePassword

Change user password

**HTTP Method:** `Post`  
**Path:** `/userservice.v1.UserService/ChangePassword`


#### Parameters

This operation takes no parameters.

#### Request Body

**Type:** String


#### Responses

**** - gRPC response

**Response Type:** String



#### Example Usage

**Synchronous:**
```java
UserserviceGrpcApiClient client = new UserserviceGrpcApiClient(config);


// Create request body
String requestBody = ...;

try {
    String result = client.changepassword(requestBody);
    System.out.println("Response: " + result);
} catch (UserserviceGrpcApiException e) {
    System.err.println("Error: " + e.getMessage());
}
```


---

### ResetPassword

Reset user password

**HTTP Method:** `Post`  
**Path:** `/userservice.v1.UserService/ResetPassword`


#### Parameters

This operation takes no parameters.

#### Request Body

**Type:** String


#### Responses

**** - gRPC response

**Response Type:** String



#### Example Usage

**Synchronous:**
```java
UserserviceGrpcApiClient client = new UserserviceGrpcApiClient(config);


// Create request body
String requestBody = ...;

try {
    String result = client.resetpassword(requestBody);
    System.out.println("Response: " + result);
} catch (UserserviceGrpcApiException e) {
    System.err.println("Error: " + e.getMessage());
}
```


---

### SendVerificationEmail

Send email verification

**HTTP Method:** `Post`  
**Path:** `/userservice.v1.UserService/SendVerificationEmail`


#### Parameters

This operation takes no parameters.

#### Request Body

**Type:** String


#### Responses

**** - gRPC response

**Response Type:** String



#### Example Usage

**Synchronous:**
```java
UserserviceGrpcApiClient client = new UserserviceGrpcApiClient(config);


// Create request body
String requestBody = ...;

try {
    String result = client.sendverificationemail(requestBody);
    System.out.println("Response: " + result);
} catch (UserserviceGrpcApiException e) {
    System.err.println("Error: " + e.getMessage());
}
```


---

### VerifyEmail

Verify email address

**HTTP Method:** `Post`  
**Path:** `/userservice.v1.UserService/VerifyEmail`


#### Parameters

This operation takes no parameters.

#### Request Body

**Type:** String


#### Responses

**** - gRPC response

**Response Type:** String



#### Example Usage

**Synchronous:**
```java
UserserviceGrpcApiClient client = new UserserviceGrpcApiClient(config);


// Create request body
String requestBody = ...;

try {
    String result = client.verifyemail(requestBody);
    System.out.println("Response: " + result);
} catch (UserserviceGrpcApiException e) {
    System.err.println("Error: " + e.getMessage());
}
```


---

### ListUserSessions

List user active sessions

**HTTP Method:** `Post`  
**Path:** `/userservice.v1.UserService/ListUserSessions`


#### Parameters

This operation takes no parameters.

#### Request Body

**Type:** String


#### Responses

**** - gRPC response

**Response Type:** String



#### Example Usage

**Synchronous:**
```java
UserserviceGrpcApiClient client = new UserserviceGrpcApiClient(config);


// Create request body
String requestBody = ...;

try {
    String result = client.listusersessions(requestBody);
    System.out.println("Response: " + result);
} catch (UserserviceGrpcApiException e) {
    System.err.println("Error: " + e.getMessage());
}
```


---

### RevokeSession

Revoke a user session

**HTTP Method:** `Post`  
**Path:** `/userservice.v1.UserService/RevokeSession`


#### Parameters

This operation takes no parameters.

#### Request Body

**Type:** String


#### Responses

**** - gRPC response

**Response Type:** String



#### Example Usage

**Synchronous:**
```java
UserserviceGrpcApiClient client = new UserserviceGrpcApiClient(config);


// Create request body
String requestBody = ...;

try {
    String result = client.revokesession(requestBody);
    System.out.println("Response: " + result);
} catch (UserserviceGrpcApiException e) {
    System.err.println("Error: " + e.getMessage());
}
```


---

### GetUserPreferences

Get user preferences

**HTTP Method:** `Post`  
**Path:** `/userservice.v1.UserService/GetUserPreferences`


#### Parameters

This operation takes no parameters.

#### Request Body

**Type:** String


#### Responses

**** - gRPC response

**Response Type:** String



#### Example Usage

**Synchronous:**
```java
UserserviceGrpcApiClient client = new UserserviceGrpcApiClient(config);


// Create request body
String requestBody = ...;

try {
    String result = client.getuserpreferences(requestBody);
    System.out.println("Response: " + result);
} catch (UserserviceGrpcApiException e) {
    System.err.println("Error: " + e.getMessage());
}
```


---

### UpdateUserPreferences

Update user preferences

**HTTP Method:** `Post`  
**Path:** `/userservice.v1.UserService/UpdateUserPreferences`


#### Parameters

This operation takes no parameters.

#### Request Body

**Type:** String


#### Responses

**** - gRPC response

**Response Type:** String



#### Example Usage

**Synchronous:**
```java
UserserviceGrpcApiClient client = new UserserviceGrpcApiClient(config);


// Create request body
String requestBody = ...;

try {
    String result = client.updateuserpreferences(requestBody);
    System.out.println("Response: " + result);
} catch (UserserviceGrpcApiException e) {
    System.err.println("Error: " + e.getMessage());
}
```


---


## Error Handling

All API errors are wrapped in `UserserviceGrpcApiException`. This exception provides:

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
    String result = client.registeruser(...);
} catch (UserserviceGrpcApiException e) {
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