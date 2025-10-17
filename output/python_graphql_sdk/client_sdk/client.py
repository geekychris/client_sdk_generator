"""
Generated from GraphQL schema

Generated GraphQL SDK for GraphQL API v1.0.0
"""

import asyncio
import json
from typing import Dict, Any, Optional, Union
import httpx
from dataclasses import dataclass

from prometheus_client import Counter, Histogram, CollectorRegistry
from tenacity import (
    retry,
    stop_after_attempt,
    wait_exponential,
    retry_if_exception_type,
)
import httpx
import time


@dataclass
class GraphQLResponse:
    """GraphQL response wrapper"""

    data: Optional[Dict[str, Any]] = None
    errors: Optional[list] = None


class GraphqlApiGraphQLClient:
    """GraphQL client for GraphQL API API"""

    def __init__(self, config: "ClientConfig"):
        self.config = config
        self.graphql_endpoint = f"{config.base_url}/graphql"
        self.client = httpx.Client(
            timeout=config.request_timeout_seconds,
            headers={"Content-Type": "application/json"},
        )

        self.telemetry_handler = TelemetryHandler()

    def sample(self, query: String) -> String:
        """
        Sample GraphQL query


        Args:
            query: GraphQL query string

        Returns:
            GraphQL response
        """

        try:
            # Build GraphQL query
            query = self._build_query("sample", query)

            # Create GraphQL request
            request_body = {"query": query}

            # Add variables if needed
            variables = {}
            if query is not None:
                variables["query"] = query
            if variables:
                request_body["variables"] = variables

            result = self._execute_graphql_request(request_body)

            return result

        except Exception as e:
            raise GraphqlApiGraphQLException(
                f"Failed to execute GraphQL operation sample: {str(e)}"
            ) from e

    async def sample_async(self, query: String) -> String:
        """
        Sample GraphQL query (Async version)

        Args:
            query: GraphQL query string

        Returns:
            GraphQL response
        """

        try:
            # Build GraphQL query
            query = self._build_query("sample", query)

            # Create GraphQL request
            request_body = {"query": query}

            # Add variables if needed
            variables = {}
            if query is not None:
                variables["query"] = query
            if variables:
                request_body["variables"] = variables

            async with httpx.AsyncClient(
                timeout=self.config.request_timeout_seconds
            ) as client:
                response = await client.post(
                    self.graphql_endpoint,
                    json=request_body,
                    headers={"Content-Type": "application/json"},
                )

                result = self._parse_graphql_response(response)

                return result

        except Exception as e:
            raise GraphqlApiGraphQLException(
                f"Failed to execute async GraphQL operation sample: {str(e)}"
            ) from e

    def _build_query(self, operation_name: str, *variables) -> str:
        """Build GraphQL query string"""
        # This is a simplified query builder - in a real implementation,
        # you would have proper GraphQL query generation based on the schema
        return f"query {operation_name} "

    def _execute_graphql_request(self, request_body: Dict[str, Any]) -> Dict[str, Any]:
        """Execute GraphQL request"""
        response = self.client.post(self.graphql_endpoint, json=request_body)
        return self._parse_graphql_response(response)

    def _parse_graphql_response(self, response: httpx.Response) -> Dict[str, Any]:
        """Parse GraphQL response"""
        if response.status_code >= 200 and response.status_code < 300:
            json_response = response.json()

            # Check for GraphQL errors
            if "errors" in json_response:
                raise GraphqlApiGraphQLException(
                    f"GraphQL errors: {json_response['errors']}"
                )

            # Return data
            return json_response.get("data", {})
        else:
            raise GraphqlApiGraphQLException(
                f"HTTP {response.status_code}: {response.text}"
            )


@retry(
    stop=stop_after_attempt(3),
    wait=wait_exponential(multiplier=2, min=0.1, max=5),
    retry=retry_if_exception_type((httpx.ConnectError, httpx.TimeoutException)),
)
async def execute_with_retry(self, operation):
    return await operation()


class TelemetryHandler:
    def __init__(self):
        self.registry = CollectorRegistry()
        self.request_duration = Histogram(
            "http_request_duration_seconds",
            "Time spent on HTTP requests",
            ["method", "path"],
            registry=self.registry,
        )
        self.request_count = Counter(
            "http_requests_total",
            "Total HTTP requests",
            ["method", "path", "status"],
            registry=self.registry,
        )

    def record_request(self, method: str, path: str, duration: float, status: int):
        self.request_duration.labels(method=method, path=path).observe(duration)
        self.request_count.labels(method=method, path=path, status=str(status)).inc()

    def close(self):
        """Close the HTTP client"""
        self.client.close()

    def __enter__(self):
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        self.close()


class GraphqlApiGraphQLException(Exception):
    """Exception raised by GraphQL client operations"""

    pass
