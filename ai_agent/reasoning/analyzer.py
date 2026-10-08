"""analyzer package."""

from typing import Any


def detect_pattern(events: list[dict[str, Any]]) -> dict[str, int]:
    """Detect repeating patterns in events."""
    patterns: dict[str, int] = {}

    for e in events:
        t = e["type"]
        patterns[t] = patterns.get(t, 0) + 1

    return patterns
