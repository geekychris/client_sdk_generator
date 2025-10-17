# PetStoreApi API Documentation

This document provides detailed information about the Pet Store API API and how to use the generated Rust SDK.

## Base URL

```
https://api.example.com
```

## Authentication

This API does not require authentication.

## Available Methods

### listpets

Returns a list of all pets in the store

**HTTP Method**: `GET`  
**Path**: `/pets`

#### Parameters

- **limit** (Query): Maximum number of pets to return
  - Type: `String`

- **tag** (Query): Filter pets by tag
  - Type: `String`



#### Responses

- **200**: A list of pets
  - Type: `String`

- **400**: Bad request


#### Usage Example

```rust

let result = client.list_pets(
).await?;

println!("Result: {:?}", result);
```

---

### createpet

Creates a new pet in the store

**HTTP Method**: `POST`  
**Path**: `/pets`



#### Responses

- **201**: Pet created successfully

- **400**: Invalid input


#### Usage Example

```rust

let result = client.create_pet(
).await?;

println!("Result: {:?}", result);
```

---

### getpet

Returns a single pet by its ID

**HTTP Method**: `GET`  
**Path**: `/pets/{petId}`

#### Parameters

- **petId** (Path, required): ID of the pet to retrieve
  - Type: `String`



#### Responses

- **200**: Pet details

- **404**: Pet not found


#### Usage Example

```rust
let pet_id = /* your String value */;

let result = client.get_pet(
    pet_id,
).await?;

println!("Result: {:?}", result);
```

---

### updatepet

Updates an existing pet

**HTTP Method**: `PUT`  
**Path**: `/pets/{petId}`

#### Parameters

- **petId** (Path, required): ID of the pet to update
  - Type: `String`



#### Responses

- **200**: Pet updated successfully

- **404**: Pet not found


#### Usage Example

```rust
let pet_id = /* your String value */;

let result = client.update_pet(
    pet_id,
).await?;

println!("Result: {:?}", result);
```

---

### deletepet

Deletes a pet from the store

**HTTP Method**: `DELETE`  
**Path**: `/pets/{petId}`

#### Parameters

- **petId** (Path, required): ID of the pet to delete
  - Type: `String`



#### Responses

- **204**: Pet deleted successfully

- **404**: Pet not found


#### Usage Example

```rust
let pet_id = /* your String value */;

let result = client.delete_pet(
    pet_id,
).await?;

println!("Result: {:?}", result);
```

---


## Data Types

### Pet


#### Properties

- **id** (required): Unique identifier for the pet
  - Type: `String`

- **name** (required): Name of the pet
  - Type: `String`

- **tag**: Tag associated with the pet
  - Type: `String`

- **status**: Pet status in the store
  - Type: `String`

- **photoUrls**: URLs of pet photos
  - Type: `String`


#### Usage Example

```rust
use com.example.client::Pet;

let instance = Pet {
    id: /* your String value */,
    name: /* your String value */,
    tag: None,
    status: None,
    photo_urls: None,
};

// Or use the builder pattern
let instance = Pet::new(
    /* id */ /* your String value */,
    /* name */ /* your String value */,
)
.with_tag(/* your String value */)
.with_status(/* your String value */)
.with_photo_urls(/* your String value */);
```

---

### Newpet


#### Properties

- **name** (required): Name of the pet
  - Type: `String`

- **tag**: Tag associated with the pet
  - Type: `String`

- **photoUrls**: URLs of pet photos
  - Type: `String`


#### Usage Example

```rust
use com.example.client::Newpet;

let instance = Newpet {
    name: /* your String value */,
    tag: None,
    photo_urls: None,
};

// Or use the builder pattern
let instance = Newpet::new(
    /* name */ /* your String value */,
)
.with_tag(/* your String value */)
.with_photo_urls(/* your String value */);
```

---

### Category


#### Properties

- **id**: Category identifier
  - Type: `String`

- **name**: Category name
  - Type: `String`


#### Usage Example

```rust
use com.example.client::Category;

let instance = Category {
    id: None,
    name: None,
};

// Or use the builder pattern
let instance = Category::new(
)
.with_id(/* your String value */)
.with_name(/* your String value */);
```

---

### Error


#### Properties

- **code** (required): Error code
  - Type: `String`

- **message** (required): Error message
  - Type: `String`

- **details**: Additional error details
  - Type: `String`


#### Usage Example

```rust
use com.example.client::Error;

let instance = Error {
    code: /* your String value */,
    message: /* your String value */,
    details: None,
};

// Or use the builder pattern
let instance = Error::new(
    /* code */ /* your String value */,
    /* message */ /* your String value */,
)
.with_details(/* your String value */);
```

---


## Error Handling

The SDK provides a comprehensive error type that covers all possible error conditions:

```rust
#[derive(Debug, thiserror::Error)]
pub enum PetStoreApiError {
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