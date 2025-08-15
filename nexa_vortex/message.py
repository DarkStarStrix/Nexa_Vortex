"""Defines the core Message data structure for the Vortex system."""
from dataclasses import dataclass, field
from typing import Any, Dict, Optional, Union
from uuid import UUID, uuid4
import json

@dataclass
class Message:
    """A standardized message for communication within the Vortex system."""
    id: UUID = field(default_factory=uuid4)
    correlation_id: Optional[UUID] = None
    headers: Dict[str, Any] = field(default_factory=dict)
    payload: Union[Dict[str, Any], str, bytes] = field(default_factory=dict)

    def __repr__(self) -> str:
        """Provides a developer-friendly representation of the message."""
        return f"Message(id={self.id}, correlation_id={self.correlation_id}, payload={self.payload})"

    def to_dict(self) -> Dict[str, Any]:
        """Serializes the message to a dictionary."""
        return {
            "id": str(self.id),
            "correlation_id": str(self.correlation_id) if self.correlation_id else None,
            "headers": self.headers,
            "payload": self.payload,
        }

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> "Message":
        """Deserializes a dictionary into a Message object."""
        return cls(
            id=UUID(data["id"]),
            correlation_id=UUID(data["correlation_id"]) if data.get("correlation_id") else None,
            headers=data.get("headers", {}),
            payload=data.get("payload", {}),
        )

    def to_json(self) -> str:
        """Serializes the message to a JSON string."""
        return json.dumps(self.to_dict())

