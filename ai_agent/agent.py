"""agent package."""

from typing import Any

from actions.fix_suggester import suggest_fix
from actions.refactor_advisor import suggest_refactor
from memory.event_store import get_recent_events, store_event
from reasoning.analyzer import detect_pattern
from reasoning.predictor import predict_risk


class SILAgent:
    """SILAgent class."""

    def analyze(self, input_text: str) -> dict[str, Any]:
        """Analyze."""
        store_event({"type": "INPUT_RECEIVED", "input": input_text})

        events = get_recent_events()
        risk = predict_risk(events)
        patterns = detect_pattern(events)
        fixes = suggest_fix(input_text)
        refactor = suggest_refactor(input_text)

        return {
            "risk_score": risk,
            "patterns": patterns,
            "fix_suggestions": fixes,
            "refactor_advice": refactor,
        }
