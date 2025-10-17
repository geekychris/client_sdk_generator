# Pet Store API API Documentation

A sample API that uses a pet store as an example

This document provides detailed information about the Pet Store API API and how to use it with the TypeScript SDK.

## Base URL

```
https://api.petstore.com/v1
```

## Authentication

No authentication required.

## Endpoints

### listPets

Returns a list of all pets in the store

**Endpoint:** ` /pets`

#### Parameters

- **limit** ([object]) - Maximum number of pets to return
  - Location: Query
  - Required: No

- **tag** ([object]) - Filter pets by tag
  - Location: Query
  - Required: No



#### Response

**Success Response (200):**

```json
{ "type": "object", "properties": {} }
```

A list of pets

#### TypeScript SDK Usage

```typescript
// Define parameters
const params = {
  limit: 'example-value'123true, // Maximum number of pets to return
  tag: 'example-value'123true, // Filter pets by tag
};

try {
  const result = await client.listpets(params);
  console.log('listPets result:', result);
} catch (error) {
  console.error('listPets error:', error);
}
```

#### cURL Example

```bash
curl -X  \
  '/pets' \
```

---

### createPet

Creates a new pet in the store

**Endpoint:** ` /pets`



#### Response

**Success Response (201):**

No response body.

Pet created successfully

#### TypeScript SDK Usage

```typescript
try {
  const result = await client.createpet();
  console.log('createPet result:', result);
} catch (error) {
  console.error('createPet error:', error);
}
```

#### cURL Example

```bash
curl -X  \
  '/pets' \
```

---

### getPet

Returns a single pet by its ID

**Endpoint:** ` /pets/{petId}`

#### Parameters

- **petId** ([object]) - ID of the pet to retrieve
  - Location: Path
  - Required: Yes



#### Response

**Success Response (200):**

No response body.

Pet details

#### TypeScript SDK Usage

```typescript
// Define parameters
const params = {
  petid: 'example-value'123true, // ID of the pet to retrieve
};

try {
  const result = await client.getpet(params);
  console.log('getPet result:', result);
} catch (error) {
  console.error('getPet error:', error);
}
```

#### cURL Example

```bash
curl -X  \
  '/pets/{petId}' \
```

---

### updatePet

Updates an existing pet

**Endpoint:** ` /pets/{petId}`

#### Parameters

- **petId** ([object]) - ID of the pet to update
  - Location: Path
  - Required: Yes



#### Response

**Success Response (200):**

No response body.

Pet updated successfully

#### TypeScript SDK Usage

```typescript
// Define parameters
const params = {
  petid: 'example-value'123true, // ID of the pet to update
};

try {
  const result = await client.updatepet(params);
  console.log('updatePet result:', result);
} catch (error) {
  console.error('updatePet error:', error);
}
```

#### cURL Example

```bash
curl -X  \
  '/pets/{petId}' \
```

---

### deletePet

Deletes a pet from the store

**Endpoint:** ` /pets/{petId}`

#### Parameters

- **petId** ([object]) - ID of the pet to delete
  - Location: Path
  - Required: Yes



#### Response

**Success Response (204):**

No response body.

Pet deleted successfully

#### TypeScript SDK Usage

```typescript
// Define parameters
const params = {
  petid: 'example-value'123true, // ID of the pet to delete
};

try {
  const result = await client.deletepet(params);
  console.log('deletePet result:', result);
} catch (error) {
  console.error('deletePet error:', error);
}
```

#### cURL Example

```bash
curl -X  \
  '/pets/{petId}' \
```

---


## Data Models

### Pet



```typescript
interface Pet {
  /**
   * Unique identifier for the pet
   */
  id: [object];
  /**
   * Name of the pet
   */
  name: [object];
  /**
   * Tag associated with the pet
   */
  tag?: [object];
  /**
   * Pet status in the store
   */
  status?: [object];
  /**
   * URLs of pet photos
   */
  photourls?: [object];
}
```



**Usage Example:**

```typescript
import { Pet, createPet, validatePet, isPet } from '';

// Create a new Pet
const pet = createPet('example'123true, 'example'123true, );

// Validate the object
const errors = validatePet(pet);
if (errors.length === 0) {
  console.log('Pet is valid');
}

// Type guard check
if (isPet(someObject)) {
  // TypeScript now knows someObject is of type Pet
  console.log('Object is a Pet');
}
```

---

### NewPet



```typescript
interface NewPet {
  /**
   * Name of the pet
   */
  name: [object];
  /**
   * Tag associated with the pet
   */
  tag?: [object];
  /**
   * URLs of pet photos
   */
  photourls?: [object];
}
```



**Usage Example:**

```typescript
import { NewPet, createNewPet, validateNewPet, isNewPet } from '';

// Create a new NewPet
const newpet = createNewPet('example'123true, );

// Validate the object
const errors = validateNewPet(newpet);
if (errors.length === 0) {
  console.log('NewPet is valid');
}

// Type guard check
if (isNewPet(someObject)) {
  // TypeScript now knows someObject is of type NewPet
  console.log('Object is a NewPet');
}
```

---

### Category



```typescript
interface Category {
  /**
   * Category identifier
   */
  id?: [object];
  /**
   * Category name
   */
  name?: [object];
}
```



**Usage Example:**

```typescript
import { Category, createCategory, validateCategory, isCategory } from '';

// Create a new Category
const category = createCategory();

// Validate the object
const errors = validateCategory(category);
if (errors.length === 0) {
  console.log('Category is valid');
}

// Type guard check
if (isCategory(someObject)) {
  // TypeScript now knows someObject is of type Category
  console.log('Object is a Category');
}
```

---

### Error



```typescript
interface Error {
  /**
   * Error code
   */
  code: [object];
  /**
   * Error message
   */
  message: [object];
  /**
   * Additional error details
   */
  details?: [object];
}
```



**Usage Example:**

```typescript
import { Error, createError, validateError, isError } from '';

// Create a new Error
const error = createError('example'123true, 'example'123true, );

// Validate the object
const errors = validateError(error);
if (errors.length === 0) {
  console.log('Error is valid');
}

// Type guard check
if (isError(someObject)) {
  // TypeScript now knows someObject is of type Error
  console.log('Object is a Error');
}
```

---


## Error Responses

The API returns structured error responses in the following format:

```typescript
interface APIError {
  status: number;        // HTTP status code
  message: string;       // Error message
  code?: string;         // Optional error code
  details?: string;      // Optional error details
}
```

### Common Error Status Codes

- **400 Bad Request**: Invalid request parameters or body
- **401 Unauthorized**: Missing or invalid authentication
- **403 Forbidden**: Insufficient permissions
- **404 Not Found**: Resource not found
- **422 Unprocessable Entity**: Validation errors
- **429 Too Many Requests**: Rate limit exceeded
- **500 Internal Server Error**: Server error

### Error Handling in TypeScript

```typescript
import { APIClientError } from 'client';

try {
  const result = await client.someMethod();
  // Handle successful response
} catch (error) {
  if (error instanceof APIClientError) {
    console.error(`API Error ${error.status}: ${error.message}`);
    
    switch (error.status) {
      case 400:
        console.error('Bad request - check your parameters');
        break;
      case 401:
        console.error('Unauthorized - check your credentials');
        break;
      case 404:
        console.error('Resource not found');
        break;
      case 429:
        console.error('Rate limit exceeded - try again later');
        break;
      default:
        console.error('Unexpected API error');
    }
  } else {
    console.error('Network or other error:', error);
  }
}
```

## Rate Limiting

Rate limiting information is not specified for this API.

## Pagination

Pagination is not specified for this API.

## SDK Features

### Automatic Retry

The SDK automatically retries failed requests with exponential backoff:

- **Max Retries**: 3 attempts by default
- **Retry Conditions**: Network errors, timeouts, and 5xx server errors
- **Backoff Strategy**: Exponential backoff with jitter

**Configuration:**
```typescript
const client = new PetStoreApiclient({
  baseURL: 'https://api.petstore.com/v1',
  retry: {
    maxRetries: 5,
    baseDelay: 1000,
    maxDelay: 10000,
  }
});
```

### Response Caching

Response caching is not configured for this SDK.

### Request Timeout

All requests have a configurable timeout:

```typescript
const client = new PetStoreApiclient({
  baseURL: 'https://api.petstore.com/v1',
  timeout: 30000, // 30 seconds
});
```

### Request Cancellation

Requests can be cancelled using AbortController:

```typescript
const controller = new AbortController();

// Cancel the request after 5 seconds
setTimeout(() => controller.abort(), 5000);

try {
  const result = await client.someMethod(params, {
    signal: controller.signal
  });
} catch (error) {
  if (error.name === 'AbortError') {
    console.log('Request was cancelled');
  }
}
```

## TypeScript Support

The SDK provides full TypeScript support with:

- **Type Definitions**: Complete TypeScript definitions for all API operations and data models
- **Type Guards**: Runtime type checking functions for all data models
- **Generic Types**: Support for generic response types and pagination
- **Strict Null Checks**: Compatible with strict TypeScript configurations

## Browser and Node.js Support

The SDK works in both browser and Node.js environments:

### Browser
- Modern browsers with fetch API support
- Works with bundlers like Webpack, Rollup, Vite
- Supports both ES modules and CommonJS

### Node.js
- Node.js 16.0.0 or higher
- Uses node-fetch for HTTP requests
- Full async/await support

## Contributing

This SDK is generated automatically from the API specification. To report issues or request features, please contact the API provider.

## Support

For API-related questions and support, please refer to the API provider's documentation or support channels.