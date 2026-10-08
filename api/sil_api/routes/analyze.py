"""analyze package."""

from fastapi import APIRouter
from pydantic import BaseModel

from sil_api.core_bridge import PipelineResult

router = APIRouter()


class AnalyzeRequest(BaseModel):
    """AnalyzeRequest class."""

    input: str


@router.post("/", response_model=PipelineResult)
def analyze(req: AnalyzeRequest) -> PipelineResult:
    """Analyze."""
    from sil_api.core_bridge import run_sil_pipeline

    result = run_sil_pipeline(req.input)
    return PipelineResult(**result)
