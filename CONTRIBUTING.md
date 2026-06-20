# Contributing to SIL Core

## Getting Started

```bash
# Clone and enter
git clone git@github.com:nrelab/SIL.git
cd SIL

# Rust toolchain
rustup toolchain install stable
cargo build --workspace

# Python API
python3 -m venv .venv
source .venv/bin/activate
pip install -r api/requirements.txt

# Dashboard
cd dashboard && npm install && cd ..

# Start all services
docker compose up --build
```

## Project Structure

```
crates/          # Rust core engine (5 crates)
  sil-normalizer # Unicode NFKC, zero-width filtering
  sil-confusable # Homoglyph detection, cross-script mixing
  sil-semantic   # Jaccard similarity, intent clustering
  sil-policy     # Risk scoring, Allow/Warn/Block/Rewrite
  sil-cli        # CLI product interface
api/             # FastAPI gateway
worker/          # Async job processor (Redis + Postgres)
dashboard/       # Next.js web UI
integrations/    # Git hooks, CI/CD, PR analyzer, repo audit
infra/           # Docker, K8s, Terraform
docs/            # Architecture, API, security docs
scripts/         # Dev workflow scripts
tests/           # Unit + integration tests
```

## Development Workflow

1. **Pick an issue** — check open issues or propose one
2. **Create a branch** — `git checkout -b feat/my-feature`
3. **Make changes** — follow the conventions below
4. **Run tests** — `cargo test --workspace` for Rust, `pytest tests/` for Python
5. **Lint** — `cargo fmt && cargo clippy --workspace`
6. **Commit** — use conventional commits (`feat:`, `fix:`, `docs:`, etc.)
7. **Open a PR** — describe what and why

## Code Conventions

- **Rust**: follow `rustfmt` defaults and `clippy` pedantic linting
- **Python**: PEP 8, type hints required for all public functions
- **TypeScript/JS**: Prettier defaults, strict mode
- **Commits**: [Conventional Commits](https://www.conventionalcommits.org/)
- **No secrets**: never commit `.env` files, tokens, or credentials

## Testing

- Rust: `cargo test --workspace`
- Python: `pytest tests/`
- All new features must include tests
- Bug fixes should include a regression test

## Pull Request Process

1. Ensure CI passes (lint, test, build)
2. Update docs if public API changes
3. Add a changelog entry if applicable
4. Request review from at least one maintainer
5. Squash merge with a clean commit message

## Security

SIL detects spoofing and confusable attacks. When contributing:

- Never introduce new unicode confusables without clear policy
- Run `cargo make security-audit` before submitting sensitive changes
- Report vulnerabilities privately — do not file a public issue

## License

By contributing, you agree that your contributions will be licensed under the MIT License (see [LICENSE](LICENSE)).
