"""pattern_db package."""

from typing import Any

PATTERN_DB: dict[str, int] = {}


def record_pattern(pattern: str) -> None:
    """Record pattern."""
    PATTERN_DB[pattern] = PATTERN_DB.get(pattern, 0) + 1


def get_patterns() -> dict[str, int]:
    """Get patterns."""
    return dict[str, Any](PATTERN_DB)


def clear_patterns() -> None:
    """Clear patterns."""
    PATTERN_DB.clear()
