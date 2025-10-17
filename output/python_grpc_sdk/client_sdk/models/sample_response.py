"""
Sample gRPC response
"""

from dataclasses import dataclass, field
from typing import Optional, List, Dict, Any, Union
from datetime import datetime, date


@dataclass
class Sampleresponse:
    """Sample gRPC response"""

    result: String  # Response result

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> "Sampleresponse":
        """Create an instance from a dictionary."""
        return cls(result=data.get("result"))

    def to_dict(self) -> Dict[str, Any]:
        """Convert to a dictionary."""
        result = {}
        if self.result is not None:
            result["result"] = self.result
        return result

    def __str__(self) -> str:
        """String representation."""
        return f"Sampleresponse(result={self.result})"
