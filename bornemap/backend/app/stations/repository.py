from collections.abc import Sequence
from sqlalchemy.ext.asyncio import AsyncSession


class StationRepository:
    def __init__(self, session: AsyncSession) -> None:
        self.session = session
