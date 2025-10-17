"""
Sample gRPC request
"""

from dataclasses import dataclass, field
from typing import Optional, List, Dict, Any, Union
from datetime import datetime, date


@dataclass
class Samplerequest:
    """Sample gRPC request"""

    message: String  # Request message

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> "Samplerequest":
        """Create an instance from a dictionary."""
        return cls(message=data.get("message"))

    def to_dict(self) -> Dict[str, Any]:
        """Convert to a dictionary."""
        result = {}
        if self.message is not None:
            result["message"] = self.message
        return result

    def __str__(self) -> str:
        """String representation."""
        return f"Samplerequest(message={self.message})"
