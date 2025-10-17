# com.example.client GraphQL API Documentation

Generated from GraphQL schema

Python GraphQL client SDK for com.example.client.

**Version:** 1.0.0

## Base URL

The GraphQL endpoint URL is configured when creating the client:

```python
from com.example.client import GraphQLClient, ClientConfig

config = ClientConfig(
    base_url="/graphql",
    api_key="your-api-key"
)
client = GraphQLClient(config)
```

## Authentication

No authentication is required for this API.

## GraphQL Operations

### Queries

Execute GraphQL queries using the `query` method:

```python
from com.example.client import GraphQLClient

# Simple query
query = """
query {
    users {
        id
        name
        email
    }
}
"""
result = client.query(query)

# Query with variables
query = """
query GetUser($id: ID!) {
    user(id: $id) {
        id
        name
        email
        profile {
            bio
            avatar_url
        }
    }
}
"""
variables = {"id": "123"}
result = client.query(query, variables=variables)
```

### Mutations

Execute GraphQL mutations using the `mutate` method:

```python
# Create mutation
mutation = """
mutation CreateUser($input: CreateUserInput!) {
    createUser(input: $input) {
        id
        name
        email
    }
}
"""
variables = {
    "input": {
        "name": "John Doe",
        "email": "john@example.com"
    }
}
result = client.mutate(mutation, variables=variables)

# Update mutation
mutation = """
mutation UpdateUser($id: ID!, $input: UpdateUserInput!) {
    updateUser(id: $id, input: $input) {
        id
        name
        email
        updated_at
    }
}
"""
variables = {
    "id": "123",
    "input": {
        "name": "John Smith"
    }
}
result = client.mutate(mutation, variables=variables)
```

### Subscriptions

Execute GraphQL subscriptions using the `subscribe` method:

```python
# Subscribe to real-time updates
subscription = """
subscription UserUpdates {
    userUpdates {
        id
        name
        status
        last_seen
    }
}
"""

for update in client.subscribe(subscription):
    print(f"User updated: {update}")

# Subscription with variables
subscription = """
subscription NotificationsByUser($user_id: ID!) {
    notifications(user_id: $user_id) {
        id
        message
        created_at
    }
}
"""
variables = {"user_id": "123"}

for notification in client.subscribe(subscription, variables=variables):
    print(f"New notification: {notification}")
```

## Data Types

### GraphQLRequest

GraphQL request object

**Type:** Object

**Fields:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `query` | String | ✅ | GraphQL query string |

**Type:** Enum

**Allowed values:** 

---

### GraphQLResponse

GraphQL response object

**Type:** Object

**Fields:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `data` | String | ❌ | Response data |

**Type:** Enum

**Allowed values:** 

---


## Error Handling

The GraphQL client provides comprehensive error handling:

```python
from com.example.client import GraphQLClient, GraphQLError

try:
    result = client.query("{ user { id name } }")
    print(result.data)
except GraphQLError as e:
    # GraphQL-specific errors
    print(f"GraphQL error: {e.message}")
    if e.errors:
        for error in e.errors:
            print(f"  - {error['message']}")
    if e.extensions:
        print(f"Extensions: {e.extensions}")
except Exception as e:
    # Other errors (network, authentication, etc.)
    print(f"Client error: {e}")
```

### Common Error Types

| Error Type | Description |
|------------|-------------|
| `ValidationError` | Query validation failed |
| `ExecutionError` | Query execution failed |
| `AuthenticationError` | Authentication failed |
| `NetworkError` | Network connectivity issues |
| `TimeoutError` | Request timeout |

### Example Error Handling

```python
from com.example.client import GraphQLClient, GraphQLError, ValidationError, AuthenticationError

try:
    result = client.query("{ users { id name } }")
except ValidationError as e:
    print(f"Query validation error: {e.message}")
except AuthenticationError as e:
    print(f"Authentication error: {e.message}")
    # Re-authenticate or refresh token
except GraphQLError as e:
    if "rate limit" in e.message.lower():
        print("Rate limit exceeded - please wait before retrying")
    else:
        print(f"GraphQL error: {e.message}")
except Exception as e:
    print(f"Unexpected error: {e}")
```

## Client Configuration

The `ClientConfig` class provides various configuration options:

### Basic Configuration

```python
from com.example.client import ClientConfig

config = ClientConfig(
    base_url="https://api.example.com/graphql",
    api_key="your-api-key",
    timeout=30.0,  # Request timeout in seconds
    retries=3,     # Number of retries for failed requests
)
```

### Headers

```python
config = ClientConfig(
    base_url="https://api.example.com/graphql",
    headers={
        "User-Agent": "MyApp/1.0",
        "Accept-Language": "en-US"
    }
)
```

### Environment Variables

You can also configure the client using environment variables:

```bash
export GRAPHQL_URL="https://api.example.com/graphql"
export API_KEY="your-api-key"
export API_TIMEOUT="30.0"
```

```python
# Load configuration from environment
config = ClientConfig.from_env()
client = GraphQLClient(config)
```

### Retry Configuration

```python
config = ClientConfig(
    base_url="https://api.example.com/graphql",
    retry_enabled=True,        # Enable automatic retries
    max_retries=3,            # Maximum number of retries
    retry_delay=1.0,          # Initial delay between retries (seconds)
    retry_backoff=2.0,        # Backoff multiplier for delays
)
```

Retries are automatically performed for:
- Network connectivity issues
- HTTP 5xx server errors
- HTTP 429 (rate limit) responses
- Connection timeouts


### Telemetry Configuration

```python
config = ClientConfig(
    base_url="https://api.example.com/graphql",
    telemetry_enabled=True,              # Enable metrics collection
    metrics_endpoint="https://metrics.example.com",  # Metrics endpoint
)
```

Collected metrics include:
- Query execution time
- Operation count by type (query/mutation/subscription)
- Error count by operation
- Success/failure rates

## Async Support

The client also supports asynchronous operations using `asyncio`:

```python
import asyncio
from com.example.client import AsyncGraphQLClient, ClientConfig

async def main():
    config = ClientConfig(base_url="https://api.example.com/graphql")
    
    async with AsyncGraphQLClient(config) as client:
        # Async query
        result = await client.query("{ users { id name } }")
        print(result.data)
        
        # Async mutation
        mutation = "mutation { createUser(input: {name: \"Alice\"}) { id } }"
        result = await client.mutate(mutation)
        print(result.data)
        
        # Async subscription
        subscription = "subscription { userUpdates { id name } }"
        async for update in client.subscribe(subscription):
            print(f"Update: {update}")

# Run the async function
asyncio.run(main())
```

---

*This documentation was generated automatically from the GraphQL schema.*