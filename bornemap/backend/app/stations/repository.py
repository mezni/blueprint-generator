from sqlalchemy import select, delete
from sqlalchemy.ext.asyncio import AsyncSession
from app.stations.models import Station


class StationRepository:
    def __init__(self, session: AsyncSession):
        self._session = session

    async def list_all(self) -> list[Station]:
        result = await self._session.execute(select(Station).order_by(Station.id))
        return list(result.scalars().all())

    async def get_by_id(self, station_id: int) -> Station | None:
        result = await self._session.execute(select(Station).where(Station.id == station_id))
        return result.scalar_one_or_none()

    async def create(self, station: Station) -> Station:
        self._session.add(station)
        await self._session.flush()
        await self._session.refresh(station)
        return station

    async def update(self, station: Station) -> Station:
        await self._session.flush()
        await self._session.refresh(station)
        return station

    async def delete(self, station_id: int) -> None:
        await self._session.execute(delete(Station).where(Station.id == station_id))
