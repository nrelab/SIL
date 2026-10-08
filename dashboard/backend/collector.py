"""collector package."""

import time
from typing import Any

EVENTS: list[dict[str, Any]] = []


def collect_event(event_type: str, payload: dict[str, Any]) -> None:
    """Collect event."""
    EVENTS.append({
        "type": event_type,
        "payload": payload,
        "timestamp": time.time(),
    })


def get_events() -> list[dict[str, Any]]:
    """Get events."""
    return EVENTS


def clear_events() -> None:
    """Clear events."""
    EVENTS.clear()
