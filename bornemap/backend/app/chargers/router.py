from fastapi import APIRouter, Depends
from sqlalchemy.ext.asyncio import AsyncSession

from app.core.dependencies import get_db
from app.chargers.schemas import ChargerResponse, ChargerCreate, ChargerUpdate
from app.chargers.service import ChargerService

router = APIRouter(prefix="/api/v1/chargers", tags=["chargers"])


@router.get("", response_model=list[ChargerResponse])
async def list_chargers(db: AsyncSession = Depends(get_db)):
    service = ChargerService(db)
    return await service.list_chargers()


@router.post("", response_model=ChargerResponse, status_code=201)
async def create_charger(data: ChargerCreate, db: AsyncSession = Depends(get_db)):
    service = ChargerService(db)
    return await service.create_charger(data)


@router.get("/{charger_id}", response_model=ChargerResponse)
async def get_charger(charger_id: int, db: AsyncSession = Depends(get_db)):
    service = ChargerService(db)
    return await service.get_charger(charger_id)


@router.patch("/{charger_id}", response_model=ChargerResponse)
async def update_charger(charger_id: int, data: ChargerUpdate, db: AsyncSession = Depends(get_db)):
    service = ChargerService(db)
    return await service.update_charger(charger_id, data)


@router.patch("/{charger_id}/toggle-active", response_model=ChargerResponse)
async def toggle_charger_active(charger_id: int, db: AsyncSession = Depends(get_db)):
    service = ChargerService(db)
    return await service.toggle_active(charger_id)


@router.delete("/{charger_id}", status_code=204)
async def delete_charger(charger_id: int, db: AsyncSession = Depends(get_db)):
    service = ChargerService(db)
    await service.delete_charger(charger_id)
