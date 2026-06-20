from fastapi import FastAPI

from sil_api.routes.scan import router as scan_router
from sil_api.routes.health import router as health_router

app = FastAPI(title="SIL API")

app.include_router(scan_router, prefix="/scan")
app.include_router(health_router, prefix="/health")


@app.get("/")
def root():
    return {"status": "SIL API running"}
