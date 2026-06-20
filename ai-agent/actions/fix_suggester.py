def suggest_fix(input_text: str) -> list[str]:
    fixes: list[str] = []

    if "ƒ" in input_text:
        fixes.append("Replace 'ƒ' with 'f'")

    if "\u200b" in input_text:
        fixes.append("Remove zero-width characters")

    if "ра" in input_text:
        fixes.append("Normalize Cyrillic characters to Latin equivalents")

    return fixes
