"""
Generated from UserService gRPC proto files

Generated gRPC SDK for UserService gRPC API v1.0.0
"""

from prometheus_client import Counter, Histogram, CollectorRegistry
from tenacity import (
    retry,
    stop_after_attempt,
    wait_exponential,
    retry_if_exception_type,
)
import httpx
import time

import grpc
import asyncio
from typing import Optional, Dict, Any, List, Union
from dataclasses import dataclass, asdict
import json
import time
from .telemetry_handler import TelemetryHandler

from .config import ClientConfig
from .exceptions import UserserviceGrpcApiException
from .models.sample_request import Samplerequest
from .models.sample_response import Sampleresponse


class UserserviceGrpcApiClient:
    """Generated from UserService gRPC proto files"""

    def __init__(self, config: ClientConfig):
        self.config = config
        self.server_address = config.base_url
        self._channel = None
        self._stub = None

        self.telemetry = TelemetryHandler()

    def __enter__(self):
        self.connect()
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        self.close()

    def connect(self):
        """Establish gRPC connection"""
        if self._channel is None:
            self._channel = grpc.insecure_channel(self.server_address)
            # Note: In a real implementation, you'd import the generated gRPC stub
            # self._stub = YourServiceStub(self._channel)

    def close(self):
        """Close gRPC connection"""
        if self._channel:
            self._channel.close()
            self._channel = None
            self._stub = None

    def register_user(self, request: String) -> String:
        """Register a new user account

        Args:
            request: The request body

        Returns:
            gRPC response
        """

        try:
            if not self._channel:
                self.connect()

            # Build gRPC request
            # Note: In a real implementation, you'd create the proper request message
            # grpc_request = YourRequestMessage(...)

            # result = self._stub.Registeruser(grpc_request)
            result = None  # Placeholder for gRPC call

            return result

        except Exception as e:
            raise UserserviceGrpcApiException(
                f"Failed to execute RegisterUser: {str(e)}"
            ) from e

    def login_user(self, request: String) -> String:
        """Authenticate a user and create session

        Args:
            request: The request body

        Returns:
            gRPC response
        """

        try:
            if not self._channel:
                self.connect()

            # Build gRPC request
            # Note: In a real implementation, you'd create the proper request message
            # grpc_request = YourRequestMessage(...)

            # result = self._stub.Loginuser(grpc_request)
            result = None  # Placeholder for gRPC call

            return result

        except Exception as e:
            raise UserserviceGrpcApiException(
                f"Failed to execute LoginUser: {str(e)}"
            ) from e

    def refresh_token(self, request: String) -> String:
        """Refresh authentication token

        Args:
            request: The request body

        Returns:
            gRPC response
        """

        try:
            if not self._channel:
                self.connect()

            # Build gRPC request
            # Note: In a real implementation, you'd create the proper request message
            # grpc_request = YourRequestMessage(...)

            # result = self._stub.Refreshtoken(grpc_request)
            result = None  # Placeholder for gRPC call

            return result

        except Exception as e:
            raise UserserviceGrpcApiException(
                f"Failed to execute RefreshToken: {str(e)}"
            ) from e

    def logout_user(self, request: String) -> String:
        """Logout user and invalidate session

        Args:
            request: The request body

        Returns:
            gRPC response
        """

        try:
            if not self._channel:
                self.connect()

            # Build gRPC request
            # Note: In a real implementation, you'd create the proper request message
            # grpc_request = YourRequestMessage(...)

            # result = self._stub.Logoutuser(grpc_request)
            result = None  # Placeholder for gRPC call

            return result

        except Exception as e:
            raise UserserviceGrpcApiException(
                f"Failed to execute LogoutUser: {str(e)}"
            ) from e

    def get_user(self, request: String) -> String:
        """Get user profile by ID

        Args:
            request: The request body

        Returns:
            gRPC response
        """

        try:
            if not self._channel:
                self.connect()

            # Build gRPC request
            # Note: In a real implementation, you'd create the proper request message
            # grpc_request = YourRequestMessage(...)

            # result = self._stub.Getuser(grpc_request)
            result = None  # Placeholder for gRPC call

            return result

        except Exception as e:
            raise UserserviceGrpcApiException(
                f"Failed to execute GetUser: {str(e)}"
            ) from e

    def get_current_user(self, request: String) -> String:
        """Get current authenticated user profile

        Args:
            request: The request body

        Returns:
            gRPC response
        """

        try:
            if not self._channel:
                self.connect()

            # Build gRPC request
            # Note: In a real implementation, you'd create the proper request message
            # grpc_request = YourRequestMessage(...)

            # result = self._stub.Getcurrentuser(grpc_request)
            result = None  # Placeholder for gRPC call

            return result

        except Exception as e:
            raise UserserviceGrpcApiException(
                f"Failed to execute GetCurrentUser: {str(e)}"
            ) from e

    def update_user(self, request: String) -> String:
        """Update user profile information

        Args:
            request: The request body

        Returns:
            gRPC response
        """

        try:
            if not self._channel:
                self.connect()

            # Build gRPC request
            # Note: In a real implementation, you'd create the proper request message
            # grpc_request = YourRequestMessage(...)

            # result = self._stub.Updateuser(grpc_request)
            result = None  # Placeholder for gRPC call

            return result

        except Exception as e:
            raise UserserviceGrpcApiException(
                f"Failed to execute UpdateUser: {str(e)}"
            ) from e

    def delete_user(self, request: String) -> String:
        """Delete user account

        Args:
            request: The request body

        Returns:
            gRPC response
        """

        try:
            if not self._channel:
                self.connect()

            # Build gRPC request
            # Note: In a real implementation, you'd create the proper request message
            # grpc_request = YourRequestMessage(...)

            # result = self._stub.Deleteuser(grpc_request)
            result = None  # Placeholder for gRPC call

            return result

        except Exception as e:
            raise UserserviceGrpcApiException(
                f"Failed to execute DeleteUser: {str(e)}"
            ) from e

    def list_users(self, request: String) -> String:
        """List users with pagination and filtering

        Args:
            request: The request body

        Returns:
            gRPC response
        """

        try:
            if not self._channel:
                self.connect()

            # Build gRPC request
            # Note: In a real implementation, you'd create the proper request message
            # grpc_request = YourRequestMessage(...)

            # result = self._stub.Listusers(grpc_request)
            result = None  # Placeholder for gRPC call

            return result

        except Exception as e:
            raise UserserviceGrpcApiException(
                f"Failed to execute ListUsers: {str(e)}"
            ) from e

    def change_password(self, request: String) -> String:
        """Change user password

        Args:
            request: The request body

        Returns:
            gRPC response
        """

        try:
            if not self._channel:
                self.connect()

            # Build gRPC request
            # Note: In a real implementation, you'd create the proper request message
            # grpc_request = YourRequestMessage(...)

            # result = self._stub.Changepassword(grpc_request)
            result = None  # Placeholder for gRPC call

            return result

        except Exception as e:
            raise UserserviceGrpcApiException(
                f"Failed to execute ChangePassword: {str(e)}"
            ) from e

    def reset_password(self, request: String) -> String:
        """Reset user password

        Args:
            request: The request body

        Returns:
            gRPC response
        """

        try:
            if not self._channel:
                self.connect()

            # Build gRPC request
            # Note: In a real implementation, you'd create the proper request message
            # grpc_request = YourRequestMessage(...)

            # result = self._stub.Resetpassword(grpc_request)
            result = None  # Placeholder for gRPC call

            return result

        except Exception as e:
            raise UserserviceGrpcApiException(
                f"Failed to execute ResetPassword: {str(e)}"
            ) from e

    def send_verification_email(self, request: String) -> String:
        """Send email verification

        Args:
            request: The request body

        Returns:
            gRPC response
        """

        try:
            if not self._channel:
                self.connect()

            # Build gRPC request
            # Note: In a real implementation, you'd create the proper request message
            # grpc_request = YourRequestMessage(...)

            # result = self._stub.Sendverificationemail(grpc_request)
            result = None  # Placeholder for gRPC call

            return result

        except Exception as e:
            raise UserserviceGrpcApiException(
                f"Failed to execute SendVerificationEmail: {str(e)}"
            ) from e

    def verify_email(self, request: String) -> String:
        """Verify email address

        Args:
            request: The request body

        Returns:
            gRPC response
        """

        try:
            if not self._channel:
                self.connect()

            # Build gRPC request
            # Note: In a real implementation, you'd create the proper request message
            # grpc_request = YourRequestMessage(...)

            # result = self._stub.Verifyemail(grpc_request)
            result = None  # Placeholder for gRPC call

            return result

        except Exception as e:
            raise UserserviceGrpcApiException(
                f"Failed to execute VerifyEmail: {str(e)}"
            ) from e

    def list_user_sessions(self, request: String) -> String:
        """List user active sessions

        Args:
            request: The request body

        Returns:
            gRPC response
        """

        try:
            if not self._channel:
                self.connect()

            # Build gRPC request
            # Note: In a real implementation, you'd create the proper request message
            # grpc_request = YourRequestMessage(...)

            # result = self._stub.Listusersessions(grpc_request)
            result = None  # Placeholder for gRPC call

            return result

        except Exception as e:
            raise UserserviceGrpcApiException(
                f"Failed to execute ListUserSessions: {str(e)}"
            ) from e

    def revoke_session(self, request: String) -> String:
        """Revoke a user session

        Args:
            request: The request body

        Returns:
            gRPC response
        """

        try:
            if not self._channel:
                self.connect()

            # Build gRPC request
            # Note: In a real implementation, you'd create the proper request message
            # grpc_request = YourRequestMessage(...)

            # result = self._stub.Revokesession(grpc_request)
            result = None  # Placeholder for gRPC call

            return result

        except Exception as e:
            raise UserserviceGrpcApiException(
                f"Failed to execute RevokeSession: {str(e)}"
            ) from e

    def get_user_preferences(self, request: String) -> String:
        """Get user preferences

        Args:
            request: The request body

        Returns:
            gRPC response
        """

        try:
            if not self._channel:
                self.connect()

            # Build gRPC request
            # Note: In a real implementation, you'd create the proper request message
            # grpc_request = YourRequestMessage(...)

            # result = self._stub.Getuserpreferences(grpc_request)
            result = None  # Placeholder for gRPC call

            return result

        except Exception as e:
            raise UserserviceGrpcApiException(
                f"Failed to execute GetUserPreferences: {str(e)}"
            ) from e

    def update_user_preferences(self, request: String) -> String:
        """Update user preferences

        Args:
            request: The request body

        Returns:
            gRPC response
        """

        try:
            if not self._channel:
                self.connect()

            # Build gRPC request
            # Note: In a real implementation, you'd create the proper request message
            # grpc_request = YourRequestMessage(...)

            # result = self._stub.Updateuserpreferences(grpc_request)
            result = None  # Placeholder for gRPC call

            return result

        except Exception as e:
            raise UserserviceGrpcApiException(
                f"Failed to execute UpdateUserPreferences: {str(e)}"
            ) from e


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


class UserserviceGrpcApiAsyncClient:
    """Async version of UserserviceGrpcApiClient"""

    def __init__(self, config: ClientConfig):
        self.config = config
        self.server_address = config.base_url
        self._channel = None
        self._stub = None

        self.telemetry = TelemetryHandler()

    async def __aenter__(self):
        await self.connect()
        return self

    async def __aexit__(self, exc_type, exc_val, exc_tb):
        await self.close()

    async def connect(self):
        """Establish async gRPC connection"""
        if self._channel is None:
            self._channel = grpc.aio.insecure_channel(self.server_address)
            # Note: In a real implementation, you'd import the generated gRPC stub
            # self._stub = YourServiceStub(self._channel)

    async def close(self):
        """Close async gRPC connection"""
        if self._channel:
            await self._channel.close()
            self._channel = None
            self._stub = None

    async def register_user(self, request: String) -> String:
        """Register a new user account (async version)

        Args:
            request: The request body

        Returns:
            gRPC response
        """

        try:
            if not self._channel:
                await self.connect()

            # Build gRPC request
            # Note: In a real implementation, you'd create the proper request message
            # grpc_request = YourRequestMessage(...)

            # result = await self._stub.Registeruser(grpc_request)
            result = None  # Placeholder for async gRPC call

            return result

        except Exception as e:
            raise UserserviceGrpcApiException(
                f"Failed to execute RegisterUser: {str(e)}"
            ) from e

    async def login_user(self, request: String) -> String:
        """Authenticate a user and create session (async version)

        Args:
            request: The request body

        Returns:
            gRPC response
        """

        try:
            if not self._channel:
                await self.connect()

            # Build gRPC request
            # Note: In a real implementation, you'd create the proper request message
            # grpc_request = YourRequestMessage(...)

            # result = await self._stub.Loginuser(grpc_request)
            result = None  # Placeholder for async gRPC call

            return result

        except Exception as e:
            raise UserserviceGrpcApiException(
                f"Failed to execute LoginUser: {str(e)}"
            ) from e

    async def refresh_token(self, request: String) -> String:
        """Refresh authentication token (async version)

        Args:
            request: The request body

        Returns:
            gRPC response
        """

        try:
            if not self._channel:
                await self.connect()

            # Build gRPC request
            # Note: In a real implementation, you'd create the proper request message
            # grpc_request = YourRequestMessage(...)

            # result = await self._stub.Refreshtoken(grpc_request)
            result = None  # Placeholder for async gRPC call

            return result

        except Exception as e:
            raise UserserviceGrpcApiException(
                f"Failed to execute RefreshToken: {str(e)}"
            ) from e

    async def logout_user(self, request: String) -> String:
        """Logout user and invalidate session (async version)

        Args:
            request: The request body

        Returns:
            gRPC response
        """

        try:
            if not self._channel:
                await self.connect()

            # Build gRPC request
            # Note: In a real implementation, you'd create the proper request message
            # grpc_request = YourRequestMessage(...)

            # result = await self._stub.Logoutuser(grpc_request)
            result = None  # Placeholder for async gRPC call

            return result

        except Exception as e:
            raise UserserviceGrpcApiException(
                f"Failed to execute LogoutUser: {str(e)}"
            ) from e

    async def get_user(self, request: String) -> String:
        """Get user profile by ID (async version)

        Args:
            request: The request body

        Returns:
            gRPC response
        """

        try:
            if not self._channel:
                await self.connect()

            # Build gRPC request
            # Note: In a real implementation, you'd create the proper request message
            # grpc_request = YourRequestMessage(...)

            # result = await self._stub.Getuser(grpc_request)
            result = None  # Placeholder for async gRPC call

            return result

        except Exception as e:
            raise UserserviceGrpcApiException(
                f"Failed to execute GetUser: {str(e)}"
            ) from e

    async def get_current_user(self, request: String) -> String:
        """Get current authenticated user profile (async version)

        Args:
            request: The request body

        Returns:
            gRPC response
        """

        try:
            if not self._channel:
                await self.connect()

            # Build gRPC request
            # Note: In a real implementation, you'd create the proper request message
            # grpc_request = YourRequestMessage(...)

            # result = await self._stub.Getcurrentuser(grpc_request)
            result = None  # Placeholder for async gRPC call

            return result

        except Exception as e:
            raise UserserviceGrpcApiException(
                f"Failed to execute GetCurrentUser: {str(e)}"
            ) from e

    async def update_user(self, request: String) -> String:
        """Update user profile information (async version)

        Args:
            request: The request body

        Returns:
            gRPC response
        """

        try:
            if not self._channel:
                await self.connect()

            # Build gRPC request
            # Note: In a real implementation, you'd create the proper request message
            # grpc_request = YourRequestMessage(...)

            # result = await self._stub.Updateuser(grpc_request)
            result = None  # Placeholder for async gRPC call

            return result

        except Exception as e:
            raise UserserviceGrpcApiException(
                f"Failed to execute UpdateUser: {str(e)}"
            ) from e

    async def delete_user(self, request: String) -> String:
        """Delete user account (async version)

        Args:
            request: The request body

        Returns:
            gRPC response
        """

        try:
            if not self._channel:
                await self.connect()

            # Build gRPC request
            # Note: In a real implementation, you'd create the proper request message
            # grpc_request = YourRequestMessage(...)

            # result = await self._stub.Deleteuser(grpc_request)
            result = None  # Placeholder for async gRPC call

            return result

        except Exception as e:
            raise UserserviceGrpcApiException(
                f"Failed to execute DeleteUser: {str(e)}"
            ) from e

    async def list_users(self, request: String) -> String:
        """List users with pagination and filtering (async version)

        Args:
            request: The request body

        Returns:
            gRPC response
        """

        try:
            if not self._channel:
                await self.connect()

            # Build gRPC request
            # Note: In a real implementation, you'd create the proper request message
            # grpc_request = YourRequestMessage(...)

            # result = await self._stub.Listusers(grpc_request)
            result = None  # Placeholder for async gRPC call

            return result

        except Exception as e:
            raise UserserviceGrpcApiException(
                f"Failed to execute ListUsers: {str(e)}"
            ) from e

    async def change_password(self, request: String) -> String:
        """Change user password (async version)

        Args:
            request: The request body

        Returns:
            gRPC response
        """

        try:
            if not self._channel:
                await self.connect()

            # Build gRPC request
            # Note: In a real implementation, you'd create the proper request message
            # grpc_request = YourRequestMessage(...)

            # result = await self._stub.Changepassword(grpc_request)
            result = None  # Placeholder for async gRPC call

            return result

        except Exception as e:
            raise UserserviceGrpcApiException(
                f"Failed to execute ChangePassword: {str(e)}"
            ) from e

    async def reset_password(self, request: String) -> String:
        """Reset user password (async version)

        Args:
            request: The request body

        Returns:
            gRPC response
        """

        try:
            if not self._channel:
                await self.connect()

            # Build gRPC request
            # Note: In a real implementation, you'd create the proper request message
            # grpc_request = YourRequestMessage(...)

            # result = await self._stub.Resetpassword(grpc_request)
            result = None  # Placeholder for async gRPC call

            return result

        except Exception as e:
            raise UserserviceGrpcApiException(
                f"Failed to execute ResetPassword: {str(e)}"
            ) from e

    async def send_verification_email(self, request: String) -> String:
        """Send email verification (async version)

        Args:
            request: The request body

        Returns:
            gRPC response
        """

        try:
            if not self._channel:
                await self.connect()

            # Build gRPC request
            # Note: In a real implementation, you'd create the proper request message
            # grpc_request = YourRequestMessage(...)

            # result = await self._stub.Sendverificationemail(grpc_request)
            result = None  # Placeholder for async gRPC call

            return result

        except Exception as e:
            raise UserserviceGrpcApiException(
                f"Failed to execute SendVerificationEmail: {str(e)}"
            ) from e

    async def verify_email(self, request: String) -> String:
        """Verify email address (async version)

        Args:
            request: The request body

        Returns:
            gRPC response
        """

        try:
            if not self._channel:
                await self.connect()

            # Build gRPC request
            # Note: In a real implementation, you'd create the proper request message
            # grpc_request = YourRequestMessage(...)

            # result = await self._stub.Verifyemail(grpc_request)
            result = None  # Placeholder for async gRPC call

            return result

        except Exception as e:
            raise UserserviceGrpcApiException(
                f"Failed to execute VerifyEmail: {str(e)}"
            ) from e

    async def list_user_sessions(self, request: String) -> String:
        """List user active sessions (async version)

        Args:
            request: The request body

        Returns:
            gRPC response
        """

        try:
            if not self._channel:
                await self.connect()

            # Build gRPC request
            # Note: In a real implementation, you'd create the proper request message
            # grpc_request = YourRequestMessage(...)

            # result = await self._stub.Listusersessions(grpc_request)
            result = None  # Placeholder for async gRPC call

            return result

        except Exception as e:
            raise UserserviceGrpcApiException(
                f"Failed to execute ListUserSessions: {str(e)}"
            ) from e

    async def revoke_session(self, request: String) -> String:
        """Revoke a user session (async version)

        Args:
            request: The request body

        Returns:
            gRPC response
        """

        try:
            if not self._channel:
                await self.connect()

            # Build gRPC request
            # Note: In a real implementation, you'd create the proper request message
            # grpc_request = YourRequestMessage(...)

            # result = await self._stub.Revokesession(grpc_request)
            result = None  # Placeholder for async gRPC call

            return result

        except Exception as e:
            raise UserserviceGrpcApiException(
                f"Failed to execute RevokeSession: {str(e)}"
            ) from e

    async def get_user_preferences(self, request: String) -> String:
        """Get user preferences (async version)

        Args:
            request: The request body

        Returns:
            gRPC response
        """

        try:
            if not self._channel:
                await self.connect()

            # Build gRPC request
            # Note: In a real implementation, you'd create the proper request message
            # grpc_request = YourRequestMessage(...)

            # result = await self._stub.Getuserpreferences(grpc_request)
            result = None  # Placeholder for async gRPC call

            return result

        except Exception as e:
            raise UserserviceGrpcApiException(
                f"Failed to execute GetUserPreferences: {str(e)}"
            ) from e

    async def update_user_preferences(self, request: String) -> String:
        """Update user preferences (async version)

        Args:
            request: The request body

        Returns:
            gRPC response
        """

        try:
            if not self._channel:
                await self.connect()

            # Build gRPC request
            # Note: In a real implementation, you'd create the proper request message
            # grpc_request = YourRequestMessage(...)

            # result = await self._stub.Updateuserpreferences(grpc_request)
            result = None  # Placeholder for async gRPC call

            return result

        except Exception as e:
            raise UserserviceGrpcApiException(
                f"Failed to execute UpdateUserPreferences: {str(e)}"
            ) from e
