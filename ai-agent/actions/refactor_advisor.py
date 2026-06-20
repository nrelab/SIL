def suggest_refactor(name: str) -> str:
    if "login" in name and "auth" in name:
        return "Consider merging login/auth modules"

    if "manager" in name and "handler" in name:
        return "Possible responsibility overlap detected"

    return "No refactor needed"
