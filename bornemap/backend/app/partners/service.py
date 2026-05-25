from fastapi import HTTPException
from sqlalchemy.ext.asyncio import AsyncSession

from app.partners.models import Partner
from app.partners.repository import PartnerRepository
from app.partners.schemas import PartnerCreate, PartnerUpdate


class PartnerService:
    def __init__(self, session: AsyncSession):
        self._repo = PartnerRepository(session)

    async def list_partners(self) -> list[Partner]:
        return await self._repo.list_all()

    async def get_partner(self, partner_id: int) -> Partner:
        partner = await self._repo.get_by_id(partner_id)
        if not partner:
            raise HTTPException(status_code=404, detail="Partner not found")
        return partner

    async def create_partner(self, data: PartnerCreate) -> Partner:
        partner = Partner(name=data.name, contact_email=data.contact_email, phone=data.phone, is_active=data.is_active)
        return await self._repo.create(partner)

    async def update_partner(self, partner_id: int, data: PartnerUpdate) -> Partner:
        partner = await self.get_partner(partner_id)
        if data.name is not None:
            partner.name = data.name
        if data.contact_email is not None:
            partner.contact_email = data.contact_email
        if data.phone is not None:
            partner.phone = data.phone
        if data.is_active is not None:
            partner.is_active = data.is_active
        return await self._repo.update(partner)

    async def toggle_active(self, partner_id: int) -> Partner:
        partner = await self.get_partner(partner_id)
        partner.is_active = not partner.is_active
        return await self._repo.update(partner)

    async def delete_partner(self, partner_id: int) -> None:
        await self.get_partner(partner_id)
        await self._repo.delete(partner_id)
