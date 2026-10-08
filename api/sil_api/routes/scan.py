"""scan package."""

from typing import Any

from fastapi import APIRouter
from pydantic import BaseModel

router = APIRouter()


class ScanRequest(BaseModel):
    """ScanRequest class."""

    input: str


@router.post("")
def scan(req: ScanRequest) -> dict[str, Any]:
    """Scan."""
    from sil_api.core_bridge import run_sil_pipeline

    result = run_sil_pipeline(req.input)
    return result
