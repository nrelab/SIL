# SIL Core Architecture

## System Overview

SIL (Semantic Integrity Layer) is a distributed security platform that ensures meaning consistency across code, data, and AI systems.

## Pipeline

```
Input → Normalizer → Confusable Detector → Semantic Engine → Policy Engine → Decision
```

## Layers

| Layer | Crate/Module | Responsibility |
|-------|-------------|----------------|
| Normalizer | sil-normalizer | Unicode NFKC, zero-width strip |
| Confusable | sil-confusable | Homoglyph detection, cross-script mix |
| Semantic | sil-semantic | Jaccard similarity, intent clustering |
| Policy | sil-policy | Risk scoring, Allow/Warn/Block/Rewrite |

## Deployment

See `infra/` for Docker, K8s, and Terraform configs.
