# UserService gRPC API gRPC API Documentation

Generated from UserService gRPC proto files

Go gRPC client SDK for UserService gRPC API.

**Version:** 1.0.0

## Installation

```bash
go mod init your-project
go get github.com/example/client
```

## Basic Usage

```go
package main

import (
    "context"
    "fmt"
    "log"
    "github.com/example/client"
)

func main() {
    config := client.ClientConfig{
        Host:   "api.example.com",
        Port:   443,
        UseTLS: true,
    }
    
    client, err := client.NewGrpcClient(config)
    if err != nil {
        log.Fatal(err)
    }
    defer client.Close()
    
    ctx := context.Background()
    
    // Example service call
    request := &client.ExampleRequest{
        Message: "Hello, gRPC!",
    }
    
    response, err := client.ExampleMethod(ctx, request)
    if err != nil {
        log.Fatal(err)
    }
    
    fmt.Printf("Response: %+v\n", response)
}
```

## Authentication

No authentication is required for this API.

## Services and Methods


## Message Types


## Error Handling

The gRPC client provides comprehensive error handling:

```go
import (
    "context"
    "fmt"
    "google.golang.org/grpc/codes"
    "google.golang.org/grpc/status"
    "github.com/example/client"
)

func handleGrpcErrors() {
    client, err := client.NewGrpcClient(config)
    if err != nil {
        log.Fatal(err)
    }
    defer client.Close()
    
    request := &client.ExampleRequest{}
    response, err := client.ExampleMethod(ctx, request)
    
    if err != nil {
        // Check for gRPC-specific errors
        if st, ok := status.FromError(err); ok {
            switch st.Code() {
            case codes.NotFound:
                fmt.Println("Resource not found")
            case codes.PermissionDenied:
                fmt.Println("Permission denied")
            case codes.Unauthenticated:
                fmt.Println("Authentication failed")
            case codes.DeadlineExceeded:
                fmt.Println("Request timed out")
            case codes.Unavailable:
                fmt.Println("Service unavailable")
            default:
                fmt.Printf("gRPC error: %s (code: %s)\n", st.Message(), st.Code())
            }
            
            // Handle error details if any
            for _, detail := range st.Details() {
                fmt.Printf("Error detail: %+v\n", detail)
            }
        } else {
            // Handle other errors (connection, etc.)
            fmt.Printf("Client error: %v\n", err)
        }
    }
}
```

### Common gRPC Status Codes

| Code | Description |
|------|-------------|
| `OK` | Success |
| `CANCELLED` | Operation was cancelled |
| `UNKNOWN` | Unknown error |
| `INVALID_ARGUMENT` | Invalid arguments provided |
| `DEADLINE_EXCEEDED` | Request timeout |
| `NOT_FOUND` | Resource not found |
| `ALREADY_EXISTS` | Resource already exists |
| `PERMISSION_DENIED` | Insufficient permissions |
| `UNAUTHENTICATED` | Authentication failed |
| `RESOURCE_EXHAUSTED` | Resource exhausted (rate limiting) |
| `FAILED_PRECONDITION` | Precondition failed |
| `ABORTED` | Operation aborted |
| `OUT_OF_RANGE` | Value out of range |
| `UNIMPLEMENTED` | Method not implemented |
| `INTERNAL` | Internal server error |
| `UNAVAILABLE` | Service unavailable |
| `DATA_LOSS` | Data loss or corruption |

### Retry Logic

```go
import (
    "time"
    "google.golang.org/grpc/codes"
    "google.golang.org/grpc/status"
)

func retryableCall() {
    var response *client.ExampleResponse
    var err error
    
    maxRetries := 3
    baseDelay := time.Second
    
    for attempt := 0; attempt <= maxRetries; attempt++ {
        response, err = client.ExampleMethod(ctx, request)
        if err == nil {
            break // Success
        }
        
        // Check if error is retryable
        if st, ok := status.FromError(err); ok {
            switch st.Code() {
            case codes.Unavailable, codes.DeadlineExceeded, codes.ResourceExhausted:
                if attempt < maxRetries {
                    delay := baseDelay * time.Duration(1<<attempt) // Exponential backoff
                    fmt.Printf("Attempt %d failed, retrying in %v...\n", attempt+1, delay)
                    time.Sleep(delay)
                    continue
                }
            }
        }
        
        // Non-retryable error or max retries exceeded
        break
    }
    
    if err != nil {
        log.Fatal(err)
    }
}
```

## Client Configuration

The `ClientConfig` struct provides various configuration options:

### Basic Configuration

```go
config := client.ClientConfig{
    Host:    "api.example.com",
    Port:    443,
    UseTLS:  true,
    Timeout: 30 * time.Second,
}
```

### TLS Configuration

```go
import (
    "crypto/tls"
    "crypto/x509"
)

// Custom TLS configuration
tlsConfig := &tls.Config{
    ServerName: "api.example.com",
    // Add custom CA certificates if needed
    RootCAs: customCertPool,
}

config := client.ClientConfig{
    Host:      "api.example.com",
    Port:      443,
    UseTLS:    true,
    TLSConfig: tlsConfig,
}
```

### Connection Pool Configuration

```go
config := client.ClientConfig{
    Host:               "api.example.com",
    Port:               443,
    UseTLS:             true,
    MaxConnectionIdle:  5 * time.Minute,
    MaxConnectionAge:   30 * time.Minute,
    KeepAliveTime:      30 * time.Second,
    KeepAliveTimeout:   10 * time.Second,
    PermitWithoutStream: true,
}
```

### Retry Configuration

```go
config := client.ClientConfig{
    Host:         "api.example.com",
    Port:         443,
    UseTLS:       true,
    RetryEnabled: true,
    MaxRetries:   3,
    RetryDelay:   1 * time.Second,
    RetryBackoff: 2.0, // Exponential backoff multiplier
}
```

Retries are automatically performed for:
- Network connectivity issues
- gRPC UNAVAILABLE status
- gRPC DEADLINE_EXCEEDED status
- gRPC RESOURCE_EXHAUSTED status

### Telemetry and Metrics

```go
config := client.ClientConfig{
    Host:             "api.example.com",
    Port:             443,
    UseTLS:           true,
    TelemetryEnabled: true,
    MetricsEndpoint:  "https://metrics.example.com",
}
```

Collected metrics include:
- Request duration by method
- Request count by method
- Error count by method and status code
- Success/failure rates
- Connection pool statistics

## Context and Cancellation

All gRPC operations support Go's context for cancellation and timeouts:

```go
import (
    "context"
    "time"
)

// With timeout
ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
defer cancel()

response, err := client.ExampleMethod(ctx, request)
if err != nil {
    if ctx.Err() == context.DeadlineExceeded {
        fmt.Println("Request timed out")
    } else {
        fmt.Printf("Request error: %v\n", err)
    }
}

// With cancellation
ctx, cancel = context.WithCancel(context.Background())

// Cancel the context from another goroutine
go func() {
    time.Sleep(5 * time.Second)
    cancel()
}()

stream, err := client.ExampleStreamingMethod(ctx, request)
if err == nil {
    for {
        response, err := stream.Recv()
        if err != nil {
            if ctx.Err() == context.Canceled {
                fmt.Println("Stream cancelled")
            }
            break
        }
        // Process response
    }
}
```

## Connection Management

### Connection Pooling

The client automatically manages connection pooling:

```go
// The client maintains a single connection that can be reused
client, err := client.NewGrpcClient(config)
if err != nil {
    log.Fatal(err)
}

// Reuse the same client for multiple requests
for i := 0; i < 100; i++ {
    response, err := client.ExampleMethod(ctx, request)
    if err != nil {
        log.Printf("Request %d failed: %v", i, err)
        continue
    }
    // Process response
}

// Close when done
client.Close()
```

### Health Checking

```go
// Check connection health
if err := client.HealthCheck(ctx); err != nil {
    fmt.Printf("Health check failed: %v\n", err)
    // Reconnect or handle error
}
```

---

*This documentation was generated automatically from the Protocol Buffer definitions.*