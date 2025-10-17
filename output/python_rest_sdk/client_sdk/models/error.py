""" """

from dataclasses import dataclass, field
from typing import Optional, List, Dict, Any, Union
from datetime import datetime, date


@dataclass
class Error:
    """ """

    code: String  # Error code
    message: String  # Error message
    details: Optional[String] = None  # Additional error details

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> "Error":
        """Create an instance from a dictionary."""
        return cls(
            code=data.get("code"),
            message=data.get("message"),
            details=data.get("details"),
        )

    def to_dict(self) -> Dict[str, Any]:
        """Convert to a dictionary."""
        result = {}
        if self.code is not None:
            result["code"] = self.code
        if self.message is not None:
            result["message"] = self.message
        if self.details is not None:
            result["details"] = self.details
        return result

    def __str__(self) -> str:
        """String representation."""
        return (
            f"Error(code={self.code}, message={self.message}, details={self.details})"
        )
