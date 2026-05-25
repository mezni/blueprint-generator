from fastapi import HTTPException
from sqlalchemy.ext.asyncio import AsyncSession

from app.stations.models import Station
from app.stations.repository import StationRepository
from app.stations.schemas import StationCreate, StationUpdate


class StationService:
    def __init__(self, session: AsyncSession):
        self._repo = StationRepository(session)

    async def list_stations(self) -> list[Station]:
        return await self._repo.list_all()

    async def get_station(self, station_id: int) -> Station:
        station = await self._repo.get_by_id(station_id)
        if not station:
            raise HTTPException(status_code=404, detail="Station not found")
        return station

    async def create_station(self, data: StationCreate) -> Station:
        station = Station(
            partner_id=data.partner_id,
            name=data.name,
            operator=data.operator,
            address=data.address,
            plug_types=data.plug_types,
            speed_kw=data.speed_kw,
            is_active=data.is_active,
        )
        return await self._repo.create(station)

    async def update_station(self, station_id: int, data: StationUpdate) -> Station:
        station = await self.get_station(station_id)
        if data.partner_id is not None:
            station.partner_id = data.partner_id
        if data.name is not None:
            station.name = data.name
        if data.operator is not None:
            station.operator = data.operator
        if data.address is not None:
            station.address = data.address
        if data.plug_types is not None:
            station.plug_types = data.plug_types
        if data.speed_kw is not None:
            station.speed_kw = data.speed_kw
        if data.is_active is not None:
            station.is_active = data.is_active
        return await self._repo.update(station)

    async def toggle_active(self, station_id: int) -> Station:
        station = await self.get_station(station_id)
        station.is_active = not station.is_active
        return await self._repo.update(station)

    async def delete_station(self, station_id: int) -> None:
        await self.get_station(station_id)
        await self._repo.delete(station_id)
