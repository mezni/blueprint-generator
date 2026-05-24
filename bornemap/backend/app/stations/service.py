from sqlalchemy.ext.asyncio import AsyncSession

from app.stations.repository import StationRepository


class StationService:
    def __init__(self, session: AsyncSession) -> None:
        self.repository = StationRepository(session)
