def analyze_pr(diff_text: str) -> dict:
    issues: list[str] = []

    if "\u0192" in diff_text:
        issues.append("UNICODE_CONFUSABLE_RISK")

    if "\u200b" in diff_text:
        issues.append("ZERO_WIDTH_CHARACTER")

    if "login" in diff_text and "auth" in diff_text:
        issues.append("SEMANTIC_OVERLAP")

    risk_level = "HIGH" if len(issues) > 1 else "LOW"

    return {"risk_level": risk_level, "issues": issues}


if __name__ == "__main__":
    result = analyze_pr("add \u0192login auth handler")
    print(result)
