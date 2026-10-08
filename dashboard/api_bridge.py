"""api_bridge package."""

import os
import sys
from typing import Any

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

from backend.aggregator import compute_risk_score
from backend.collector import EVENTS
from backend.metrics import compute_metrics
from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware

app = FastAPI(title="SIL Dashboard API")

app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_methods=["*"],
    allow_headers=["*"],
)


@app.get("/metrics")
def metrics() -> dict[str, Any]:
    """Metrics."""
    return compute_metrics(EVENTS)


@app.get("/risk")
def risk() -> dict[str, float]:
    """Risk."""
    return {"risk_score": compute_risk_score(EVENTS)}


@app.get("/events")
def events() -> dict[str, Any]:
    """Events."""
    return {"events": EVENTS}


@app.post("/ingest")
def ingest(event_type: str, payload: dict[str, Any]) -> dict[str, str]:
    """Ingest."""
    from backend.collector import collect_event

    collect_event(event_type, payload)
    return {"status": "ingested"}
