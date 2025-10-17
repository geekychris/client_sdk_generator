""" """

from dataclasses import dataclass, field
from typing import Optional, List, Dict, Any, Union
from datetime import datetime, date


@dataclass
class Newpet:
    """ """

    name: String  # Name of the pet
    tag: Optional[String] = None  # Tag associated with the pet
    photo_urls: Optional[String] = None  # URLs of pet photos

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> "Newpet":
        """Create an instance from a dictionary."""
        return cls(
            name=data.get("name"), tag=data.get("tag"), photo_urls=data.get("photoUrls")
        )

    def to_dict(self) -> Dict[str, Any]:
        """Convert to a dictionary."""
        result = {}
        if self.name is not None:
            result["name"] = self.name
        if self.tag is not None:
            result["tag"] = self.tag
        if self.photo_urls is not None:
            result["photoUrls"] = self.photo_urls
        return result

    def __str__(self) -> str:
        """String representation."""
        return f"Newpet(name={self.name}, tag={self.tag}, photo_urls={self.photo_urls})"
