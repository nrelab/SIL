# SIL Core

Semantic Integrity Layer for Code, Data & AI Systems.

## What is SIL?

SIL is a security + intelligence layer that detects:

- Unicode spoofing
- Confusable attacks
- Semantic drift in code
- AI-agent misinterpretation risks

## Architecture

```
Normalizer → Confusable → Semantic → Policy → Decision Engine
```

## Quick Start

```bash
docker compose up --build
```

## CLI

```bash
cargo run -p sil-cli -- --input "hello"
cargo run -p sil-cli -- --input "раypal"
```

## API

```bash
curl -X POST http://localhost:8000/scan \
  -H "Content-Type: application/json" \
  -d '{"input": "раypal"}'
```

## Project Structure

```
crates/          # Rust core engine (5 crates)
api/             # FastAPI gateway
worker/          # Async job processor
dashboard/       # Web UI
integrations/    # Git hooks, CI/CD, PR analyzer
infra/           # Docker, K8s, Terraform
docs/            # Architecture, API, security docs
scripts/         # Dev workflow scripts
tests/           # Unit + integration tests
```

## Modules

| Module | Responsibility |
|--------|---------------|
| sil-normalizer | Unicode NFKC, zero-width filtering |
| sil-confusable | Homoglyph detection, cross-script mixing |
| sil-semantic | Jaccard similarity, intent clustering |
| sil-policy | Risk scoring, Allow/Warn/Block/Rewrite |
| sil-cli | CLI product interface |

## License

MIT
