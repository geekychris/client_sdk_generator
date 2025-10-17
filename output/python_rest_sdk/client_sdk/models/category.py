""" """

from dataclasses import dataclass, field
from typing import Optional, List, Dict, Any, Union
from datetime import datetime, date


@dataclass
class Category:
    """ """

    id: Optional[String] = None  # Category identifier
    name: Optional[String] = None  # Category name

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> "Category":
        """Create an instance from a dictionary."""
        return cls(id=data.get("id"), name=data.get("name"))

    def to_dict(self) -> Dict[str, Any]:
        """Convert to a dictionary."""
        result = {}
        if self.id is not None:
            result["id"] = self.id
        if self.name is not None:
            result["name"] = self.name
        return result

    def __str__(self) -> str:
        """String representation."""
        return f"Category(id={self.id}, name={self.name})"
