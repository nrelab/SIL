"""
Rust SIL Engine bridge layer.

Currently uses a simulated pipeline. Replace with actual FFI
(e.g. PyO3 / ctypes) once the Rust core is compiled as a shared library.
"""


def run_sil_pipeline(text: str) -> dict:
    # Step 1: Normalize (Rust call placeholder)
    normalized = text

    # Step 2: Confusable detection
    flags: list[str] = []
    if "ра" in text or "ƒ" in text:
        flags.append("VISUAL_MISMATCH_DETECTED")
    if any(ord(c) > 0x7F for c in text):
        flags.append("CROSS_SCRIPT_MIXING")

    # Step 3: Semantic score (placeholder)
    semantic_score = 0.6

    # Step 4: Policy decision
    if "ƒ" in text:
        decision = "REWRITE"
        normalized = text.replace("ƒ", "f")
    elif any("\u0400" <= c <= "\u04FF" for c in text):
        decision = "BLOCK"
    else:
        decision = "ALLOW"

    return {
        "input": text,
        "normalized": normalized,
        "flags": flags,
        "semantic_score": semantic_score,
        "decision": decision,
    }
