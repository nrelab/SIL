PATTERN_DB: dict[str, int] = {}


def record_pattern(pattern: str) -> None:
    PATTERN_DB[pattern] = PATTERN_DB.get(pattern, 0) + 1


def get_patterns() -> dict[str, int]:
    return dict(PATTERN_DB)


def clear_patterns() -> None:
    PATTERN_DB.clear()
