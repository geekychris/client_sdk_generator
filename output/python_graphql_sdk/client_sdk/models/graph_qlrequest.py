"""
GraphQL request object
"""

from dataclasses import dataclass, field
from typing import Optional, List, Dict, Any, Union
from datetime import datetime, date


@dataclass
class Graphqlrequest:
    """GraphQL request object

    This is a GraphQL type.
    """

    query: String  # GraphQL query string

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> "Graphqlrequest":
        """Create an instance from a dictionary (typically from GraphQL response)."""
        return cls(query=data.get("query"))

    def to_dict(self) -> Dict[str, Any]:
        """Convert to a dictionary (for GraphQL variables)."""
        result = {}
        if self.query is not None:
            result["query"] = self.query
        return result

    def __str__(self) -> str:
        """String representation."""
        return f"Graphqlrequest(query={self.query})"
