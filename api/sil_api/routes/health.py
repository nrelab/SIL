"""health package."""

from fastapi import APIRouter

router = APIRouter()


@router.get("/")
def health() -> dict[str, str]:
    """Health."""
    return {"status": "ok", "system": "SIL operational"}
