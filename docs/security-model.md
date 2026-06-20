# SIL Security Model

## Threat Model

SIL protects against:

1. **Unicode Spoofing** — Visually identical characters from different scripts
2. **Zero-Width Injection** — Invisible control characters in identifiers
3. **Homoglyph Attacks** — `ƒ` vs `f`, Cyrillic `ра` vs Latin `pa`
4. **Semantic Confusion** — Same intent, different naming (`login_user` vs `authenticate_user`)
5. **AI Misinterpretation** — Input that causes LLM/agent misclassification

## Trust Decision Flow

```
Risk Score > 0.8 → BLOCK
Risk Score > 0.5 → WARN
Repairable issue  → REWRITE
Otherwise         → ALLOW
```

## Enforcement Points

- **Pre-commit hook** — blocks unsafe commits locally
- **CI/CD gate** — fails PR with detected risks
- **API middleware** — scans all inputs at runtime
