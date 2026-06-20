#!/bin/bash
set -euo pipefail

MODE="${1:-help}"

case "$MODE" in
  api)
    echo "Starting SIL API..."
    uvicorn sil_api.main:app --host 0.0.0.0 --port 8000 --reload --app-dir api
    ;;
  cli)
    shift
    cargo run -p sil-cli -- "$@"
    ;;
  worker)
    echo "Starting SIL Worker..."
    python3 worker/worker.py
    ;;
  test)
    echo "Running all tests..."
    cargo test --workspace
    python3 -m pytest tests/ -v
    ;;
  docker)
    echo "Starting Docker stack..."
    docker compose up --build
    ;;
  scan)
    shift
    cargo run -p sil-cli -- --input "$@"
    ;;
  *)
    echo "SIL Core — Usage: ./scripts/run.sh <command>"
    echo ""
    echo "Commands:"
    echo "  api          Start FastAPI dev server"
    echo "  cli          Run SIL CLI (append args)"
    echo "  worker       Start async worker"
    echo "  test         Run all tests (Rust + Python)"
    echo "  docker       Start full Docker stack"
    echo "  scan <text>  Scan input text"
    ;;
esac
