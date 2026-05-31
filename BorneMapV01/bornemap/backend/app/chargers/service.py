from fastapi import HTTPException
from sqlalchemy.ext.asyncio import AsyncSession

from app.chargers.models import Charger
from app.chargers.repository import ChargerRepository
from app.chargers.schemas import ChargerCreate, ChargerUpdate


class ChargerService:
    def __init__(self, session: AsyncSession):
        self._repo = ChargerRepository(session)

    async def list_chargers(self) -> list[Charger]:
        return await self._repo.list_all()

    async def get_charger(self, charger_id: int) -> Charger:
        charger = await self._repo.get_by_id(charger_id)
        if not charger:
            raise HTTPException(status_code=404, detail="Charger not found")
        return charger

    async def create_charger(self, data: ChargerCreate) -> Charger:
        charger = Charger(
            station_id=data.station_id,
            connector=data.connector,
            power_kw=data.power_kw,
            status=data.status,
            is_active=data.is_active,
        )
        return await self._repo.create(charger)

    async def update_charger(self, charger_id: int, data: ChargerUpdate) -> Charger:
        charger = await self.get_charger(charger_id)
        if data.station_id is not None:
            charger.station_id = data.station_id
        if data.connector is not None:
            charger.connector = data.connector
        if data.power_kw is not None:
            charger.power_kw = data.power_kw
        if data.status is not None:
            charger.status = data.status
        if data.is_active is not None:
            charger.is_active = data.is_active
        return await self._repo.update(charger)

    async def toggle_active(self, charger_id: int) -> Charger:
        charger = await self.get_charger(charger_id)
        charger.is_active = not charger.is_active
        return await self._repo.update(charger)

    async def delete_charger(self, charger_id: int) -> None:
        await self.get_charger(charger_id)
        await self._repo.delete(charger_id)
