# com.example.client API Documentation

Generated API documentation for com.example.client.

## Client

The main client class provides methods for interacting with the API.

```python
from com.example.client import Client, ClientConfig

config = ClientConfig(base_url="https://api.example.com")
client = Client(config)
```

## Models

### Pet

Pet data model

**Properties:**

- `id` (required): Unique identifier for the pet
- `name` (required): Name of the pet
- `tag` (optional): Tag associated with the pet
- `status` (optional): Pet status in the store
- `photoUrls` (optional): URLs of pet photos

### NewPet

NewPet data model

**Properties:**

- `name` (required): Name of the pet
- `tag` (optional): Tag associated with the pet
- `photoUrls` (optional): URLs of pet photos

### Category

Category data model

**Properties:**

- `id` (optional): Category identifier
- `name` (optional): Category name

### Error

Error data model

**Properties:**

- `code` (required): Error code
- `message` (required): Error message
- `details` (optional): Additional error details


## Operations

### listPets

Returns a list of all pets in the store

**Parameters:**

- `limit` (optional): Maximum number of pets to return
- `tag` (optional): Filter pets by tag


**Response:** A list of pets

```python
# Example usage
result = client.list_pets()
```

### createPet

Creates a new pet in the store



**Response:** Pet created successfully

```python
# Example usage
result = client.create_pet()
```

### getPet

Returns a single pet by its ID

**Parameters:**

- `petId` (required): ID of the pet to retrieve


**Response:** Pet details

```python
# Example usage
result = client.get_pet(pet_id="value")
```

### updatePet

Updates an existing pet

**Parameters:**

- `petId` (required): ID of the pet to update


**Response:** Pet updated successfully

```python
# Example usage
result = client.update_pet(pet_id="value")
```

### deletePet

Deletes a pet from the store

**Parameters:**

- `petId` (required): ID of the pet to delete


**Response:** Pet deleted successfully

```python
# Example usage
result = client.delete_pet(pet_id="value")
```

