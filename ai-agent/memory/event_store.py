EVENT_MEMORY: list[dict] = []


def store_event(event: dict) -> None:
    EVENT_MEMORY.append(event)


def get_recent_events(limit: int = 100) -> list[dict]:
    return EVENT_MEMORY[-limit:]


def clear_memory() -> None:
    EVENT_MEMORY.clear()
