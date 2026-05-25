from sqlalchemy import select, delete
from sqlalchemy.ext.asyncio import AsyncSession
from app.chargers.models import Charger


class ChargerRepository:
    def __init__(self, session: AsyncSession):
        self._session = session

    async def list_all(self) -> list[Charger]:
        result = await self._session.execute(select(Charger).order_by(Charger.id))
        return list(result.scalars().all())

    async def get_by_id(self, charger_id: int) -> Charger | None:
        result = await self._session.execute(select(Charger).where(Charger.id == charger_id))
        return result.scalar_one_or_none()

    async def create(self, charger: Charger) -> Charger:
        self._session.add(charger)
        await self._session.flush()
        await self._session.refresh(charger)
        return charger

    async def update(self, charger: Charger) -> Charger:
        await self._session.flush()
        await self._session.refresh(charger)
        return charger

    async def delete(self, charger_id: int) -> None:
        await self._session.execute(delete(Charger).where(Charger.id == charger_id))
