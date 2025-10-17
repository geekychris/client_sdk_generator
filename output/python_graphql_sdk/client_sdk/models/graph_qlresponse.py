"""
GraphQL response object
"""

from dataclasses import dataclass, field
from typing import Optional, List, Dict, Any, Union
from datetime import datetime, date


@dataclass
class Graphqlresponse:
    """GraphQL response object

    This is a GraphQL type.
    """

    data: Optional[String] = None  # Response data

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> "Graphqlresponse":
        """Create an instance from a dictionary (typically from GraphQL response)."""
        return cls(data=data.get("data"))

    def to_dict(self) -> Dict[str, Any]:
        """Convert to a dictionary (for GraphQL variables)."""
        result = {}
        if self.data is not None:
            result["data"] = self.data
        return result

    def __str__(self) -> str:
        """String representation."""
        return f"Graphqlresponse(data={self.data})"
