from fastapi import APIRouter, Response

from app.core.dependencies import check_db_health

router = APIRouter(prefix="/health", tags=["health"])


@router.get("/live")
async def liveness() -> dict:
    return {"status": "ok"}


@router.get("/ready")
async def readiness(response: Response) -> dict:
    db_ok = await check_db_health()
    if not db_ok:
        response.status_code = 503
        return {"status": "unhealthy", "database": "disconnected"}
    return {"status": "ok", "database": "connected"}
