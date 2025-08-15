"""Defines the core message structure for the Vortex framework."""

from __future__ import annotations

import time
from dataclasses import dataclass, field
from typing import Any, Dict, Optional, Union
from uuid import UUID, uuid4


@dataclass
class Message:
    """
    Represents a message passed between components in the Vortex system.

    Attributes:
        id: A unique identifier for the message.
        correlation_id: An optional identifier to correlate messages, for example, in a request-reply pattern.
        timestamp: The time the message was created.
        content_type: The type of content in the message body.
        content_encoding: The encoding of the message content.
        headers: A dictionary of message headers.
        payload: The message payload.
    """

    id: UUID = field(default_factory=uuid4)
    correlation_id: Optional[UUID] = None
    timestamp: float = field(default_factory=time.time)
    content_type: str = "application/json"
    content_encoding: str = "utf-8"
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
            "timestamp": self.timestamp,
            "content_type": self.content_type,
            "content_encoding": self.content_encoding,
            "headers": self.headers,
            "payload": self.payload,
        }

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> Message:
        """Deserializes a dictionary to a Message object."""
        return cls(
            id=UUID(data["id"]),
            correlation_id=UUID(data["correlation_id"]) if data.get("correlation_id") else None,
            timestamp=data["timestamp"],
            content_type=data["content_type"],
            content_encoding=data["content_encoding"],
            headers=data["headers"],
            payload=data["payload"],
        )
