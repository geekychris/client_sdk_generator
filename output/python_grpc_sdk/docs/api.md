# com.example.client gRPC API Documentation

Generated gRPC API documentation for com.example.client.

## Client

The main gRPC client class provides methods for interacting with the API.

```python
from com.example.client import GrpcClient, ClientConfig

config = ClientConfig(
    host="api.example.com",
    port=443,
    use_tls=True
)
client = GrpcClient(config)
```

## Service Methods


## Message Types


## Error Handling

The gRPC client provides comprehensive error handling:

```python
import grpc
from com.example.client import GrpcClient, GrpcError

try:
    response = client.some_method(request)
except grpc.RpcError as e:
    if e.code() == grpc.StatusCode.NOT_FOUND:
        print("Resource not found")
    elif e.code() == grpc.StatusCode.PERMISSION_DENIED:
        print("Permission denied")
    else:
        print(f"gRPC error: {e.details()}")
except GrpcError as e:
    print(f"Client error: {e}")
```

## Authentication

This API does not require authentication.

## Configuration Options

- `host`: gRPC server hostname
- `port`: gRPC server port (default: 443)
- `use_tls`: Enable TLS/SSL (default: True)
- `timeout_seconds`: Request timeout (default: 30)
- `max_message_length`: Maximum message size in bytes
- `keepalive_time_ms`: Keepalive time in milliseconds
- `keepalive_timeout_ms`: Keepalive timeout in milliseconds

```python
config = ClientConfig(
    host="api.example.com",
    port=8443,
    use_tls=True,
    timeout_seconds=60,
    max_message_length=1024*1024*4,  # 4MB
    keepalive_time_ms=30000,
    keepalive_timeout_ms=10000
)
```