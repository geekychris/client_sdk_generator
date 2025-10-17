"""
A sample API that uses a pet store as an example

Generated SDK for Pet Store API v1.0.0
"""

from prometheus_client import Counter, Histogram, CollectorRegistry
from tenacity import retry, stop_after_attempt, wait_exponential, retry_if_exception_type
import httpx
import time

import httpx
import asyncio
from typing import Optional, Dict, Any, List, Union
from dataclasses import dataclass, asdict
import json
from urllib.parse import urljoin
from .telemetry_handler import TelemetryHandler

from .config import ClientConfig
from .exceptions import PetStoreApiException
from .models.pet import Pet
from .models.new_pet import Newpet
from .models.category import Category
from .models.error import Error


class PetStoreApiClient:
    """A sample API that uses a pet store as an example"""
    
    def __init__(self, config: ClientConfig):
        self.config = config
        self.base_url = config.base_url
        self._client = httpx.Client(
            timeout=config.timeout,
            headers=config.default_headers or {}
        )
        
        self.telemetry = TelemetryHandler()
        
    
    def __enter__(self):
        return self
    
    def __exit__(self, exc_type, exc_val, exc_tb):
        self._client.close()
    
    def list_pets(self, limit: Optional[String] = None, tag: Optional[String] = None) -> String:
        """Returns a list of all pets in the store
        
        Args:
            limit: Maximum number of pets to return
            tag: Filter pets by tag
        
        Returns:
            A list of pets
        """
        
        
        try:
            url = self._build_url("/pets", limit, tag)
            
            params = {}
            headers = {}
            
            # Add query parameters
            if limit is not None:
                params["limit"] = limit
            if tag is not None:
                params["tag"] = tag
            
            # Add headers
            if limit is not None:
                headers["limit"] = str(limit)
            if tag is not None:
                headers["tag"] = str(tag)
            
            # Prepare request data
            json_data = None
            
            response = self._client.request(
                "GET",
                url,
                params=params,
                headers=headers,
                json=json_data
            )
            result = self._parse_response(response, None  # TODO: Fix type parsing)
            
            
            
            return result
            
        except Exception as e:
            raise PetStoreApiException(f"Failed to execute listPets: {str(e)}") from e
    
    def create_pet(self) -> None:
        """Creates a new pet in the store
        
        Args:
        
        Returns:
            Pet created successfully
        """
        
        
        try:
            url = self._build_url("/pets")
            
            params = {}
            headers = {}
            
            # Add query parameters
            
            # Add headers
            
            # Prepare request data
            json_data = None
            
            response = self._client.request(
                "POST",
                url,
                params=params,
                headers=headers,
                json=json_data
            )
            result = self._parse_response(response)
            
            
            
            return result
            
        except Exception as e:
            raise PetStoreApiException(f"Failed to execute createPet: {str(e)}") from e
    
    def get_pet(self, pet_id: String) -> None:
        """Returns a single pet by its ID
        
        Args:
            pet_id: ID of the pet to retrieve
        
        Returns:
            Pet details
        """
        
        
        try:
            url = self._build_url("/pets/{petId}", pet_id)
            
            params = {}
            headers = {}
            
            # Add query parameters
            if pet_id is not None:
                params["petId"] = pet_id
            
            # Add headers
            if pet_id is not None:
                headers["petId"] = str(pet_id)
            
            # Prepare request data
            json_data = None
            
            response = self._client.request(
                "GET",
                url,
                params=params,
                headers=headers,
                json=json_data
            )
            result = self._parse_response(response)
            
            
            
            return result
            
        except Exception as e:
            raise PetStoreApiException(f"Failed to execute getPet: {str(e)}") from e
    
    def update_pet(self, pet_id: String) -> None:
        """Updates an existing pet
        
        Args:
            pet_id: ID of the pet to update
        
        Returns:
            Pet updated successfully
        """
        
        
        try:
            url = self._build_url("/pets/{petId}", pet_id)
            
            params = {}
            headers = {}
            
            # Add query parameters
            if pet_id is not None:
                params["petId"] = pet_id
            
            # Add headers
            if pet_id is not None:
                headers["petId"] = str(pet_id)
            
            # Prepare request data
            json_data = None
            
            response = self._client.request(
                "PUT",
                url,
                params=params,
                headers=headers,
                json=json_data
            )
            result = self._parse_response(response)
            
            
            
            return result
            
        except Exception as e:
            raise PetStoreApiException(f"Failed to execute updatePet: {str(e)}") from e
    
    def delete_pet(self, pet_id: String) -> None:
        """Deletes a pet from the store
        
        Args:
            pet_id: ID of the pet to delete
        
        Returns:
            Pet deleted successfully
        """
        
        
        try:
            url = self._build_url("/pets/{petId}", pet_id)
            
            params = {}
            headers = {}
            
            # Add query parameters
            if pet_id is not None:
                params["petId"] = pet_id
            
            # Add headers
            if pet_id is not None:
                headers["petId"] = str(pet_id)
            
            # Prepare request data
            json_data = None
            
            response = self._client.request(
                "DELETE",
                url,
                params=params,
                headers=headers,
                json=json_data
            )
            result = self._parse_response(response)
            
            
            
            return result
            
        except Exception as e:
            raise PetStoreApiException(f"Failed to execute deletePet: {str(e)}") from e
    
    
    def _build_url(self, path: str, *path_params) -> str:
        """Build the full URL for a request."""
        url = path
        for param in path_params:
            url = url.replace("{" + str(param) + "}", str(param), 1)
        return urljoin(self.base_url, url.lstrip('/'))
    
    def _parse_response(self, response: httpx.Response, response_type=None):
        """Parse HTTP response."""
        response.raise_for_status()
        
        if response_type is None:
            return None
            
        if response.status_code == 204:  # No Content
            return None
            
        try:
            data = response.json()
            if response_type and hasattr(response_type, '__dataclass_fields__'):
                return response_type(**data)
            return data
        except Exception as e:
            raise PetStoreApiException(f"Failed to parse response: {str(e)}") from e
    
    
@retry(
    stop=stop_after_attempt(3),
    wait=wait_exponential(multiplier=2, min=0.1, max=5),
    retry=retry_if_exception_type((httpx.ConnectError, httpx.TimeoutException))
)
async def execute_with_retry(self, operation):
    return await operation()



class TelemetryHandler:
    def __init__(self):
        self.registry = CollectorRegistry()
        self.request_duration = Histogram(
            'http_request_duration_seconds',
            'Time spent on HTTP requests',
            ['method', 'path'],
            registry=self.registry
        )
        self.request_count = Counter(
            'http_requests_total',
            'Total HTTP requests',
            ['method', 'path', 'status'],
            registry=self.registry
        )
    
    def record_request(self, method: str, path: str, duration: float, status: int):
        self.request_duration.labels(method=method, path=path).observe(duration)
        self.request_count.labels(method=method, path=path, status=str(status)).inc()



class PetStoreApiAsyncClient:
    """Async version of PetStoreApiClient"""
    
    def __init__(self, config: ClientConfig):
        self.config = config
        self.base_url = config.base_url
        self._client = None
        
        self.telemetry = TelemetryHandler()
        
    
    async def __aenter__(self):
        self._client = httpx.AsyncClient(
            timeout=self.config.timeout,
            headers=self.config.default_headers or {}
        )
        return self
    
    async def __aexit__(self, exc_type, exc_val, exc_tb):
        if self._client:
            await self._client.aclose()
    
    async def list_pets(self, limit: Optional[String] = None, tag: Optional[String] = None) -> String:
        """Returns a list of all pets in the store (async version)
        
        Args:
            limit: Maximum number of pets to return
            tag: Filter pets by tag
        
        Returns:
            A list of pets
        """
        
        
        try:
            url = self._build_url("/pets", limit, tag)
            
            params = {}
            headers = {}
            
            # Add query parameters
            if limit is not None:
                params["limit"] = limit
            if tag is not None:
                params["tag"] = tag
            
            # Add headers
            if limit is not None:
                headers["limit"] = str(limit)
            if tag is not None:
                headers["tag"] = str(tag)
            
            # Prepare request data
            json_data = None
            
            response = await self._client.request(
                "GET",
                url,
                params=params,
                headers=headers,
                json=json_data
            )
            result = self._parse_response(response, None  # TODO: Fix type parsing)
            
            
            
            return result
            
        except Exception as e:
            raise PetStoreApiException(f"Failed to execute listPets: {str(e)}") from e
    
    async def create_pet(self) -> None:
        """Creates a new pet in the store (async version)
        
        Args:
        
        Returns:
            Pet created successfully
        """
        
        
        try:
            url = self._build_url("/pets")
            
            params = {}
            headers = {}
            
            # Add query parameters
            
            # Add headers
            
            # Prepare request data
            json_data = None
            
            response = await self._client.request(
                "POST",
                url,
                params=params,
                headers=headers,
                json=json_data
            )
            result = self._parse_response(response)
            
            
            
            return result
            
        except Exception as e:
            raise PetStoreApiException(f"Failed to execute createPet: {str(e)}") from e
    
    async def get_pet(self, pet_id: String) -> None:
        """Returns a single pet by its ID (async version)
        
        Args:
            pet_id: ID of the pet to retrieve
        
        Returns:
            Pet details
        """
        
        
        try:
            url = self._build_url("/pets/{petId}", pet_id)
            
            params = {}
            headers = {}
            
            # Add query parameters
            if pet_id is not None:
                params["petId"] = pet_id
            
            # Add headers
            if pet_id is not None:
                headers["petId"] = str(pet_id)
            
            # Prepare request data
            json_data = None
            
            response = await self._client.request(
                "GET",
                url,
                params=params,
                headers=headers,
                json=json_data
            )
            result = self._parse_response(response)
            
            
            
            return result
            
        except Exception as e:
            raise PetStoreApiException(f"Failed to execute getPet: {str(e)}") from e
    
    async def update_pet(self, pet_id: String) -> None:
        """Updates an existing pet (async version)
        
        Args:
            pet_id: ID of the pet to update
        
        Returns:
            Pet updated successfully
        """
        
        
        try:
            url = self._build_url("/pets/{petId}", pet_id)
            
            params = {}
            headers = {}
            
            # Add query parameters
            if pet_id is not None:
                params["petId"] = pet_id
            
            # Add headers
            if pet_id is not None:
                headers["petId"] = str(pet_id)
            
            # Prepare request data
            json_data = None
            
            response = await self._client.request(
                "PUT",
                url,
                params=params,
                headers=headers,
                json=json_data
            )
            result = self._parse_response(response)
            
            
            
            return result
            
        except Exception as e:
            raise PetStoreApiException(f"Failed to execute updatePet: {str(e)}") from e
    
    async def delete_pet(self, pet_id: String) -> None:
        """Deletes a pet from the store (async version)
        
        Args:
            pet_id: ID of the pet to delete
        
        Returns:
            Pet deleted successfully
        """
        
        
        try:
            url = self._build_url("/pets/{petId}", pet_id)
            
            params = {}
            headers = {}
            
            # Add query parameters
            if pet_id is not None:
                params["petId"] = pet_id
            
            # Add headers
            if pet_id is not None:
                headers["petId"] = str(pet_id)
            
            # Prepare request data
            json_data = None
            
            response = await self._client.request(
                "DELETE",
                url,
                params=params,
                headers=headers,
                json=json_data
            )
            result = self._parse_response(response)
            
            
            
            return result
            
        except Exception as e:
            raise PetStoreApiException(f"Failed to execute deletePet: {str(e)}") from e
    
    
    def _build_url(self, path: str, *path_params) -> str:
        """Build the full URL for a request."""
        url = path
        for param in path_params:
            url = url.replace("{" + str(param) + "}", str(param), 1)
        return urljoin(self.base_url, url.lstrip('/'))
    
    def _parse_response(self, response: httpx.Response, response_type=None):
        """Parse HTTP response."""
        response.raise_for_status()
        
        if response_type is None:
            return None
            
        if response.status_code == 204:  # No Content
            return None
            
        try:
            data = response.json()
            if response_type and hasattr(response_type, '__dataclass_fields__'):
                return response_type(**data)
            return data
        except Exception as e:
            raise PetStoreApiException(f"Failed to parse response: {str(e)}") from e
