"""aggregator package."""

from typing import Any, Optional

from backend.metrics import compute_metrics


def compute_risk_score(events: Optional[list[dict[str, Any]]] = None) -> float:
    """Compute risk score."""
    metrics = compute_metrics(events)

    score: float = 0.0
    score += metrics["confusable_count"] * 0.4
    score += metrics["semantic_conflicts"] * 0.3
    score += metrics["blocked_inputs"] * 0.3

    return min(score, 1.0)
