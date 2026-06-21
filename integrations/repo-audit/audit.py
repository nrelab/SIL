def audit_repository(files: list) -> dict:
    report = {
        "total_files": len(files),
        "unicode_risks": 0,
        "confusable_hits": 0,
        "semantic_drift": 0,
    }

    for f in files:
        if "\u0192" in f:
            report["unicode_risks"] += 1

        if "\u0440\u0430" in f:
            report["confusable_hits"] += 1

    return report


if __name__ == "__main__":
    files = [
        "src/login.rs",
        "src/\u0192dev.rs",
        "src/\u0440\u0430ypal_handler.rs",
    ]
    report = audit_repository(files)
    print(report)
