from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware

from app.core.config import settings
from app.health.router import router as health_router
from app.users.router import router as users_router
from app.partners.router import router as partners_router
from app.stations.router import router as stations_router
from app.chargers.router import router as chargers_router

app = FastAPI(title=settings.app_name)

app.add_middleware(
    CORSMiddleware,
    allow_origins=settings.cors_origins,
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

app.include_router(health_router)
app.include_router(users_router)
app.include_router(partners_router)
app.include_router(stations_router)
app.include_router(chargers_router)
