from sqlalchemy import select, delete
from sqlalchemy.ext.asyncio import AsyncSession
from app.partners.models import Partner


class PartnerRepository:
    def __init__(self, session: AsyncSession):
        self._session = session

    async def list_all(self) -> list[Partner]:
        result = await self._session.execute(select(Partner).order_by(Partner.id))
        return list(result.scalars().all())

    async def get_by_id(self, partner_id: int) -> Partner | None:
        result = await self._session.execute(select(Partner).where(Partner.id == partner_id))
        return result.scalar_one_or_none()

    async def create(self, partner: Partner) -> Partner:
        self._session.add(partner)
        await self._session.flush()
        await self._session.commit()
        await self._session.refresh(partner)
        return partner

    async def update(self, partner: Partner) -> Partner:
        await self._session.flush()
        await self._session.commit()
        await self._session.refresh(partner)
        return partner

    async def delete(self, partner_id: int) -> None:
        await self._session.execute(delete(Partner).where(Partner.id == partner_id))
        await self._session.commit()
