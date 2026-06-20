def predict_risk(events: list[dict]) -> float:
    score = 0.0

    for e in events:
        if e["type"] == "CONFUSABLE_DETECTED":
            score += 0.3
        elif e["type"] == "SEMANTIC_CONFLICT":
            score += 0.4
        elif e["type"] == "POLICY_BLOCK":
            score += 0.2

    return min(score, 1.0)
