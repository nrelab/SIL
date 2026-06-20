import time

EVENTS: list[dict] = []


def collect_event(event_type: str, payload: dict) -> None:
    EVENTS.append({
        "type": event_type,
        "payload": payload,
        "timestamp": time.time(),
    })


def get_events() -> list[dict]:
    return EVENTS


def clear_events() -> None:
    EVENTS.clear()
