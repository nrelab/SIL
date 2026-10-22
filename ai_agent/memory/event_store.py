"""event_store package."""

from typing import Any

EVENT_MEMORY: list[dict[str, Any]] = []


def store_event(event: dict[str, Any]) -> None:
    """Store event."""
    EVENT_MEMORY.append(event)


def get_recent_events(limit: int = 100) -> list[dict[str, Any]]:
    """Get recent events."""
    return EVENT_MEMORY[-limit:]


def clear_memory() -> None:
    """Clear memory."""
    EVENT_MEMORY.clear()
