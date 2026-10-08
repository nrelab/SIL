"""fix_suggester package."""

def suggest_fix(input_text: str) -> list[str]:
    """Suggest fixes for the given input."""
    fixes: list[str] = []

    if "\u0192" in input_text:
        fixes.append("Replace '\\u0192' with 'f'")

    if "\u200b" in input_text:
        fixes.append("Remove zero-width characters")

    if "\u0440\u0430" in input_text:
        fixes.append("Normalize Cyrillic characters to Latin equivalents")

    return fixes
