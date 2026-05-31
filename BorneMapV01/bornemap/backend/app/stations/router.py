from fastapi import APIRouter, Depends
from sqlalchemy.ext.asyncio import AsyncSession

from app.core.dependencies import get_db
from app.stations.schemas import StationResponse, StationCreate, StationUpdate
from app.stations.service import StationService

router = APIRouter(prefix="/api/v1/stations", tags=["stations"])


@router.get("", response_model=list[StationResponse])
async def list_stations(db: AsyncSession = Depends(get_db)):
    service = StationService(db)
    return await service.list_stations()


@router.post("", response_model=StationResponse, status_code=201)
async def create_station(data: StationCreate, db: AsyncSession = Depends(get_db)):
    service = StationService(db)
    return await service.create_station(data)


@router.get("/{station_id}", response_model=StationResponse)
async def get_station(station_id: int, db: AsyncSession = Depends(get_db)):
    service = StationService(db)
    return await service.get_station(station_id)


@router.patch("/{station_id}", response_model=StationResponse)
async def update_station(station_id: int, data: StationUpdate, db: AsyncSession = Depends(get_db)):
    service = StationService(db)
    return await service.update_station(station_id, data)


@router.patch("/{station_id}/toggle-active", response_model=StationResponse)
async def toggle_station_active(station_id: int, db: AsyncSession = Depends(get_db)):
    service = StationService(db)
    return await service.toggle_active(station_id)


@router.delete("/{station_id}", status_code=204)
async def delete_station(station_id: int, db: AsyncSession = Depends(get_db)):
    service = StationService(db)
    await service.delete_station(station_id)
