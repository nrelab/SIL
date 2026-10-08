"""analyze package."""

from typing import Any

from fastapi import APIRouter
from pydantic import BaseModel

router = APIRouter()


class AnalyzeRequest(BaseModel):
    """AnalyzeRequest class."""

    input: str


@router.post("/")
def analyze(req: AnalyzeRequest) -> dict[str, Any]:
    """Analyze."""
    from sil_api.core_bridge import run_sil_pipeline

    result = run_sil_pipeline(req.input)
    return result
