#!/bin/bash
set -euo pipefail

echo "🔧 SIL Core Setup"

# Install Rust toolchain
if ! command -v rustc &>/dev/null; then
    echo "Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi

# Build Rust workspace
echo "Building Rust crates..."
cargo build --workspace

# Install Python deps
echo "Installing Python dependencies..."
pip3 install -r api/sil_api/requirements.txt

# Setup git hooks
echo "Installing git hooks..."
cp integrations/git-hooks/pre-commit .git/hooks/pre-commit
cp integrations/git-hooks/pre-push .git/hooks/pre-push
chmod +x .git/hooks/*

echo "✅ SIL Core setup complete"
echo "Run: cargo test --workspace"
