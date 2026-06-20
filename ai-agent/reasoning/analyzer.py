def detect_pattern(events: list[dict]) -> dict[str, int]:
    patterns: dict[str, int] = {}

    for e in events:
        t = e["type"]
        patterns[t] = patterns.get(t, 0) + 1

    return patterns
