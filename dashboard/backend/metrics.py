from typing import Optional

from backend.collector import get_events


def compute_metrics(events: Optional[list[dict]] = None) -> dict:
    if events is None:
        events = get_events()

    metrics = {
        "total_events": len(events),
        "confusable_count": 0,
        "blocked_inputs": 0,
        "semantic_conflicts": 0,
    }

    for e in events:
        if e["type"] == "CONFUSABLE_DETECTED":
            metrics["confusable_count"] += 1
        elif e["type"] == "POLICY_BLOCK":
            metrics["blocked_inputs"] += 1
        elif e["type"] == "SEMANTIC_CLUSTER_MATCH":
            metrics["semantic_conflicts"] += 1

    return metrics
