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
from pydantic import BaseModel

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


class IngestRequest(BaseModel):
    """IngestRequest class."""

    event_type: str
    payload: dict[str, Any]


@app.post("/ingest")
def ingest(req: IngestRequest) -> dict[str, str]:
    """Ingest."""
    from backend.collector import collect_event

    collect_event(req.event_type, req.payload)
    return {"status": "ingested"}
