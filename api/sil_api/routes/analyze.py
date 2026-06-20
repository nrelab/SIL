from fastapi import APIRouter
from pydantic import BaseModel

router = APIRouter()


class AnalyzeRequest(BaseModel):
    input: str


@router.post("/")
def analyze(req: AnalyzeRequest):
    from sil_api.core_bridge import run_sil_pipeline

    result = run_sil_pipeline(req.input)
    return result
