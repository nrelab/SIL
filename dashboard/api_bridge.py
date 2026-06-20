import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware

from backend.collector import EVENTS
from backend.metrics import compute_metrics
from backend.aggregator import compute_risk_score

app = FastAPI(title="SIL Dashboard API")

app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_methods=["*"],
    allow_headers=["*"],
)


@app.get("/metrics")
def metrics():
    return compute_metrics(EVENTS)


@app.get("/risk")
def risk():
    return {"risk_score": compute_risk_score(EVENTS)}


@app.get("/events")
def events():
    return {"events": EVENTS}


@app.post("/ingest")
def ingest(event_type: str, payload: dict):
    from backend.collector import collect_event

    collect_event(event_type, payload)
    return {"status": "ingested"}
