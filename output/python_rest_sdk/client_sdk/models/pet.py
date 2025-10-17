""" """

from dataclasses import dataclass, field
from typing import Optional, List, Dict, Any, Union
from datetime import datetime, date


@dataclass
class Pet:
    """ """

    id: String  # Unique identifier for the pet
    name: String  # Name of the pet
    tag: Optional[String] = None  # Tag associated with the pet
    status: Optional[String] = None  # Pet status in the store
    photo_urls: Optional[String] = None  # URLs of pet photos

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> "Pet":
        """Create an instance from a dictionary."""
        return cls(
            id=data.get("id"),
            name=data.get("name"),
            tag=data.get("tag"),
            status=data.get("status"),
            photo_urls=data.get("photoUrls"),
        )

    def to_dict(self) -> Dict[str, Any]:
        """Convert to a dictionary."""
        result = {}
        if self.id is not None:
            result["id"] = self.id
        if self.name is not None:
            result["name"] = self.name
        if self.tag is not None:
            result["tag"] = self.tag
        if self.status is not None:
            result["status"] = self.status
        if self.photo_urls is not None:
            result["photoUrls"] = self.photo_urls
        return result

    def __str__(self) -> str:
        """String representation."""
        return f"Pet(id={self.id}, name={self.name}, tag={self.tag}, status={self.status}, photo_urls={self.photo_urls})"
